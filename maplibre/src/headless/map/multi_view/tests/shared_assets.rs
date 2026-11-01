use std::time::Duration;

use super::{helpers::*, *};
use crate::{
    background::BackgroundPlugin,
    headless::{create_headless_renderer_on_host_gpu_with_shared_assets, SharedMapAssets},
    io::resource_loader::SharedLoader,
    overlay::OverlayPlugin,
    raster::{DefaultRasterTransferables, RasterPlugin},
    render::{
        memory_budget::MemoryBudget,
        settings::{BufferPoolSizes, Msaa, RendererSettings},
        RenderPlugin,
    },
    terrain::{transferables::DefaultDemTransferables, TerrainPlugin},
    vector::{DefaultVectorTransferables, VectorPlugin},
    window::PhysicalSize,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn shared_map_assets_deduplicates_static_dem_and_pipelines_across_isolated_worlds() {
    let Some(gpu) = host_gpu().await else {
        eprintln!("skipping test: no host GPU available");
        return;
    };

    let client = CountingTileClient::default();
    let shared_assets = SharedMapAssets::new();

    let settings = RendererSettings {
        msaa: Msaa::Off,
        memory_budget: MemoryBudget::capped_megabytes(64),
        buffer_pools: BufferPoolSizes::default(),
        symbol_pools: BufferPoolSizes::default(),
        ..Default::default()
    };

    let mut renderer_a = create_headless_renderer_on_host_gpu_with_shared_assets(
        gpu.clone(),
        settings.clone(),
        SharedLoader::new_with_client(client.clone()),
        shared_assets.clone(),
    );
    let mut renderer_b = create_headless_renderer_on_host_gpu_with_shared_assets(
        gpu.clone(),
        settings,
        SharedLoader::new_with_client(client.clone()),
        shared_assets.clone(),
    );

    let style_a = terrain_raster_style("#112233");
    let style_b = terrain_raster_style("#332211");

    let plugins_a = vec![
        Box::new(RenderPlugin) as Box<dyn crate::plugin::Plugin<_>>,
        Box::new(BackgroundPlugin),
        Box::new(RasterPlugin::<DefaultRasterTransferables>::default()),
        Box::new(TerrainPlugin::<DefaultDemTransferables>::default()),
        Box::new(VectorPlugin::<DefaultVectorTransferables>::default()),
        Box::new(OverlayPlugin),
    ];
    let plugins_b = vec![
        Box::new(RenderPlugin) as Box<dyn crate::plugin::Plugin<_>>,
        Box::new(BackgroundPlugin),
        Box::new(RasterPlugin::<DefaultRasterTransferables>::default()),
        Box::new(TerrainPlugin::<DefaultDemTransferables>::default()),
        Box::new(VectorPlugin::<DefaultVectorTransferables>::default()),
        Box::new(OverlayPlugin),
    ];

    let mut map_a = HeadlessMap::new_with_plugins(
        style_a,
        renderer_a.take_kernel_builder(),
        plugins_a,
        PhysicalSize::new(160, 90),
    )
    .expect("create map A");
    let mut map_b = HeadlessMap::new_with_plugins(
        style_b,
        renderer_b.take_kernel_builder(),
        plugins_b,
        PhysicalSize::new(160, 90),
    )
    .expect("create map B");

    let view = external_view_at(47.0, 8.0, 2000.0, 15.0, 160.0 / 90.0);
    map_a.set_external_view(view.clone()).expect("set view A");
    map_b.set_external_view(view).expect("set view B");

    for _ in 0..10 {
        map_a.run_frame().expect("frame A");
        map_b.run_frame().expect("frame B");
        tokio::time::sleep(Duration::from_millis(5)).await;
    }

    let loaded_a = loaded_dem_coords(&map_a);
    let loaded_b = loaded_dem_coords(&map_b);
    assert!(!loaded_a.is_empty(), "Map A should have loaded DEM tiles");
    assert!(!loaded_b.is_empty(), "Map B should have loaded DEM tiles");

    let max_fetches = client.max_fetches_for_any_url();
    assert_eq!(
        max_fetches, 1,
        "Static DEM tiles should be deduplicated: max fetches was {max_fetches}"
    );

    let pipe_count = shared_assets.pipeline_cache().len();
    assert!(
        pipe_count > 0,
        "SharedMapAssets should hold compiled pipelines"
    );
}
