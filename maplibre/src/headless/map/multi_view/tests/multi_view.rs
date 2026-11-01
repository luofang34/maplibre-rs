use std::time::Duration;

use super::{helpers::*, *};
use crate::{
    background::BackgroundPlugin,
    headless::create_headless_renderer_with_loader,
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
async fn heterogeneous_multi_view_renders_otw_and_sensor_without_skipped_slot_eviction() {
    let client = CountingTileClient::default();
    let loader = SharedLoader::new_with_client(client.clone());
    let mut renderer = create_headless_renderer_with_loader(
        RendererSettings {
            msaa: Msaa::Off,
            memory_budget: MemoryBudget::capped_megabytes(64),
            buffer_pools: BufferPoolSizes::default(),
            symbol_pools: BufferPoolSizes::default(),
            ..Default::default()
        },
        loader,
    )
    .await;

    let style = terrain_raster_style("#102030");
    let plugins = vec![
        Box::new(RenderPlugin) as Box<dyn crate::plugin::Plugin<_>>,
        Box::new(BackgroundPlugin),
        Box::new(RasterPlugin::<DefaultRasterTransferables>::default()),
        Box::new(TerrainPlugin::<DefaultDemTransferables>::default()),
        Box::new(VectorPlugin::<DefaultVectorTransferables>::default()),
        Box::new(OverlayPlugin),
    ];
    let mut map = HeadlessMap::new_with_plugins(
        style,
        renderer.take_kernel_builder(),
        plugins,
        PhysicalSize::new(160, 90),
    )
    .expect("create HeadlessMap");

    // Register 3 slots:
    // Slot 1: OTW (160x90, color + depth readout, independent covering)
    // Slot 2: OTW_StereoRight (160x90, color only, StereoReuse of Slot 1)
    // Slot 3: Sensor (80x60, color + depth readout, independent covering)
    let otw_slot = ViewSlotId::new(1);
    let otw_right = ViewSlotId::new(2);
    let sensor_slot = ViewSlotId::new(3);

    map.register_view_slot(
        otw_slot,
        ViewSlotConfig {
            size: PhysicalSize::new(160, 90),
            depth_readout: true,
        },
    )
    .expect("register otw");
    map.register_view_slot(
        otw_right,
        ViewSlotConfig {
            size: PhysicalSize::new(160, 90),
            depth_readout: false,
        },
    )
    .expect("register otw right");
    map.register_view_slot(
        sensor_slot,
        ViewSlotConfig {
            size: PhysicalSize::new(80, 60),
            depth_readout: true,
        },
    )
    .expect("register sensor");

    let otw_view = external_view_at(47.0, 8.0, 1500.0, 10.0, 160.0 / 90.0);
    let sensor_view = external_view_at(47.05, 8.05, 3000.0, 60.0, 80.0 / 60.0);

    // Warm up the pipeline over 8 frames.
    for _ in 0..8 {
        map.run_multi_view_frame(MultiViewFrame {
            slots: vec![
                ViewSlotEntry {
                    slot: otw_slot,
                    view: otw_view,
                    policy: CoveringPolicy::Independent,
                },
                ViewSlotEntry {
                    slot: otw_right,
                    view: otw_view,
                    policy: CoveringPolicy::StereoReuse(otw_slot),
                },
                ViewSlotEntry {
                    slot: sensor_slot,
                    view: sensor_view,
                    policy: CoveringPolicy::Independent,
                },
            ],
        })
        .expect("multi-view frame");
        tokio::time::sleep(Duration::from_millis(5)).await;
    }

    let otw_color = map.view_slot_color_texture(otw_slot).expect("otw color");
    let otw_depth = map.view_slot_depth_texture(otw_slot).expect("otw depth");
    let sensor_color = map
        .view_slot_color_texture(sensor_slot)
        .expect("sensor color");
    let sensor_depth = map
        .view_slot_depth_texture(sensor_slot)
        .expect("sensor depth");
    assert_eq!((otw_color.width(), otw_color.height()), (160, 90));
    assert_eq!((otw_depth.width(), otw_depth.height()), (160, 90));
    assert_eq!((sensor_color.width(), sensor_color.height()), (80, 60));
    assert_eq!((sensor_depth.width(), sensor_depth.height()), (80, 60));
    assert!(map.view_slot_depth_texture(otw_right).is_none());
    assert_eq!(map.view_slot_has_readback_buffer(otw_slot), Some(false));
    assert_eq!(map.view_slot_has_readback_buffer(sensor_slot), Some(false));

    // Per-slot OverlayMesh isolation: green on OTW, red on Sensor.
    map.set_view_slot_overlay(otw_slot, fullscreen_overlay(160, 90, [0.0, 1.0, 0.0, 1.0]))
        .expect("set otw overlay");
    map.set_view_slot_overlay(
        sensor_slot,
        fullscreen_overlay(80, 60, [1.0, 0.0, 0.0, 1.0]),
    )
    .expect("set sensor overlay");
    map.run_multi_view_frame(MultiViewFrame {
        slots: vec![
            ViewSlotEntry {
                slot: otw_slot,
                view: otw_view,
                policy: CoveringPolicy::Independent,
            },
            ViewSlotEntry {
                slot: sensor_slot,
                view: sensor_view,
                policy: CoveringPolicy::Independent,
            },
        ],
        ..Default::default()
    })
    .expect("overlay frame");

    let otw_rgba = crate::headless::map::readback::read_blocking(
        &map,
        &map.view_slot_color_texture(otw_slot).unwrap(),
    )
    .expect("read otw");
    assert_eq!(map.view_slot_has_readback_buffer(otw_slot), Some(true));
    assert_eq!(map.view_slot_has_readback_buffer(sensor_slot), Some(false));

    let otw_pixel = &otw_rgba[0..4];
    assert!(
        otw_pixel[1] > 200 && otw_pixel[0] < 50,
        "otw should be green: {:?}",
        otw_pixel
    );

    let sensor_rgba = crate::headless::map::readback::read_blocking(
        &map,
        &map.view_slot_color_texture(sensor_slot).unwrap(),
    )
    .expect("read sensor");
    assert_eq!(map.view_slot_has_readback_buffer(sensor_slot), Some(true));
    let sensor_pixel = &sensor_rgba[0..4];
    assert!(
        sensor_pixel[0] > 200 && sensor_pixel[1] < 50,
        "sensor should be red: {:?}",
        sensor_pixel
    );

    // Dynamic slot addition / removal.
    let adhoc = ViewSlotId::new(42);
    map.register_view_slot(
        adhoc,
        ViewSlotConfig {
            size: PhysicalSize::new(64, 64),
            depth_readout: false,
        },
    )
    .expect("register adhoc");
    map.run_multi_view_frame(MultiViewFrame {
        slots: vec![ViewSlotEntry {
            slot: adhoc,
            view: otw_view,
            policy: CoveringPolicy::Independent,
        }],
        ..Default::default()
    })
    .expect("run adhoc");
    assert!(map.view_slot_color_texture(adhoc).is_some());

    map.unregister_view_slot(adhoc).expect("unregister adhoc");
    assert!(map.view_slot_color_texture(adhoc).is_none());
    assert!(map.unregister_view_slot(adhoc).is_err());
}
