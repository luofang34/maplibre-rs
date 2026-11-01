//! Cross-domain static asset and GPU pipeline sharing across [`HeadlessMap`](super::map::HeadlessMap) instances.
//!
//! Host applications that maintain isolated scene worlds on the same [`HostGpu`](crate::render::host_gpu::HostGpu)
//! (such as `TruthScene` and `BeliefAvionics`) construct one [`SharedMapAssets`] handle and pass
//! clones of it when building each map's headless renderer. All instances then share:
//! - In-flight HTTP/PMTiles coalescing via an optional [`SharedLoader`],
//! - Decoded immutable [`DemTile`](crate::terrain::DemTile) grids, static [`image::RgbaImage`]
//!   basemap tiles, glyph ranges, and sprite sheets via [`AssetCache`],
//! - Compiled [`wgpu::ShaderModule`], canonical [`wgpu::BindGroupLayout`], and
//!   [`wgpu::RenderPipeline`] objects via [`SharedPipelines`].
//!
//! Each map's `Style`, `World`, and `GeoJsonSource`s remain strictly isolated.

use crate::{
    io::resource_loader::SharedLoader, render::resource::SharedPipelines, sdf::assets::AssetCache,
};

/// Default decoded static-asset byte budget shared across maps (`128 MiB`).
const DEFAULT_SHARED_ASSET_BUDGET_BYTES: usize = 128 * 1024 * 1024;

/// Cloneable handle sharing immutable decoded tile/glyph/sprite assets and compiled GPU pipelines
/// across independent [`HeadlessMap`](super::map::HeadlessMap) instances.
#[derive(Clone, Debug)]
pub struct SharedMapAssets {
    loader: Option<SharedLoader>,
    asset_cache: AssetCache,
    pipelines: SharedPipelines,
}

impl Default for SharedMapAssets {
    fn default() -> Self {
        Self::new()
    }
}

impl SharedMapAssets {
    /// Creates a shared asset and pipeline cache with a `128 MiB` decoded-asset budget.
    pub fn new() -> Self {
        Self {
            loader: None,
            asset_cache: AssetCache::with_budget(DEFAULT_SHARED_ASSET_BUDGET_BYTES),
            pipelines: SharedPipelines::new(),
        }
    }

    /// Creates a shared asset and pipeline cache backed by `loader` for single-flight source fetches.
    pub fn with_loader(loader: SharedLoader) -> Self {
        Self {
            loader: Some(loader),
            ..Self::new()
        }
    }

    /// Overrides the decoded-asset cache (`AssetCache`) budget or instance.
    pub fn with_asset_cache(mut self, asset_cache: AssetCache) -> Self {
        self.asset_cache = asset_cache;
        self
    }

    /// Overrides the [`SharedPipelines`] cache instance.
    pub fn with_pipelines(mut self, pipelines: SharedPipelines) -> Self {
        self.pipelines = pipelines;
        self
    }

    /// Shared resource loader, if one was configured.
    pub fn loader(&self) -> Option<&SharedLoader> {
        self.loader.as_ref()
    }

    /// Shared cache of decoded DEM grids, static raster images, glyph ranges, and sprites.
    pub fn asset_cache(&self) -> &AssetCache {
        &self.asset_cache
    }

    /// Shared cache of compiled WGSL shader modules and render pipelines.
    pub fn pipelines(&self) -> &SharedPipelines {
        &self.pipelines
    }
}
