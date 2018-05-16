use std::ops::Deref;

// These must match mouse_listener.js
pub const MOUSE_EVENT_MAX : usize = 100;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MouseButton {
    Left = 1,
    Middle = 2,
    Right = 3,
}

mod event;
use self::event::{MouseEvent, MouseEventType};

mod state;
pub use self::state::MouseState;

/// The type tracking and managing the state of the mouse
pub struct MouseManager {
    /// Raw memory for setting mouse events
    mouse_events: [MouseEvent; MOUSE_EVENT_MAX],
    mouse_state: MouseState,
}

impl MouseManager {
    pub(super) fn new() -> MouseManager {
        MouseManager {
            mouse_events: [MouseEvent::NONE; MOUSE_EVENT_MAX],
            mouse_state: MouseState::default(),
        }
    }


    #[doc(hidden)]
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn mouse_events_ptr(&self) -> *const u32 {
        unsafe { ::std::mem::transmute::<*const MouseEvent, *const u32>(self.mouse_events.as_ptr()) }
    }

    /// Updates the mouse state given the list of events
    pub(super) fn pre_tick_process_mouse_state(&mut self) {
        self.mouse_state.start_update();

        for event in self.mouse_events.iter() {
            if event.event_type == MouseEventType::None {
                break;
            }

            self.mouse_state.update(event);
        }
    }
}

impl Deref for MouseManager {
    type Target = MouseState;

    fn deref(&self) -> &Self::Target {
        &self.mouse_state
    }
}
