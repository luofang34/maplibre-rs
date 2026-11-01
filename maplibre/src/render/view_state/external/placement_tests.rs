//! Views of a scene the host scaled and placed in its own world.
#![allow(clippy::expect_used, clippy::panic)]

use cgmath::{Deg, InnerSpace, Matrix4, Rad, SquareMatrix, Vector3};

use super::{ExternalAnchor, ExternalView};
use crate::{
    coords::{LatLon, WorldCoords, Zoom},
    projection::{globe::unit_sphere_to_lat_lon, ProjectionType},
    render::{camera::EyeFrustum, view_state::ViewState},
    window::PhysicalSize,
};

fn scene_view_state() -> ViewState {
    let zoom = Zoom::new(4.0);
    ViewState::new(
        PhysicalSize::new(800, 600).expect("non-zero size"),
        WorldCoords::from_lat_lon(LatLon::new(0.0, 0.0), zoom),
        zoom,
        Deg(0.0),
        Rad(0.6435011087932844),
    )
}

#[test]
fn a_scaled_placement_puts_the_eye_where_the_scene_sees_it() {
    // A scene a thousand times smaller than the world, moved a metre up, seen by an eye
    // two metres above the world origin looking straight down.
    let world_from_scene =
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)) * Matrix4::from_scale(1e-3);
    let world_from_eye = Matrix4::from_translation(Vector3::new(0.0, 0.0, 2.0));
    let view = ExternalView {
        anchor: ExternalAnchor {
            position: LatLon::new(47.0, 11.0),
            altitude_meters: 500.0,
        },
        view: world_from_eye
            .invert()
            .expect("a translation is invertible")
            * world_from_scene,
        frustum: EyeFrustum::symmetric(Rad(1.0), 1.0, 0.05, 50.0),
    };
    let mut view_state = scene_view_state();
    view_state
        .set_external_view(view, &ProjectionType::Mercator)
        .expect("the eye is above the scene");

    let pose = view_state.camera_pose();
    // One metre above the scene origin in the world is a thousand scene metres.
    assert!((pose.altitude_meters - 1500.0).abs() < 1e-6, "{pose:?}");
    assert!(pose.pitch.0.abs() < 1e-6, "{pose:?}");
    assert!(
        pose.bearing.0.abs() < 1e-6 && pose.roll.0.abs() < 1e-6,
        "{pose:?}"
    );
    assert!(
        (pose.position.latitude - 47.0).abs() < 1e-9
            && (pose.position.longitude - 11.0).abs() < 1e-9,
        "{pose:?}"
    );
    let projection = view_state
        .external_projection()
        .expect("an external view is in effect");
    // Clip distances arrive in world metres and reach the map in its pixels: the near
    // plane is 50 scene metres away, at the map's pixels per metre.
    let near = 0.05 / 1e-3 * view_state.pixels_per_meter();
    let expected = EyeFrustum::symmetric(Rad(1.0), 1.0, near, near * 1000.0).projection();
    assert!((projection.x.x - expected.x.x).abs() < 1e-9);
    assert!((projection.w.z - expected.w.z).abs() < 1e-6 * expected.w.z.abs());
}

#[test]
fn a_globe_standing_on_a_table_is_seen_from_where_the_viewer_stands() {
    // A globe fifteen centimetres in radius, its polar axis vertical (world y) and the
    // anchor turned towards the viewer at +z, its center a metre ahead of the eye and
    // thirty centimetres below it.
    let radius_meters = 6_371_008.8;
    let radius = 0.15;
    let scale = radius / radius_meters;
    let latitude = 47.26_f64.to_radians();
    let east = Vector3::new(1.0, 0.0, 0.0);
    let north = Vector3::new(0.0, latitude.cos(), -latitude.sin());
    let up = Vector3::new(0.0, latitude.sin(), latitude.cos());
    let rotation = cgmath::Matrix3::from_cols(east, north, up);
    let center = Vector3::new(0.0, -0.3, -1.0);
    let world_from_scene = Matrix4::from_translation(center + up * radius)
        * Matrix4::from(rotation)
        * Matrix4::from_scale(scale);
    let view = ExternalView {
        anchor: ExternalAnchor {
            position: LatLon::new(47.26, 11.39),
            altitude_meters: 0.0,
        },
        view: world_from_scene,
        frustum: EyeFrustum::symmetric(Rad(1.2), 16.0 / 9.0, 0.1, 100.0),
    };
    let mut view_state = scene_view_state();
    view_state
        .set_external_view(view, &ProjectionType::Globe)
        .expect("the eye is beside the globe");

    let eye = view_state
        .external_globe_eye()
        .expect("the globe camera takes the eye");
    // The eye is a metre and a bit from the center, seven radii out.
    let distance = (center.magnitude()) / radius;
    assert!(
        (eye.position.magnitude() - distance).abs() < 1e-6,
        "{eye:?}"
    );
    // The view axis misses the globe, so the map center is the point beneath the eye: the
    // eye's direction from the globe center, thirty degrees south of the anchor.
    let toward_eye = -center / center.magnitude();
    let expected_latitude = 47.26 - toward_eye.dot(up).acos().to_degrees();
    let beneath_eye = unit_sphere_to_lat_lon(eye.position.normalize());
    assert!(
        (beneath_eye.latitude - expected_latitude).abs() < 1e-6,
        "{beneath_eye:?} vs latitude {expected_latitude}"
    );
    assert!(
        (beneath_eye.longitude - 11.39).abs() < 1e-6,
        "{beneath_eye:?}"
    );
    let map_center = view_state.external_view().anchor.position;
    assert!(
        (map_center.latitude - expected_latitude).abs() < 1e-6
            && (map_center.longitude - 11.39).abs() < 1e-6,
        "{map_center:?}"
    );
    // The zoom follows the eye's distance to that point: six radii out.
    let zoom_distance = view_state.camera_to_center_distance() / view_state.pixels_per_meter();
    assert!(
        (zoom_distance - (distance - 1.0) * radius_meters).abs() < 1.0,
        "{zoom_distance} metres to the center"
    );
}
