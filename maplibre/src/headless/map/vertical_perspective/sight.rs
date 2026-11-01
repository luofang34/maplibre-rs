//! Ground the frame draws is in sight of the camera, and ground hills nearer the camera
//! cover is not, as the CPU line of sight tells them.

use super::*;

/// Through [`HeadlessMap::drawn_terrain`], ground the frame draws is in sight of the camera,
/// and ground nearer drawn ground covers on the screen is not.
#[tokio::test]
async fn drawn_terrain_sees_the_picked_ground_and_not_what_ridges_cover() {
    use crate::terrain::sightline::{
        pick_globe_terrain, target_view, TargetAltitude, TerrainPick, Visibility,
    };
    let scene = Scene {
        meters: 1500.0,
        relief: 600.0,
        exaggeration: 1.0,
        pitch: 80.0,
        zoom: 11.67,
    };
    let mut hills = LevelMap::new(scene).await;
    hills.settle(None).await;
    let camera = globe_camera_for_view(hills.map.view_state()).expect("camera");
    let terrain = hills
        .map
        .context()
        .drawn_terrain(camera.body())
        .expect("terrain drawn");
    let mut picks = Vec::new();
    for y in (HEIGHT / 3..HEIGHT).step_by(3) {
        let pixel = Point2::new(f64::from(WIDTH / 2) + 0.5, f64::from(y) + 0.5);
        if let TerrainPick::Ground(hit) = pick_globe_terrain(&camera, terrain, pixel) {
            picks.push(crate::coords::LatLon::new(
                (std::f64::consts::PI * (1.0 - 2.0 * hit.mercator.y))
                    .sinh()
                    .atan()
                    .to_degrees(),
                hit.mercator.x * 360.0 - 180.0,
            ));
        }
    }
    assert!(picks.len() > 100, "{} ground picks", picks.len());
    let seen = |at| target_view(&camera, Some(terrain), at, TargetAltitude::OnGround).visibility;
    let visible = picks
        .iter()
        .filter(|&&at| seen(at) == Visibility::Visible)
        .count();
    assert!(
        visible * 100 >= picks.len() * 98,
        "{visible} of {} picked points in sight",
        picks.len()
    );
    // Over the picked region, ground is hidden exactly when the pick through its pixel lands
    // nearer the camera than it.
    let (lat, lon) = picks.iter().fold(
        ((f64::MAX, f64::MIN), (f64::MAX, f64::MIN)),
        |((a, b), (c, d)), p| {
            (
                (a.min(p.latitude), b.max(p.latitude)),
                (c.min(p.longitude), d.max(p.longitude)),
            )
        },
    );
    let eye = camera.camera_position();
    let (mut agree, mut checked, mut hidden) = (0, 0, 0);
    for i in 0..60 {
        for j in 0..60 {
            let at = crate::coords::LatLon::new(
                lat.0 + (lat.1 - lat.0) * f64::from(i) / 59.0,
                lon.0 + (lon.1 - lon.0) * f64::from(j) / 59.0,
            );
            let view = target_view(&camera, Some(terrain), at, TargetAltitude::OnGround);
            let (Some(pixel), Some(elevation)) = (view.pixel, view.elevation) else {
                continue;
            };
            // Off the screen no pick describes the point.
            if view.visibility == Visibility::OutsideFrustum {
                continue;
            }
            let TerrainPick::Ground(hit) = pick_globe_terrain(&camera, terrain, pixel) else {
                continue;
            };
            let unit = |at, meters| {
                crate::projection::globe::lat_lon_to_unit_sphere(at)
                    * camera.body().unit_radius_at(meters)
            };
            let picked = crate::coords::LatLon::new(
                (std::f64::consts::PI * (1.0 - 2.0 * hit.mercator.y))
                    .sinh()
                    .atan()
                    .to_degrees(),
                hit.mercator.x * 360.0 - 180.0,
            );
            let own = (unit(at, elevation) - eye).magnitude();
            let first = (unit(picked, hit.elevation) - eye).magnitude();
            // Too close to call: the pick and the point lie on the same patch of ground.
            if (first / own - 1.0).abs() < 2e-3 {
                continue;
            }
            let covered = first < own;
            checked += 1;
            hidden += usize::from(covered);
            let said = match view.visibility {
                Visibility::Visible => Some(false),
                Visibility::BehindTerrain => Some(true),
                _ => None,
            };
            agree += usize::from(said == Some(covered));
        }
    }
    assert!(hidden > 50, "{hidden} of {checked} points behind hills");
    assert!(
        agree == checked,
        "line of sight agrees with the picks for {agree} of {checked} points"
    );
}
