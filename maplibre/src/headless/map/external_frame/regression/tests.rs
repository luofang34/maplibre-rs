#![allow(clippy::expect_used, clippy::panic)]

use std::time::Duration;

use cgmath::{Matrix4, Rad, Vector3};

use super::{tests::eye_view, ExternalFrame};
use crate::{
    context::MapContext,
    coords::LatLon,
    headless::{create_headless_renderer, map::HeadlessMap},
    render::{
        camera::EyeFrustum,
        eventually::{Eventually, Eventually::Initialized},
        RenderPlugin, RenderStageLabel,
    },
    style::Style,
    tcs::system::{stage::SystemStage, SystemResult},
    terrain::{resources::TerrainResources, DrapePhase},
};

#[derive(Default)]
struct Observations {
    extractions: usize,
    frames: Vec<(usize, Vec<crate::coords::WorldTileCoords>, usize)>,
}

fn ingest(context: &mut MapContext) -> SystemResult {
    let observations = context.world.resources.get_or_init_mut::<Observations>();
    observations.extractions = observations.extractions.wrapping_add(1);
    Ok(())
}

fn observe(context: &mut MapContext) -> SystemResult {
    let world = &mut context.world;
    let drawn = match world.resources.get::<Eventually<TerrainResources>>() {
        Some(Initialized(terrain)) => terrain.draws().iter().map(|draw| draw.coords).collect(),
        _ => Vec::new(),
    };
    let drapes = world
        .resources
        .get::<DrapePhase>()
        .map_or(0, |phase| phase.targets.len());
    let observations = world.resources.get_or_init_mut::<Observations>();
    observations
        .frames
        .push((observations.extractions, drawn, drapes));
    Ok(())
}

/// A frame of the view from `world_from_eye` of ground at Innsbruck.
pub(super) fn frame_from(timestamp: u64, world_from_eye: Matrix4<f64>) -> ExternalFrame {
    ExternalFrame {
        opaque_environment: false,
        timestamp: Duration::from_millis(timestamp),
        view: eye_view(
            LatLon::new(47.26, 11.39),
            0.0,
            world_from_eye,
            EyeFrustum::symmetric(Rad(1.4), 1.0, 0.05, 1.0e8),
        ),
        depth: None,
        request_overscan: 1.5,
    }
}

/// Four kilometres up, looking straight down.
pub(super) fn overhead() -> Matrix4<f64> {
    Matrix4::from_translation(Vector3::new(-0.032, 0.0, 4000.0))
}

fn frame(timestamp: u64) -> ExternalFrame {
    frame_from(timestamp, overhead())
}

#[tokio::test]
async fn a_still_view_ingests_and_refines_terrain_once_per_frame() {
    let style: Style = serde_json::from_str(r##"{"version":8,"sources":{"dem":{"type":"raster-dem","tiles":["https://dem.example/{z}/{x}/{y}.png"]}},"layers":[{"id":"background","type":"background","paint":{"background-color":"#718474"}}],"terrain":{"source":"dem"}}"##).expect("terrain style");
    let (kernel, renderer) = create_headless_renderer(2048, 2048, None)
        .await
        .expect("renderer");
    let mut map = HeadlessMap::new(
        style,
        renderer,
        kernel,
        vec![
            Box::new(RenderPlugin),
            Box::new(crate::background::BackgroundPlugin),
            Box::new(crate::terrain::TerrainPlugin::<
                crate::terrain::DefaultDemTransferables,
            >::default()),
        ],
    )
    .expect("map");
    // Deterministic ingestion at the same stage as worker results, without network timing.
    map.schedule.remove_stage(RenderStageLabel::Request);
    map.schedule.add_stage_before(
        RenderStageLabel::Prepare,
        RenderStageLabel::Request,
        SystemStage::default().with_system(ingest),
    );
    map.schedule
        .add_system_to_stage(RenderStageLabel::PhaseSort, observe);
    map.run_external_frame(frame(16)).expect("first frame");
    map.run_external_frame(frame(32)).expect("second frame");
    let observations = map
        .world()
        .resources
        .get::<Observations>()
        .expect("observations");
    assert_eq!(observations.extractions, 2);
    for observed in &observations.frames {
        assert!(!observed.1.is_empty());
        assert!(observed.2 <= 8);
    }
    assert!(observations.frames[0].2 > 0);
    // A tile may be refined into its children between the frames, but the surface they cover
    // stays the same: the area is equal and every tile of the second frame lies in one of the
    // first.
    let (first, second) = (&observations.frames[0].1, &observations.frames[1].1);
    use crate::coords::WorldTileCoords;
    let area = |tiles: &[WorldTileCoords]| -> f64 {
        tiles
            .iter()
            .map(|tile| 4_f64.powi(-i32::from(u8::from(tile.z))))
            .sum()
    };
    assert!(
        (area(first) - area(second)).abs() < 1e-15,
        "texture refinement keeps the entire stationary surface visible: {first:?} then {second:?}"
    );
    assert!(
        second.iter().all(|tile| {
            std::iter::successors(Some(*tile), WorldTileCoords::get_parent)
                .any(|ancestor| first.contains(&ancestor))
        }),
        "the second frame draws only within the first: {first:?} then {second:?}"
    );
}

#[tokio::test]
async fn moving_and_stationary_external_views_both_request_visible_tiles() {
    use crate::{projection::ProjectionType, render::frame_input::ViewSource};
    let style: Style = serde_json::from_str(r#"{"version":8,"sources":{"v":{"type":"vector","tiles":["https://unused.example/{z}/{x}/{y}.pbf"]}},"layers":[]}"#).expect("style");
    let (kernel, renderer) = create_headless_renderer(128, 128, None)
        .await
        .expect("renderer");
    let mut map = HeadlessMap::new(
        style,
        renderer,
        kernel,
        vec![
            Box::new(RenderPlugin),
            Box::new(crate::vector::VectorPlugin::<
                crate::vector::DefaultVectorTransferables,
            >::default()),
        ],
    )
    .expect("map");
    let initial = frame(16).view;
    let settled = frame_from(
        16,
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 2000.0)),
    )
    .view;
    map.map_context
        .view_state
        .set_external_view(initial, &ProjectionType::Mercator)
        .expect("initial pose");
    map.map_context.view_state.update_references();
    map.frame_input_mut().view = ViewSource::External(settled);
    run_input_and_requests(&mut map).expect("moving frame");
    assert!(!map.view_state().eye_settled());
    assert!(
        !map.world().tiles.tiles.is_empty(),
        "motion must not suspend visible requests"
    );
    map.map_context.world.tiles.clear();
    map.map_context.view_state.update_references();
    run_input_and_requests(&mut map).expect("stationary frame");
    assert!(map.view_state().eye_settled());
    assert!(!map.view_state().did_camera_change());
    assert!(
        !map.world().tiles.tiles.is_empty(),
        "settling resumes requests without movement"
    );
}

#[tokio::test]
async fn an_invalid_view_preserves_the_last_frame_and_recovers() {
    let style: Style = serde_json::from_str(r##"{"version":8,"sources":{},"layers":[{"id":"background","type":"background","paint":{"background-color":"#718474"}}]}"##).expect("style");
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
        ],
    )
    .expect("map");
    map.run_external_frame(frame(16)).expect("valid view");
    let previous = map.view_state().view_projection().0;
    let mut invalid = frame(32);
    invalid.view.frustum.near = f64::NAN;
    assert!(matches!(
        map.run_external_frame(invalid),
        Err(crate::headless::map::HeadlessMapOperationError::ExternalView { .. })
    ));
    assert_eq!(map.view_state().view_projection().0, previous);
    map.run_external_frame(frame(48))
        .expect("the view recovered");
}

#[tokio::test]
async fn an_external_frame_lands_in_one_timeline_record() {
    use crate::render::frame_trace::Clock;
    let style: Style =
        serde_json::from_str(r#"{"version":8,"sources":{},"layers":[]}"#).expect("style");
    let (kernel, renderer) = create_headless_renderer(64, 64, None)
        .await
        .expect("renderer");
    let mut map =
        HeadlessMap::new(style, renderer, kernel, vec![Box::new(RenderPlugin)]).expect("map");
    map.enable_frame_trace(16);
    for timestamp in [16, 32] {
        map.run_external_frame(frame(timestamp)).expect("frame");
        // The host records under the number the map reports after the frame.
        let host_frame = map.last_frame_stats().host_frame;
        map.frame_trace_mut().expect("trace").record_span(
            host_frame,
            "copy-encode",
            Clock::Cpu,
            Duration::from_micros(10),
        );
    }
    let export = map.frame_trace_mut().expect("trace").export();
    assert_eq!(
        export
            .frames
            .iter()
            .map(|record| record.frame)
            .collect::<Vec<_>>(),
        [1, 2],
        "one record per frame"
    );
    for record in &export.frames {
        let renders = record
            .spans
            .iter()
            .filter(|span| span.name == "Render")
            .count();
        assert_eq!(
            renders, 1,
            "the frame's stages are in its record: {record:?}"
        );
        assert!(record.spans.iter().any(|span| span.name == "copy-encode"));
    }
}

#[derive(Default)]
struct MovedFrames(Vec<bool>);

fn note_moved(context: &mut MapContext) -> SystemResult {
    let moved = crate::render::frame_signals::camera_moved(&context.world, &context.view_state);
    context
        .world
        .resources
        .get_or_init_mut::<MovedFrames>()
        .0
        .push(moved);
    Ok(())
}

#[tokio::test]
async fn a_still_external_view_is_at_rest() {
    let style: Style =
        serde_json::from_str(r#"{"version":8,"sources":{},"layers":[]}"#).expect("style");
    let (kernel, renderer) = create_headless_renderer(64, 64, None)
        .await
        .expect("renderer");
    let mut map =
        HeadlessMap::new(style, renderer, kernel, vec![Box::new(RenderPlugin)]).expect("map");
    map.schedule
        .add_system_to_stage(RenderStageLabel::Queue, note_moved);
    // The same view twice: the map sees what it saw the frame before.
    map.run_external_frame(frame(16)).expect("first frame");
    map.run_external_frame(frame(32)).expect("second frame");
    let moved = &map
        .world()
        .resources
        .get::<MovedFrames>()
        .expect("observed")
        .0;
    assert_eq!(
        moved[1..],
        [false],
        "the view did not move, so drapes catch up at full pace: {moved:?}"
    );
}

/// Applies the frame input and runs the request systems, without drawing.
fn run_input_and_requests(
    map: &mut crate::headless::map::HeadlessMap,
) -> crate::schedule::StageResult {
    for label in [RenderStageLabel::Input, RenderStageLabel::Request] {
        let stage = map
            .schedule
            .get_stage_mut::<SystemStage>(&label)
            .expect("stage");
        crate::schedule::Stage::run(stage, &mut map.map_context)?;
    }
    Ok(())
}
