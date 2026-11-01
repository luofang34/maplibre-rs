#![allow(clippy::expect_used, clippy::panic)]

use super::Body;

#[test]
fn earth_is_the_default_and_a_smaller_body_scales_every_derived_length() {
    let moon = Body {
        radius_meters: 1_737_400.0,
        highest_ground_meters: 10_786.0,
        lowest_ground_meters: -9_129.0,
    };

    assert_eq!(Body::default(), Body::EARTH);
    assert_eq!(Body::EARTH.lowest_ground_meters, -450.0);
    assert!((Body::EARTH.circumference_meters() - 40_030_228.9).abs() < 1.0);
    assert!(
        (moon.circumference_at_latitude(60.0) - moon.circumference_meters() * 0.5).abs() < 1e-6
    );
    assert!((moon.unit_radius_at(1_737.4) - 1.001).abs() < 1e-9);
}
