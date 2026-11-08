//! Frames drawn from a view the host supplies, through the offscreen map.

use std::time::Duration;

use crate::{
    headless::map::{HeadlessMap, HeadlessMapOperationError},
    projection::ProjectionType,
    render::{frame_input::ViewSource, view_state::ExternalView},
};

/// One frame of an [`ExternalView`].
#[derive(Debug)]
pub(crate) struct ExternalFrame {
    /// Draw continuous sky across the whole frame.
    pub(crate) opaque_environment: bool,
    /// Time since the host started; animated properties read it.
    pub(crate) timestamp: Duration,
    /// The view to draw.
    pub(crate) view: ExternalView,
    /// A `Depth32Float` texture of the surface size the frame's depth is written to.
    pub(crate) depth: Option<wgpu::TextureView>,
    /// How far beyond the frustum tiles are requested, as a factor on its tangents.
    pub(crate) request_overscan: f64,
}

impl HeadlessMap {
    /// Draws `frame`, or returns the reason its view is invalid with the map as it was.
    pub(crate) fn run_external_frame(
        &mut self,
        frame: ExternalFrame,
    ) -> Result<(), HeadlessMapOperationError> {
        let projection = self
            .map_context
            .style
            .projection
            .as_ref()
            .map_or(ProjectionType::Mercator, |spec| {
                spec.projection_type.clone()
            });
        self.map_context
            .view_state
            .clone()
            .set_external_view(frame.view, &projection)
            .map_err(|source| HeadlessMapOperationError::ExternalView { source })?;
        self.map_context
            .view_state
            .set_opaque_environment(frame.opaque_environment);
        let input = self.frame_input_mut();
        input.timestamp = frame.timestamp;
        input.view = ViewSource::External(frame.view);
        input.request_overscan = frame.request_overscan;
        self.map_context.renderer.resources.eye_depth_target = frame.depth;
        let result = crate::render::frame::run_frame(&mut self.schedule, &mut self.map_context);
        let resources = &mut self.map_context.renderer.resources;
        resources.eye_depth_target = None;
        // The graph runner takes the target after a frame; a failed frame must not leave it
        // behind for the next one.
        resources.render_target.take();
        result.map_err(|source| HeadlessMapOperationError::Schedule { source })
    }
}

#[cfg(test)]
pub(crate) mod tests;

#[cfg(test)]
#[path = "external_frame/resizing/tests.rs"]
mod resizing;

#[cfg(test)]
#[path = "external_frame/imagery/tests.rs"]
mod imagery;

#[cfg(test)]
#[path = "external_frame/regression/tests.rs"]
mod regression;

#[cfg(test)]
#[path = "external_frame/surface/tests.rs"]
mod surface;

#[cfg(test)]
#[path = "external_frame/poles/tests.rs"]
mod poles;

#[cfg(test)]
#[path = "external_frame/drape_sources/tests.rs"]
mod drape_sources;
