use std::ops::Deref;

// These must match mouse_listener.js
pub const MOUSE_EVENT_MAX : usize = 100;

bitflags! {
    pub(self) struct MouseButtons: u32 {
        const EMPTY      = 0b00000000;
        const PRIMARY    = 0b00000001;
        const SECONDARY  = 0b00000010;
    }
}

pub mod event;
pub use self::event::{MouseEvent, MouseEventType};

pub mod state;
pub use self::state::{MouseState};

/// The type tracking and managing the state of the mouse
pub struct MouseManager {
    /// Raw memory for setting mouse events
    mouse_events: [MouseEvent; MOUSE_EVENT_MAX],
    mouse_state: MouseState,
}

impl MouseManager {
    pub fn new() -> MouseManager {
        MouseManager {
            mouse_events: [MouseEvent::NONE; MOUSE_EVENT_MAX],
            mouse_state: MouseState::default(),
        }
    }

    /// Get pointer to the keys rect
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn mouse_events_ptr(&self) -> *const MouseEvent {
        self.mouse_events.as_ptr()
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
