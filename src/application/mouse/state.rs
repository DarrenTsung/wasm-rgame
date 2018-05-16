use super::{MouseButton, MouseEvent, MouseEventType};

/// The type that holds the state of the Mouse. 
pub struct MouseState {
    pub prev_on_screen: bool,
    pub on_screen: bool,

    pub prev_x: f32,
    pub prev_y: f32,

    pub pos_x: f32,
    pub pos_y: f32,

    button_states: [MouseButtonState; 4],
}

#[derive(Clone, Copy, PartialEq)]
enum MouseButtonState {
    None,
    Down,
    Held,
    Up,
}

impl MouseState {
    /// Returns true if the MouseButton was just pressed.
    pub fn button_down(&self, button: MouseButton) -> bool {
        self.button_state(button) == MouseButtonState::Down
    }

    /// Returns true every frame the MouseButton is pressed. This includes
    /// the frame the MouseButton was just pressed.
    pub fn button_pressed(&self, button: MouseButton) -> bool {
        match self.button_state(button) {
            MouseButtonState::Down | MouseButtonState::Held => true,
            _ => false,
        }
    }

    /// Returns true if the MouseButton was just released.
    pub fn button_up(&self, button: MouseButton) -> bool {
        self.button_state(button) == MouseButtonState::Down
    }

    pub(super) fn default() -> MouseState {
        MouseState {
            prev_on_screen: false,
            on_screen: false,
            button_states: [MouseButtonState::None; 4],
            prev_x: 0.0,
            prev_y: 0.0,
            pos_x: 0.0,
            pos_y: 0.0,
        }
    }

    fn button_state(&self, button: MouseButton) -> MouseButtonState {
        self.button_states[button as usize]
    }

    pub(super) fn update(&mut self, event: &MouseEvent) {
        match event.event_type {
            MouseEventType::None => unreachable!(),
            MouseEventType::Down => {
                self.button_states[event.button as usize] = MouseButtonState::Down;
            },
            MouseEventType::Up => {
                self.button_states[event.button as usize] = MouseButtonState::Up;
            },
            MouseEventType::Move => (),
            MouseEventType::Over => {
                self.on_screen = true;
            },
            MouseEventType::Out => {
                self.on_screen = false;
            },
        }
        self.update_pos_with_event(event);
    }

    pub(super) fn start_update(&mut self) {
        self.prev_on_screen = self.on_screen;

        for state in self.button_states.iter_mut() {
            if *state == MouseButtonState::Down {
                *state = MouseButtonState::Held;
            } else if *state == MouseButtonState::Up {
                *state = MouseButtonState::None;
            }
        }
    }

    fn update_pos_with_event(&mut self, event: &MouseEvent) {
        self.prev_x = self.pos_x;
        self.prev_y = self.pos_y;

        self.pos_x = event.pos_mx as f32 / 1000.0;
        self.pos_y = event.pos_my as f32 / 1000.0;
    }
}
