//! Native HTTP source loading with optional disk caching and typed retry classification.

use std::path::PathBuf;

use async_trait::async_trait;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::{Client, StatusCode};
use reqwest_middleware::ClientWithMiddleware;

use crate::io::source_client::{ByteRange, HttpClient, SourceFetchError};

const USER_AGENT: &str = concat!(
    "maplibre-rs/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/maplibre/maplibre-rs)"
);

/// Cloneable HTTP transport that fetches complete bodies and preserves HTTP or network causes.
/// A configured cache directory enables the HTTP cache middleware; clones share the client.
#[derive(Clone)]
pub struct ReqwestHttpClient {
    client: ClientWithMiddleware,
}

impl From<reqwest::Error> for SourceFetchError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() || err.is_connect() || err.is_request() || err.is_body() {
            SourceFetchError::temporary(err)
        } else {
            SourceFetchError(Box::new(err))
        }
    }
}

impl From<reqwest_middleware::Error> for SourceFetchError {
    fn from(err: reqwest_middleware::Error) -> Self {
        match err {
            reqwest_middleware::Error::Reqwest(error) => error.into(),
            other => SourceFetchError(Box::new(other)),
        }
    }
}

impl ReqwestHttpClient {
    /// Creates a client, optionally storing cacheable responses under `cache_path`.
    /// `None` disables the disk cache. If client configuration fails, construction logs
    /// a warning and falls back to Reqwest's default client.
    pub fn new<P>(cache_path: Option<P>) -> Self
    where
        P: Into<PathBuf>,
    {
        // Public tile servers such as OpenStreetMap reject requests without a User-Agent.
        let client = match Client::builder().user_agent(USER_AGENT).build() {
            Ok(client) => client,
            Err(error) => {
                tracing::warn!(%error, "cannot build the HTTP client with a user agent");
                Client::new()
            }
        };
        let mut builder = reqwest_middleware::ClientBuilder::new(client);

        if let Some(cache_path) = cache_path {
            builder = builder.with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager {
                    path: cache_path.into(),
                },
                options: HttpCacheOptions::default(),
            }))
        }
        let client = builder.build();

        Self { client }
    }
}

#[cfg_attr(not(feature = "thread-safe-futures"), async_trait(?Send))]
#[cfg_attr(feature = "thread-safe-futures", async_trait)]
impl HttpClient for ReqwestHttpClient {
    async fn fetch(&self, url: &str) -> Result<Vec<u8>, SourceFetchError> {
        self.get(url, None).await
    }

    async fn fetch_range(&self, url: &str, range: ByteRange) -> Result<Vec<u8>, SourceFetchError> {
        self.get(url, Some(range)).await
    }
}

/// An in-process HTTP handler for local test servers when network sockets are unavailable.
pub type InProcessHandler = std::sync::Arc<
    dyn Fn(
            &str,
            Option<ByteRange>,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<Vec<u8>, SourceFetchError>> + Send>,
        > + Send
        + Sync,
>;

static IN_PROCESS_HANDLERS: std::sync::RwLock<Vec<(String, InProcessHandler)>> =
    std::sync::RwLock::new(Vec::new());

/// Registers an in-process handler for URLs matching `prefix`.
pub fn register_in_process_handler(prefix: String, handler: InProcessHandler) {
    if let Ok(mut handlers) = IN_PROCESS_HANDLERS.write() {
        handlers.retain(|(p, _)| p != &prefix);
        handlers.push((prefix, handler));
    }
}

/// Unregisters an in-process handler by `prefix`.
pub fn unregister_in_process_handler(prefix: &str) {
    if let Ok(mut handlers) = IN_PROCESS_HANDLERS.write() {
        handlers.retain(|(p, _)| p != prefix);
    }
}

impl ReqwestHttpClient {
    /// A server that ignores the range answers with the whole body, cut down here.
    async fn get(&self, url: &str, range: Option<ByteRange>) -> Result<Vec<u8>, SourceFetchError> {
        let mut request = self.client.get(url);
        if let Some(range) = range {
            request = request.header(reqwest::header::RANGE, range.header());
        }
        let response = match request.send().await {
            Ok(response) => response,
            Err(err) => {
                let handler = {
                    let handlers = IN_PROCESS_HANDLERS.read().ok();
                    handlers.and_then(|h| {
                        h.iter()
                            .find(|(prefix, _)| url.starts_with(prefix))
                            .map(|(_, handler)| handler.clone())
                    })
                };
                if let Some(handler) = handler {
                    return handler(url, range).await;
                }
                return Err(err.into());
            }
        };
        if response.status() == StatusCode::NOT_FOUND {
            return Err(SourceFetchError::not_found(url));
        }
        let status = response.status().as_u16();
        match response.error_for_status() {
            Ok(response) => {
                if response.status() == StatusCode::NOT_MODIFIED {
                    log::info!("Using data from cache");
                }

                let partial = response.status() == StatusCode::PARTIAL_CONTENT;
                let body = response.bytes().await?;
                match range {
                    Some(range) if !partial => range.slice(url, &body).map(<[u8]>::to_vec),
                    _ => Ok(Vec::from(body.as_ref())),
                }
            }
            Err(error) => Err(SourceFetchError::http_response(url, status, error)),
        }
    }
}
