#![allow(clippy::expect_used)]

use cgmath::{InnerSpace, Vector3, Vector4};

use super::*;
use crate::{
    coords::{LatLon, Zoom},
    projection::body::Body,
};

#[test]
fn orientation_3d_handles_steep_bank_vertical_pitch_and_inverted_attitudes() {
    let body = Body::EARTH;
    let loc = LatLon::new(37.6188, -122.375);
    let (east, north, up) = enu_axes_on_unit_sphere(loc);

    // 1. Heading 90° (due East), 60° right bank, level pitch:
    // Nose (+Y_body) points due East; right wing (+X_body) drops below horizon (-up).
    let banked = Placement3d::new_msl(
        loc,
        3_000.0,
        Orientation3d::from_heading_pitch_roll_degrees(90.0, 0.0, 60.0),
    );
    let mat = banked.model_to_unit_sphere(&body, 0.0, None);
    let origin = mat.w.truncate();
    assert!((origin.magnitude() - body.unit_radius_at(3_000.0)).abs() < 1e-12);

    let nose_dir = (mat * Vector4::new(0.0, 10.0, 0.0, 0.0))
        .truncate()
        .normalize();
    let right_wing_dir = (mat * Vector4::new(10.0, 0.0, 0.0, 0.0))
        .truncate()
        .normalize();
    let dorsal_dir = (mat * Vector4::new(0.0, 0.0, 10.0, 0.0))
        .truncate()
        .normalize();
    assert!(
        (nose_dir - east).magnitude() < 1e-12,
        "nose must point due East"
    );
    assert!(
        (right_wing_dir.dot(up) - (-60.0_f64.to_radians().sin())).abs() < 1e-12,
        "60° right bank must drop right wing by sin(60°)"
    );
    assert!((nose_dir.cross(dorsal_dir).magnitude() - 1.0).abs() < 1e-12);

    // 2. Pure +90° vertical climb and -90° vertical dive without gimbal lock:
    let climb = Placement3d::new_msl(
        loc,
        1_500.0,
        Orientation3d::from_heading_pitch_roll_degrees(35.0, 90.0, 45.0),
    );
    let climb_nose = (climb.model_to_unit_sphere(&body, 0.0, None)
        * Vector4::new(0.0, 1.0, 0.0, 0.0))
    .truncate()
    .normalize();
    assert!(
        (climb_nose - up).magnitude() < 1e-12,
        "+90° pitch must point nose straight Up"
    );

    let dive = Placement3d::new_msl(
        loc,
        1_500.0,
        Orientation3d::from_heading_pitch_roll_degrees(215.0, -90.0, -30.0),
    );
    let dive_nose = (dive.model_to_unit_sphere(&body, 0.0, None)
        * Vector4::new(0.0, 1.0, 0.0, 0.0))
    .truncate()
    .normalize();
    assert!(
        (dive_nose + up).magnitude() < 1e-12,
        "-90° pitch must point nose straight Down"
    );

    // 3. ±180° inverted level flight due North: dorsal (+Z_body) points straight Down (-up).
    let inverted = Placement3d::new_msl(
        loc,
        2_000.0,
        Orientation3d::from_heading_pitch_roll_degrees(0.0, 0.0, 180.0),
    );
    let inv_mat = inverted.model_to_unit_sphere(&body, 0.0, None);
    let inv_nose = (inv_mat * Vector4::new(0.0, 1.0, 0.0, 0.0))
        .truncate()
        .normalize();
    let inv_dorsal = (inv_mat * Vector4::new(0.0, 0.0, 1.0, 0.0))
        .truncate()
        .normalize();
    assert!((inv_nose - north).magnitude() < 1e-12);
    assert!((inv_dorsal + up).magnitude() < 1e-12);
}

#[test]
fn slope_aligned_ground_vehicle_and_mercator_transform_match_datum() {
    let body = Body::EARTH;
    let loc = LatLon::new(47.0, 8.0);
    // 10° uphill runway sloping down to the South (normal tilted toward South -Y_enu).
    let normal_enu = Vector3::new(
        0.0,
        -10.0_f64.to_radians().sin(),
        10.0_f64.to_radians().cos(),
    );
    let vehicle = Placement3d {
        position: loc,
        altitude_meters: 1.2,
        altitude_reference: AltitudeReference::OnGroundSlopeAligned,
        orientation: Orientation3d::from_heading_pitch_roll_degrees(0.0, 0.0, 0.0),
    };
    assert!((vehicle.resolved_altitude_msl(430.0) - 431.2).abs() < 1e-12);

    let rot = vehicle
        .effective_orientation(Some(normal_enu))
        .to_rotation_matrix3();
    let vehicle_up_enu = rot * Vector3::new(0.0, 0.0, 1.0);
    assert!(
        (vehicle_up_enu - normal_enu).magnitude() < 1e-12,
        "OnGroundSlopeAligned must align vehicle dorsal +Z with terrain normal"
    );

    let merc = vehicle.model_to_mercator_world(Zoom::new(16.0), &body, 430.0, Some(normal_enu));
    assert!((merc.w.z - 431.2).abs() < 1e-9);
}
