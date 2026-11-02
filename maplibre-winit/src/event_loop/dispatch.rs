//! Input and frames for an initialized map.
use instant::Instant;
use maplibre::{
    environment::Environment,
    io::apc::AsyncProcedureCall,
    map::Map,
    render::frame_input::FrameInput,
    window::{HeadedMapWindow, MapWindow, MapWindowConfig, PhysicalSize},
};
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
};

use crate::input::{InputController, UpdateState};

pub(super) struct Dispatch<E: Environment>
where
    <E::MapWindowConfig as MapWindowConfig>::MapWindow: HeadedMapWindow,
{
    pub(super) map: Map<E>,
    input: InputController,
    started: Instant,
    last_render: Instant,
    frame: u64,
    max_frames: Option<u64>,
    scale_factor: f64,
}

impl<E: Environment> Dispatch<E>
where
    <E::MapWindowConfig as MapWindowConfig>::MapWindow: HeadedMapWindow,
{
    pub(super) fn new(map: Map<E>, max_frames: Option<u64>) -> Self {
        let now = Instant::now();
        let scale_factor = map.window().scale_factor();
        Self {
            map,
            input: InputController::new(0.2, 100.0, 1.0),
            started: now,
            last_render: now,
            frame: 0,
            max_frames,
            scale_factor,
        }
    }

    pub(super) fn device_event(&mut self, event: &winit::event::DeviceEvent) {
        self.input.device_input(event);
    }

    pub(super) fn sync_size(&mut self) {
        self.scale_factor = self.map.window().scale_factor();
        let size = self.map.window().size();
        if let Ok(context) = self.map.context_mut() {
            context.resize(size, self.scale_factor);
        }
        self.last_render = Instant::now();
    }

    pub(super) fn window_event(&mut self, event: &WindowEvent, target: &ActiveEventLoop) {
        if matches!(event, WindowEvent::RedrawRequested) {
            return;
        }
        if self.input.window_input(event, self.scale_factor) {
            // The gesture is applied when the next frame updates the view.
            self.map.window().request_redraw();
            return;
        }
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => target.exit(),
            WindowEvent::Resized(size) => {
                // Minimized windows can report a zero extent, which cannot configure a surface.
                if let Some(size) = PhysicalSize::new(size.width, size.height) {
                    if let Ok(context) = self.map.context_mut() {
                        context.resize(size, self.scale_factor);
                        self.map.window().request_redraw();
                    }
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = *scale_factor;
                let size = self.map.window().size();
                if let Ok(context) = self.map.context_mut() {
                    context.resize(size, self.scale_factor);
                    self.map.window().request_redraw();
                }
            }
            _ => {}
        }
    }

    pub(super) fn redraw(
        &mut self,
        target: &ActiveEventLoop,
    ) -> Result<(), maplibre::map::MapError> {
        if self.max_frames == Some(0) {
            target.exit();
            return Ok(());
        }
        let now = Instant::now();
        let elapsed = now - self.last_render;
        self.last_render = now;
        if let Ok(context) = self.map.context_mut() {
            context
                .world
                .resources
                .get_or_init_mut::<FrameInput>()
                .timestamp = now - self.started;
            self.input.update_state(context, elapsed);
        }
        self.map.run_schedule()?;
        self.frame = self.frame.wrapping_add(1);
        if self.max_frames.is_some_and(|max| self.frame >= max) {
            target.exit();
        }
        let next = NextFrame {
            map: self.map.needs_redraw(),
            retry: self.map.has_scheduled_retries(),
            arrivals: self.map.kernel().apc().has_arrivals(),
            input: self.input.moved_view(),
            counting_frames: self.max_frames.is_some(),
        };
        // An idle map draws nothing until an event or a change asks for a frame.
        if next.wanted() {
            self.map.window().request_redraw();
        }
        Ok(())
    }
}

/// What decides whether the event loop asks for another frame after drawing one.
#[derive(Clone, Copy, Debug, Default)]
struct NextFrame {
    /// The map changed, animates or still loads since the frame it drew.
    map: bool,
    /// A failed tile waits for its retry deadline, which only a frame notices.
    retry: bool,
    /// Worker results arrived that the next frame applies.
    arrivals: bool,
    /// Input moved the view, which inertia or a held key carries into the next frame.
    input: bool,
    /// The host asked for a number of frames, which are drawn whether or not anything changes.
    counting_frames: bool,
}

impl NextFrame {
    fn wanted(self) -> bool {
        self.map || self.retry || self.arrivals || self.input || self.counting_frames
    }
}

#[cfg(test)]
mod tests;
