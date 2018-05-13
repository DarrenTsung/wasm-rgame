use super::MouseButton;

// This must match in mouse_listener.js
const _MOUSE_EVENT_SIZE : usize = 4;

#[derive(Clone, Copy)]
pub struct MouseEvent {
    pub(crate) event_type: MouseEventType,
    pub(super) button: MouseButton,
    pub(super) pos_mx: u32,
    pub(super) pos_my: u32,
}

impl MouseEvent {
    pub const NONE : MouseEvent = MouseEvent {
        event_type: MouseEventType::None,
        button: MouseButton::Left,
        pos_mx: 0,
        pos_my: 0,
    };
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
#[allow(dead_code)] // This is set from the javascript side
pub enum MouseEventType {
    None = 0,
    Down = 1,
    Up = 2,
    Move = 3,
    Over = 4,
    Out = 5,
}
