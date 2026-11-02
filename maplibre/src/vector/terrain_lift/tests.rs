use super::*;

fn vertices(positions: &[[f32; 2]]) -> Vec<ShaderVertex> {
    positions
        .iter()
        .map(|position| ShaderVertex::new(*position, [0.0, 1.0]))
        .collect()
}

#[test]
fn a_building_rises_from_the_ground_at_the_middle_of_its_footprint() {
    let mut building = vertices(&[[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]]);
    for vertex in &mut building {
        vertex.elevation = 10.0;
        vertex.distance = 10.0;
    }
    elevate_span(&mut building, Lift::Extrusion, &|[x, y]| {
        Some(f64::from(x + y) * 100.0)
    });
    assert!(building.iter().all(|vertex| vertex.elevation == 410.0));
    assert!(building.iter().all(|vertex| vertex.distance == 410.0));
}

#[test]
fn a_heatmap_point_stands_on_the_surface_under_it() {
    let mut points = vertices(&[[0.0, 0.0], [4.0, 0.0]]);
    elevate_span(&mut points, Lift::Surface, &|[x, _]| {
        Some(f64::from(x) * 100.0)
    });
    assert_eq!(points[0].elevation, 0.0);
    assert_eq!(points[1].elevation, 400.0);
}

#[test]
fn only_heatmaps_and_extrusions_are_lifted() {
    let layer = |kind: &str| -> StyleLayer {
        serde_json::from_value(serde_json::json!({"id": "l", "type": kind, "source": "s"}))
            .unwrap_or_else(|error| panic!("{kind}: {error}"))
    };
    assert!(matches!(lift(&layer("heatmap")), Some(Lift::Surface)));
    assert!(matches!(
        lift(&layer("fill-extrusion")),
        Some(Lift::Extrusion)
    ));
    assert!(lift(&layer("line")).is_none());
}
