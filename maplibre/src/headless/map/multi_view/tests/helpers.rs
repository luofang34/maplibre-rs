use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use cgmath::{Deg, Matrix3, Matrix4, Rad};
use image::ImageEncoder;

use crate::{
    coords::{LatLon, WorldTileCoords},
    headless::map::HeadlessMap,
    io::source_client::{HttpClient, SourceFetchError},
    overlay::{OverlayMesh, OverlayVertex},
    render::{
        camera::EyeFrustum,
        host_gpu::HostGpu,
        view_state::{ExternalAnchor, ExternalView},
    },
    style::Style,
    terrain::DemTileComponent,
};

#[derive(Clone, Default)]
pub struct CountingTileClient {
    pub fetches: Arc<Mutex<HashMap<String, usize>>>,
}

impl CountingTileClient {
    pub fn total_fetches(&self) -> usize {
        self.fetches.lock().expect("lock").values().sum()
    }

    pub fn max_fetches_for_any_url(&self) -> usize {
        self.fetches
            .lock()
            .expect("lock")
            .values()
            .copied()
            .max()
            .unwrap_or(0)
    }
}

#[cfg_attr(not(feature = "thread-safe-futures"), async_trait::async_trait(?Send))]
#[cfg_attr(feature = "thread-safe-futures", async_trait::async_trait)]
impl HttpClient for CountingTileClient {
    async fn fetch(&self, url: &str) -> Result<Vec<u8>, SourceFetchError> {
        *self
            .fetches
            .lock()
            .expect("lock")
            .entry(url.to_owned())
            .or_insert(0) += 1;

        let rgba = if url.contains("dem.invalid") {
            // Terrarium encoding: (R * 256 + G + B / 256) - 32768 -> ~492 m MSL
            [129, 236, 0, 255]
        } else {
            [40, 120, 200, 255]
        };
        let mut pixels = Vec::with_capacity(16 * 16 * 4);
        for _ in 0..(16 * 16) {
            pixels.extend_from_slice(&rgba);
        }
        let mut bytes = Vec::new();
        image::codecs::png::PngEncoder::new(&mut bytes)
            .write_image(&pixels, 16, 16, image::ExtendedColorType::Rgba8)
            .expect("encode PNG");
        Ok(bytes)
    }
}

pub async fn host_gpu() -> Option<HostGpu> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .ok()?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .ok()?;
    Some(HostGpu {
        instance,
        adapter,
        device,
        queue,
    })
}

pub fn external_view_at(
    lat: f64,
    lon: f64,
    alt_m: f64,
    pitch_deg: f64,
    aspect: f64,
) -> ExternalView {
    let base_from_local = Matrix3::new(1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0);
    let pitch = Matrix3::from_angle_x(Deg(-pitch_deg));
    ExternalView {
        anchor: ExternalAnchor {
            position: LatLon::new(lat, lon),
            altitude_meters: alt_m,
        },
        view: Matrix4::from(pitch * base_from_local),
        frustum: EyeFrustum::symmetric(Rad(0.8), aspect, 1.0, 250_000.0),
    }
}

pub fn terrain_raster_style(bg_hex: &str) -> Style {
    serde_json::from_value(serde_json::json!({
        "version": 8,
        "projection": { "type": "vertical-perspective" },
        "center": [8.0, 47.0],
        "zoom": 6.0,
        "sources": {
            "dem": {
                "type": "raster-dem",
                "tiles": ["https://dem.invalid/{z}/{x}/{y}.png"],
                "tileSize": 256,
                "maxzoom": 8,
                "encoding": "terrarium"
            },
            "imagery": {
                "type": "raster",
                "tiles": ["https://imagery.invalid/{z}/{x}/{y}.png"],
                "tileSize": 256,
                "maxzoom": 8
            },
            "tracks": {
                "type": "geojson",
                "data": { "type": "FeatureCollection", "features": [] }
            }
        },
        "terrain": { "source": "dem", "exaggeration": 1.0 },
        "layers": [
            { "id": "bg", "type": "background", "paint": { "background-color": bg_hex } },
            { "id": "imagery", "type": "raster", "source": "imagery" },
            {
                "id": "track-line",
                "type": "line",
                "source": "tracks",
                "paint": { "line-color": "#ffaa00", "line-width": 3.0 }
            }
        ]
    }))
    .expect("valid terrain style")
}

pub fn fullscreen_overlay(width: u32, height: u32, color: [f32; 4]) -> OverlayMesh {
    let w = width as f32;
    let h = height as f32;
    OverlayMesh {
        vertices: vec![
            OverlayVertex {
                position: [0.0, 0.0],
                depth: 1.0,
                color,
            },
            OverlayVertex {
                position: [w, 0.0],
                depth: 1.0,
                color,
            },
            OverlayVertex {
                position: [w, h],
                depth: 1.0,
                color,
            },
            OverlayVertex {
                position: [0.0, h],
                depth: 1.0,
                color,
            },
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
    }
}

pub fn loaded_dem_coords(map: &HeadlessMap) -> HashSet<WorldTileCoords> {
    let tiles = &map.world().tiles;
    tiles
        .tiles
        .values()
        .map(|tile| tile.coords)
        .filter(|&coords| {
            matches!(
                tiles.query::<&DemTileComponent>(coords),
                Some(DemTileComponent::Loaded(_))
            )
        })
        .collect()
}
