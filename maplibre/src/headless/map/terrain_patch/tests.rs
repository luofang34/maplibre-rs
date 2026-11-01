#![allow(clippy::expect_used, clippy::panic)]

use std::collections::HashSet;

use cgmath::{Deg, Matrix3, Matrix4, Rad};
use image::{Rgba, RgbaImage};

use super::*;
use crate::{
    background::BackgroundPlugin,
    coords::{LatLon, WorldTileCoords, ZoomLevel},
    headless::{create_headless_renderer_with_settings, HeadlessPlugin},
    render::{
        camera::EyeFrustum,
        eventually::{Eventually, Eventually::Initialized},
        frame_input::ViewSource,
        settings::{BufferPoolSizes, Msaa, RendererSettings, TerrainDrapeMode},
        view_state::{ExternalAnchor, ExternalView},
        RenderPlugin,
    },
    style::Style,
    terrain::{resources::TerrainResources, transferables::DefaultDemTransferables, TerrainPlugin},
};

fn encode_terrarium(elevation_m: f64) -> Rgba<u8> {
    let shifted = (elevation_m + 32_768.0).clamp(0.0, 65_535.99);
    let r = (shifted / 256.0).floor() as u8;
    let rem = shifted - f64::from(r) * 256.0;
    let g = rem.floor() as u8;
    let b = ((rem - f64::from(g)) * 256.0).floor() as u8;
    Rgba([r, g, b, 255])
}

fn synthetic_ridge_dem(size: u32, base_m: f64, peak_m: f64) -> RgbaImage {
    RgbaImage::from_fn(size, size, |x, y| {
        let nx = f64::from(x) / f64::from(size.saturating_sub(1).max(1));
        let ny = f64::from(y) / f64::from(size.saturating_sub(1).max(1));
        let ridge = (1.0 - (nx - 0.5).abs() * 2.0).max(0.0);
        let undulation = (ny * std::f64::consts::TAU).sin() * 180.0;
        let elev = (base_m + ridge * (peak_m - base_m) + undulation).max(-420.0);
        encode_terrarium(elev)
    })
}

fn svs_terrain_style() -> Style {
    serde_json::from_value(serde_json::json!({
        "version": 8,
        "projection": { "type": "vertical-perspective" },
        "center": [8.0, 47.0],
        "zoom": 7.0,
        "sources": {
            "dem": {
                "type": "raster-dem",
                "tiles": ["https://dem.invalid/{z}/{x}/{y}.png"],
                "tileSize": 256,
                "minzoom": 0,
                "maxzoom": 7,
                "encoding": "terrarium"
            }
        },
        "terrain": { "source": "dem", "exaggeration": 1.0 },
        "layers": [
            { "id": "bg", "type": "background", "paint": { "background-color": "#05080c" } }
        ]
    }))
    .expect("valid SVS terrain style")
}

fn svs_external_view(lat: f64, lon: f64, alt_m: f64, pitch_deg: f64, aspect: f64) -> ExternalView {
    let base_from_local = Matrix3::new(1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0);
    let pitch = Matrix3::from_angle_x(Deg(-pitch_deg));
    ExternalView {
        anchor: ExternalAnchor {
            position: LatLon::new(lat, lon),
            altitude_meters: alt_m,
        },
        view: Matrix4::from(pitch * base_from_local),
        frustum: EyeFrustum::symmetric(Rad(0.75), aspect, 1.0, 220_000.0),
    }
}

#[tokio::test]
async fn resident_terrain_patch_exports_stitched_129x129_grid_and_drape_handle() {
    let settings = RendererSettings {
        msaa: Msaa { samples: 1 },
        buffer_pools: BufferPoolSizes::EMPTY,
        symbol_pools: BufferPoolSizes::EMPTY,
        terrain_drape_mode: TerrainDrapeMode::Draped { edge_px: 256 },
        ..RendererSettings::default()
    };
    let (kernel, renderer) = create_headless_renderer_with_settings(256, 256, None, settings)
        .await
        .expect("headless renderer");
    let mut map = HeadlessMap::new(
        svs_terrain_style(),
        renderer,
        kernel,
        vec![
            Box::new(RenderPlugin),
            Box::new(BackgroundPlugin),
            Box::new(TerrainPlugin::<DefaultDemTransferables>::default()),
            Box::new(HeadlessPlugin::new(false).retain_supplied_tiles()),
        ],
    )
    .expect("headless map");

    let mut dem_tiles = Vec::new();
    for z in 0..=6_u8 {
        let side = 1_i32 << z;
        for y in 0..side.min(32) {
            for x in 0..side.min(40) {
                if z <= 3 || ((30..=36).contains(&x) && (20..=25).contains(&y)) {
                    dem_tiles.push((
                        WorldTileCoords {
                            x,
                            y,
                            z: ZoomLevel::new(z),
                        },
                        synthetic_ridge_dem(32, 200.0, 1_800.0),
                    ));
                }
            }
        }
    }
    map.load_dem_tiles(dem_tiles).expect("load DEM");
    map.frame_input_mut().view =
        ViewSource::External(svs_external_view(47.0, 8.0, 4_500.0, -35.0, 1.0));
    map.run_frame().expect("terrain frame");

    let drawn_coords: Vec<WorldTileCoords> = {
        let Some(Initialized(terrain)) =
            map.world().resources.get::<Eventually<TerrainResources>>()
        else {
            panic!("terrain initialized");
        };
        terrain.draws().iter().map(|d| d.coords).collect()
    };
    assert!(!drawn_coords.is_empty());
    let first = drawn_coords[0];
    let patch = map
        .resident_terrain_patch(first)
        .expect("resident terrain patch");
    assert_eq!(patch.grid_side, 129);
    assert_eq!(patch.stitched_heights_meters.len(), 129 * 129);
    assert!(patch.stitched_to_neighbors);
    assert!(patch.max_elevation_meters > patch.min_elevation_meters);

    // Check shared East-West seam continuity across any adjacent rendered pair at the same zoom.
    let drawn_set: HashSet<WorldTileCoords> = drawn_coords.iter().copied().collect();
    if let Some(&west) = drawn_coords.iter().find(|c| {
        drawn_set.contains(&WorldTileCoords {
            x: c.x + 1,
            y: c.y,
            z: c.z,
        })
    }) {
        let east = WorldTileCoords {
            x: west.x + 1,
            y: west.y,
            z: west.z,
        };
        let w_patch = map.resident_terrain_patch(west).expect("west patch");
        let e_patch = map.resident_terrain_patch(east).expect("east patch");
        for row in 0..129 {
            let wh = w_patch.height_at_grid(128, row).expect("west border");
            let eh = e_patch.height_at_grid(0, row).expect("east border");
            assert!(
                (wh - eh).abs() <= 1e-2,
                "stitched seam mismatch at row {row}: west={wh}, east={eh}"
            );
        }
    }

    // Baked drapes reach the resident patch.
    map.run_frame().expect("draped frame");
    let draped_patch = map
        .resident_terrain_patch(first)
        .expect("draped terrain patch");
    assert!(
        draped_patch.drape_texture.is_some(),
        "the resident mipmapped drape texture handle is exposed"
    );
}
