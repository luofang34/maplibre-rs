#![allow(clippy::expect_used, clippy::panic)]
use std::time::Duration;

use cgmath::{Matrix4, Rad, Vector3};

use crate::{
    coords::{LatLon, WorldTileCoords},
    headless::{
        create_headless_renderer,
        map::{
            external_frame::{tests::eye_view, ExternalFrame},
            HeadlessMap,
        },
    },
    render::{camera::EyeFrustum, RenderPlugin},
    style::Style,
};

/// A globe with a 200 m building just south of the equator.
fn building_style() -> Style {
    let mut style: Style = serde_json::from_value(serde_json::json!({
        "version": 8, "projection": {"type":"globe"}, "terrain":{"source":"dem"},
        "center": [0.0, 0.0], "zoom": 0,
        "sources": {
            "buildings": {"type":"vector", "tiles":["https://buildings.example/{z}/{x}/{y}.pbf"]},
            "dem": {"type":"raster-dem", "tiles":["https://dem.example/{z}/{x}/{y}.png"], "encoding":"terrarium"}
        }, "layers": [
            {"id":"background","type":"background","paint":{"background-color":"#990000"}},
            {"id":"building","type":"fill-extrusion","source":"buildings","source-layer":"buildings",
             "paint":{"fill-extrusion-color":"#00ff00","fill-extrusion-height":200}}
        ]
    }))
    .expect("style");
    for (index, layer) in style.layers.iter_mut().enumerate() {
        layer.index = index as u32;
    }
    style
}

async fn building_map() -> HeadlessMap {
    let style = building_style();
    let layers = style.layers[1..].to_vec();
    let root = WorldTileCoords::default();
    let (kernel, renderer) = create_headless_renderer(64, 64, None)
        .await
        .expect("renderer");
    let mut map = HeadlessMap::new(
        style,
        renderer,
        kernel,
        vec![
            Box::new(RenderPlugin),
            Box::new(crate::background::BackgroundPlugin),
            Box::new(crate::vector::VectorPlugin::<
                crate::vector::DefaultVectorTransferables,
            >::default()),
            Box::new(crate::terrain::TerrainPlugin::<
                crate::terrain::DefaultDemTransferables,
            >::default()),
            Box::new(crate::headless::HeadlessPlugin::new(false).preserve_tile_sources()),
        ],
    )
    .expect("map");
    map.load_dem_tiles(vec![(
        root,
        image::RgbaImage::from_pixel(2, 2, image::Rgba([128, 0, 0, 255])),
    )])
    .expect("DEM");
    let footprint = serde_json::json!({"type":"Feature","properties":{},
        "geometry":{"type":"Polygon","coordinates":[[[0.1,-0.3],[0.3,-0.3],[0.3,-0.1],[0.1,-0.1],[0.1,-0.3]]]}});
    let layers = map
        .process_geojson(
            &footprint,
            "buildings",
            layers,
            root,
            crate::projection::ProjectionType::Globe,
        )
        .expect("building");
    assert!(!layers.vector.is_empty(), "processed geometry");
    map.render_tile(layers).expect("upload building");
    map
}

fn render_from_above(map: &mut HeadlessMap) {
    for timestamp in [0, 16, 32] {
        map.run_external_frame(ExternalFrame {
            opaque_environment: true,
            timestamp: Duration::from_millis(timestamp),
            view: eye_view(
                LatLon::new(0.0, 0.0),
                0.0,
                Matrix4::from_translation(Vector3::new(0.0, 0.0, 4000.0)),
                EyeFrustum::symmetric(Rad(1.0), 1.0, 0.05, 1e9),
            ),
            depth: None,
            request_overscan: 1.0,
        })
        .expect("building frame");
    }
}

/// The height of the base, held in `distance`, of the building the frame uploaded.
fn lifted_base(map: &HeadlessMap) -> f32 {
    let lifted = &map
        .world()
        .resources
        .get::<super::CachedBuffers>()
        .expect("lifted buffers")
        .buffers;
    let (_, id, buffer) = lifted.first().expect("the building is lifted");
    assert_eq!(id, "building");
    buffer
        .buffer
        .vertices
        .iter()
        .map(|vertex| vertex.distance)
        .fold(f32::MAX, f32::min)
}

#[tokio::test]
async fn a_building_stands_on_finer_terrain_when_the_dem_refines() {
    let mut map = building_map().await;
    render_from_above(&mut map);
    assert!(lifted_base(&map).abs() < 1.0, "on the sea level DEM");
    let fine = (0..2)
        .map(|x| {
            (
                WorldTileCoords::from((x, 1, crate::coords::ZoomLevel::new(1))),
                image::RgbaImage::from_pixel(2, 2, image::Rgba([129, 244, 0, 255])),
            )
        })
        .collect();
    map.load_dem_tiles(fine).expect("refined 500 metre terrain");
    render_from_above(&mut map);
    let base = lifted_base(&map);
    assert!(
        (base - 500.0).abs() < 1.0,
        "the building must rise from the finer terrain: {base}"
    );
}
