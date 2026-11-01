#![allow(clippy::expect_used, clippy::panic)]

use super::drawn_covering;
use crate::{
    coords::{LatLon, WorldCoords, WorldTileCoords, Zoom, TILE_SIZE},
    render::view_state::ViewState,
    style::Style,
    tcs::world::World,
};

fn style() -> Style {
    serde_json::from_str(
        r#"{"version":8,"sources":{"osm":{"type":"vector","tiles":["https://osm.example/{z}/{x}/{y}.pbf"],"maxzoom":14}},"layers":[]}"#,
    )
    .expect("a vector style parses")
}

fn view(zoom: f64) -> ViewState {
    let zoom = Zoom::new(zoom);
    ViewState::new(
        crate::window::PhysicalSize::new(1024, 768).expect("a viewport"),
        WorldCoords::from_lat_lon(LatLon::new(47.26, 11.39), zoom),
        zoom,
        cgmath::Deg(0.0),
        cgmath::Rad(0.8),
    )
}

fn tiles(style: &Style, view: &ViewState, world: &mut World) -> Vec<WorldTileCoords> {
    let level = view.zoom().zoom_level(TILE_SIZE);
    let (region, _) = drawn_covering(style, view, world, level).expect("a covering");
    region.expect("a region").iter().collect()
}

#[test]
fn every_view_selects_its_own_tiles() {
    let style = style();
    let mut world = World::default();
    let coarse = tiles(&style, &view(6.0), &mut world);
    let fine = tiles(&style, &view(7.0), &mut world);
    assert_ne!(coarse, fine);
}
