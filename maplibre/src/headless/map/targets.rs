//! Zero-copy offscreen color and reversed-Z `Depth32Float` attachment accessors.

use crate::{
    headless::map::HeadlessMap,
    render::{eventually::Eventually, resource::Head},
};

impl HeadlessMap {
    /// The single-sample color texture the offscreen head renders into, in the surface format.
    pub fn head_texture(&self) -> Option<&wgpu::Texture> {
        match self.map_context.renderer.resources.surface.head() {
            Head::Headless(head) => Some(head.texture()),
            Head::Headed(_) => None,
        }
    }

    /// Persistent view of [`Self::head_texture`] for zero-copy GPU composition.
    pub fn head_texture_view(&self) -> Option<&wgpu::TextureView> {
        match self.map_context.renderer.resources.surface.head() {
            Head::Headless(head) => Some(head.view()),
            Head::Headed(_) => None,
        }
    }

    /// Resize the render attachments and viewport without discarding supplied tiles. The
    /// display keeps the pixel ratio [`Self::set_pixel_ratio`] gave it, so the viewport is the
    /// new size in layout pixels.
    pub fn resize(&mut self, size: crate::window::PhysicalSize) {
        let ratio = self
            .map_context
            .world
            .resources
            .get::<crate::sdf::provided::DisplayPixelRatio>()
            .map_or(1.0, |ratio| f64::from(ratio.0));
        self.map_context.resize(size, ratio);
    }

    /// Alias for [`Self::head_texture`].
    pub fn color_texture(&self) -> Option<&wgpu::Texture> {
        self.head_texture()
    }

    /// Alias for [`Self::head_texture_view`].
    pub fn color_texture_view(&self) -> Option<&wgpu::TextureView> {
        self.head_texture_view()
    }

    /// Enables or disables persistent single-sample reversed-Z `Depth32Float` readout populated
    /// via `DEPTH_COPY` at the end of each frame.
    pub fn set_depth_readout(&mut self, enabled: bool) {
        self.map_context.renderer.settings.depth_readout = enabled;
    }

    /// Borrows the persistent single-sample reversed-Z `Depth32Float` texture when
    /// [`Self::set_depth_readout`] (or [`crate::render::settings::RendererSettings::depth_readout`])
    /// was enabled before `run_frame()`.
    pub fn depth_texture(&self) -> Option<&wgpu::Texture> {
        match &self.map_context.renderer.resources.depth_readout_texture {
            Eventually::Initialized(texture) => Some(&texture.texture),
            Eventually::Uninitialized => None,
        }
    }

    /// Borrows a persistent view of [`Self::depth_texture`] for zero-copy GPU shader sampling.
    pub fn depth_texture_view(&self) -> Option<&wgpu::TextureView> {
        match &self.map_context.renderer.resources.depth_readout_texture {
            Eventually::Initialized(texture) => Some(&texture.view),
            Eventually::Uninitialized => None,
        }
    }

    /// Returns `true` if the offscreen target's `MAP_READ` staging buffer has been allocated.
    pub fn has_readback_buffer(&self) -> bool {
        match self.map_context.renderer.resources.surface.head() {
            Head::Headless(head) => head.has_readback_buffer(),
            Head::Headed(_) => false,
        }
    }
}

#[cfg(test)]
mod tests;
