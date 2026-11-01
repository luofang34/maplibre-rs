#![allow(clippy::expect_used, clippy::panic)]

use std::time::Duration;

use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};

use super::ExternalFrame;
use crate::{
    coords::LatLon,
    headless::{create_headless_renderer, map::HeadlessMap},
    render::{
        camera::EyeFrustum,
        view_state::{ExternalAnchor, ExternalView},
        RenderPlugin,
    },
    style::Style,
    terrain::resources::TerrainResources,
};

const SIZE: u32 = 64;

/// The view from `world_from_eye` of a scene whose origin stands at `position`, `altitude_meters`
/// above the sea, and whose axes are the world's.
pub(crate) fn eye_view(
    position: LatLon,
    altitude_meters: f64,
    world_from_eye: Matrix4<f64>,
    frustum: EyeFrustum,
) -> ExternalView {
    ExternalView {
        anchor: ExternalAnchor {
            position,
            altitude_meters,
        },
        view: world_from_eye.invert().expect("an invertible pose"),
        frustum,
    }
}

#[tokio::test]
async fn a_headless_surface_multisamples_like_a_window() {
    use crate::render::{eventually::Eventually, settings::Msaa};

    let style: Style = serde_json::from_str(r#"{"version": 8, "sources": {}, "layers": []}"#)
        .expect("an empty style parses");
    let (kernel, renderer) = create_headless_renderer(SIZE, SIZE, None)
        .await
        .expect("a headless renderer");
    assert!(
        renderer
            .state()
            .surface()
            .is_multisampling_supported(Msaa { samples: 4 }),
        "the adapter multisamples the surface format"
    );
    let mut map =
        HeadlessMap::new(style, renderer, kernel, vec![Box::new(RenderPlugin)]).expect("a map");
    map.run_frame().expect("a frame renders");
    assert!(
        matches!(
            map.map_context.renderer.resources.multisampling_texture,
            Eventually::Initialized(Some(_))
        ),
        "the frame resolves from a multisampled texture"
    );
}

/// The plugins a globe host builds its map with, terrain included.
fn device_plugins() -> Vec<Box<dyn crate::plugin::Plugin<crate::headless::HeadlessEnvironment>>> {
    vec![
        Box::new(RenderPlugin),
        Box::new(crate::background::BackgroundPlugin),
        Box::new(crate::vector::VectorPlugin::<
            crate::vector::DefaultVectorTransferables,
        >::default()),
        Box::new(crate::sdf::SdfPlugin::<
            crate::vector::DefaultVectorTransferables,
        >::default()),
        Box::new(crate::raster::RasterPlugin::<
            crate::raster::DefaultRasterTransferables,
        >::default()),
        Box::new(crate::hillshade::HillshadePlugin),
        Box::new(crate::terrain::TerrainPlugin::<
            crate::terrain::DefaultDemTransferables,
        >::default()),
    ]
}

/// A globe style with terrain and a vector source.
fn terrain_globe_style() -> Style {
    serde_json::from_str(
        r##"{"version":8,"sources":{"osm":{"type":"vector","tiles":["https://osm.example/{z}/{x}/{y}.pbf"],"maxzoom":14},"dem":{"type":"raster-dem","tiles":["https://dem.example/{z}/{x}/{y}.png"],"tileSize":256,"maxzoom":12,"encoding":"terrarium"}},"layers":[{"id":"land","type":"fill","source":"osm","source-layer":"land","paint":{"fill-color":"#ccc"}}],"terrain":{"source":"dem","exaggeration":1},"projection":{"type":"globe"}}"##,
    )
    .expect("a globe terrain style parses")
}

#[tokio::test]
async fn a_globe_on_a_table_requests_only_the_levels_it_shows() {
    use cgmath::Matrix3;

    let (width, height) = (1888, 1792);
    let (kernel, renderer) = create_headless_renderer(width, height, None)
        .await
        .expect("a headless renderer");
    let mut map =
        HeadlessMap::new(terrain_globe_style(), renderer, kernel, device_plugins()).expect("a map");
    map.set_max_pitch(cgmath::Deg(89.0));
    // A globe fifteen centimetres in radius a metre ahead of the eyes, as on the table.
    let radius_meters = 6_371_008.8;
    let radius = 0.15;
    let scale = radius / radius_meters;
    let latitude = 47.26_f64.to_radians();
    let east = Vector3::new(1.0, 0.0, 0.0);
    let north = Vector3::new(0.0, latitude.cos(), -latitude.sin());
    let up = Vector3::new(0.0, latitude.sin(), latitude.cos());
    let rotation = Matrix3::from_cols(east, north, up);
    let center = Vector3::new(0.0, -0.3, -1.0);
    let world_from_scene = Matrix4::from_translation(center + up * radius)
        * Matrix4::from(rotation)
        * Matrix4::from_scale(scale);
    for frame_number in 0..2_u64 {
        map.run_external_frame(ExternalFrame {
            opaque_environment: false,
            timestamp: Duration::from_millis(16 * (frame_number + 1)),
            view: ExternalView {
                anchor: ExternalAnchor {
                    position: LatLon::new(47.26, 11.39),
                    altitude_meters: 0.0,
                },
                view: world_from_scene,
                frustum: EyeFrustum::symmetric(Rad(1.4), 1.05, 0.05, 100.0),
            },
            depth: None,
            request_overscan: 1.2,
        })
        .expect("the frame renders");
    }
    let view = map.view_state();
    let level = u8::from(view.zoom().zoom_level(crate::coords::TILE_SIZE));
    let mut levels: Vec<u8> = map
        .world()
        .tiles
        .tiles
        .values()
        .map(|tile| u8::from(tile.coords.z))
        .collect();
    levels.sort_unstable();
    levels.dedup();
    assert!(
        levels.iter().all(|z| *z <= level + 2),
        "a table globe at zoom {:.2} (level {level}) requested tiles at levels {levels:?}",
        view.zoom().value()
    );
}

#[tokio::test]
async fn terrain_draws_reuse_their_bind_groups_from_frame_to_frame() {
    use cgmath::Matrix3;

    use crate::render::eventually::{Eventually, Eventually::Initialized};

    let (width, height) = (1888, 1792);
    let (kernel, renderer) = create_headless_renderer(width, height, None)
        .await
        .expect("a headless renderer");
    let mut map =
        HeadlessMap::new(terrain_globe_style(), renderer, kernel, device_plugins()).expect("a map");
    let radius_meters = 6_371_008.8;
    let latitude = 47.26_f64.to_radians();
    let rotation = Matrix3::from_cols(
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, latitude.cos(), -latitude.sin()),
        Vector3::new(0.0, latitude.sin(), latitude.cos()),
    );
    let table = Matrix4::from_translation(Vector3::new(0.0, -0.15, -1.0))
        * Matrix4::from(rotation)
        * Matrix4::from_scale(0.15 / radius_meters);
    let mut groups_per_frame = Vec::new();
    for frame_number in 0..3_u64 {
        map.run_external_frame(ExternalFrame {
            opaque_environment: false,
            timestamp: Duration::from_millis(16 * (frame_number + 1)),
            view: ExternalView {
                anchor: ExternalAnchor {
                    position: LatLon::new(47.26, 11.39),
                    altitude_meters: 0.0,
                },
                view: table,
                frustum: EyeFrustum::symmetric(Rad(1.4), 1.05, 0.05, 100.0),
            },
            depth: None,
            request_overscan: 1.2,
        })
        .expect("the frame renders");
        let Some(Initialized(terrain)) =
            map.world().resources.get::<Eventually<TerrainResources>>()
        else {
            panic!("terrain resources exist");
        };
        let mut groups: Vec<_> = terrain
            .draws()
            .iter()
            .map(|draw| (draw.coords, draw.bind_group.as_ref().clone()))
            .collect();
        groups.sort_by_key(|(coords, _)| (u8::from(coords.z), coords.x, coords.y));
        groups_per_frame.push(groups);
    }
    assert!(
        !groups_per_frame[2].is_empty(),
        "the table globe draws terrain tiles"
    );
    assert_eq!(
        groups_per_frame[1], groups_per_frame[2],
        "a frame at rest recreated its terrain bind groups"
    );
}
