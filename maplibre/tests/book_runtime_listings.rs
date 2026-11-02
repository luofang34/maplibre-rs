//! The Rust listings of the book's runtime examples. The book includes them by anchor, so they
//! cannot drift from the API they describe.
#![allow(dead_code)]

use std::{sync::Arc, time::Duration};

use cgmath::Deg;
use maplibre::{
    coords::LatLon,
    environment::Environment,
    map::{Map, MapError},
    render::view_state::{CameraOptions, FlightCurve, FlyOptions},
    sdf::query::{QueryGeometry, QueryOptions},
    style::source::{GeoJsonData, Source},
    window::{HeadedMapWindow, MapWindowConfig},
};
use serde_json::json;

fn fly_to<E: Environment>(map: &mut Map<E>) -> Result<(), MapError>
where
    <E::MapWindowConfig as MapWindowConfig>::MapWindow: HeadedMapWindow,
{
    // ANCHOR: fly_to
    let destination = CameraOptions {
        center: Some(LatLon::new(47.27574, 11.39085)),
        zoom: Some(12.0),
        pitch: Some(Deg(60.0)),
        ..CameraOptions::default()
    };
    map.context_mut()?.fly_to(
        destination,
        FlightCurve::Fly(FlyOptions {
            duration: Some(Duration::from_secs(6)),
            ..FlyOptions::default()
        }),
    );
    // ANCHOR_END: fly_to
    Ok(())
}

fn restyle<E: Environment>(map: &mut Map<E>) -> Result<(), MapError>
where
    <E::MapWindowConfig as MapWindowConfig>::MapWindow: HeadedMapWindow,
{
    // ANCHOR: restyle
    map.set_paint_property("water", "fill-color", json!("#1c7ed6"))?;
    map.set_layout_property("place-label", "visibility", json!("none"))?;
    map.set_filter("road", Some(json!(["==", ["get", "class"], "motorway"])))?;
    // ANCHOR_END: restyle
    Ok(())
}

fn geojson<E: Environment>(map: &mut Map<E>, progress: f64) -> Result<(), MapError>
where
    <E::MapWindowConfig as MapWindowConfig>::MapWindow: HeadedMapWindow,
{
    // ANCHOR: geojson
    let position = json!({
        "type": "Point",
        "coordinates": [progress * 360.0 - 180.0, 0.0]
    });
    map.set_geojson_data("vehicle", GeoJsonData::Inline(Arc::new(position)))?;
    // ANCHOR_END: geojson
    Ok(())
}

fn add_layer<E: Environment>(map: &mut Map<E>, source: Source) -> Result<(), MapError>
where
    <E::MapWindowConfig as MapWindowConfig>::MapWindow: HeadedMapWindow,
{
    // ANCHOR: add_layer
    map.add_source("urban-areas", source)?;
    map.add_layer(
        json!({
            "id": "urban-areas-fill",
            "type": "fill",
            "source": "urban-areas",
            "paint": { "fill-color": "#f08", "fill-opacity": 0.4 }
        }),
        Some("place-label"),
    )?;
    // ANCHOR_END: add_layer
    Ok(())
}

fn query<E: Environment>(map: &Map<E>, cursor: [f64; 2]) -> Result<Vec<String>, MapError>
where
    <E::MapWindowConfig as MapWindowConfig>::MapWindow: HeadedMapWindow,
{
    // ANCHOR: query
    let options = QueryOptions {
        layers: Some(vec!["countries-fill".to_owned()]),
        filter: None,
    };
    let names = map
        .query_rendered_features(QueryGeometry::Point(cursor), &options)?
        .into_iter()
        .filter_map(|feature| feature.properties.get("NAME").cloned())
        .map(|name| name.to_string())
        .collect();
    // ANCHOR_END: query
    Ok(names)
}
