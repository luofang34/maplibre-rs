#![allow(clippy::expect_used, clippy::panic)]

use super::*;

#[tokio::test]
async fn level_transitions_keep_terrain_and_sky_drawable() {
    let mut map = map().await;
    for (index, pitch) in [45.0, 70.0, 89.9, 90.0, 90.1, 90.0, 45.0, 90.0]
        .into_iter()
        .enumerate()
    {
        let mut input = frame_from(
            index as u64 * 16,
            overhead()
                * Matrix4::from_angle_z(Deg(37.3))
                * Matrix4::from_angle_x(Deg(pitch))
                * Matrix4::from_angle_z(Deg(3.7)),
        );
        input.opaque_environment = true;
        input.view.frustum.near = 0.001;
        input.view.frustum.far = 1.0e7;
        map.run_external_frame(input).expect("level transition");
        let Some(Initialized(terrain)) =
            map.world().resources.get::<Eventually<TerrainResources>>()
        else {
            panic!("terrain resources");
        };
        assert!(
            !terrain.draws().is_empty(),
            "ground remains covered at pitch {pitch}"
        );
        let pixels = read_back_blocking(&map);
        assert!(
            pixels
                .as_chunks::<4>()
                .0
                .iter()
                .all(|pixel| pixel[3] == 255),
            "ground and sky cover the whole frame at pitch {pitch}"
        );
    }
}
