use super::{MouseButtons, MouseEvent, MouseEventType};

pub struct MouseState {
    pub prev_on_screen: bool,
    pub on_screen: bool,

    pub button_state: MouseButtonState,

    pub prev_x: f32,
    pub prev_y: f32,

    pub pos_x: f32,
    pub pos_y: f32,
}

pub struct MouseButtonState {
    buttons_clicked: Vec<MouseButtons>,
}

impl MouseState {
    pub fn default() -> MouseState {
        MouseState {
            prev_on_screen: false,
            on_screen: false,
            button_state: MouseButtonState::new(),
            prev_x: 0.0,
            prev_y: 0.0,
            pos_x: 0.0,
            pos_y: 0.0,
        }
    }

    pub(super) fn update(&mut self, event: &MouseEvent) {
        match event.event_type {
            MouseEventType::None => (),
            MouseEventType::Click => {
                self.button_state.process_click_event(event);
            },
            MouseEventType::Move => {
                self.prev_x = self.pos_x;
                self.prev_y = self.pos_y;

                self.pos_x = event.pos_mx as f32 / 1000.0;
                self.pos_y = event.pos_my as f32 / 1000.0;
            },
            MouseEventType::Over => {
                self.on_screen = true;
            },
            MouseEventType::Out => {
                self.on_screen = false;
            },
        }
    }

    pub(super) fn start_update(&mut self) {
        self.prev_on_screen = self.on_screen;
        self.button_state.reset();
    }
}

impl MouseButtonState {
    pub fn new() -> MouseButtonState {
        MouseButtonState {
            buttons_clicked: Vec::new(),
        }
    }

    pub fn clicked_primary(&self) -> bool {
        self.buttons_clicked.iter().any(|buttons| buttons.contains(MouseButtons::PRIMARY))
    }

    fn reset(&mut self) {
        self.buttons_clicked.clear();
    }

    fn process_click_event(&mut self, event: &MouseEvent) {
        debug_assert!(event.event_type == MouseEventType::Click);

        self.buttons_clicked.push(event.buttons);
    }
}
