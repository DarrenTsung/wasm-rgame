pub mod drawables;
pub use self::drawables::{DrawRect, DrawActionColor};

// NOTE: this must match in render.js
const MAX_DRAW_ARRAY_SIZE : usize = 1000;

pub type Color = [u8; 4];

pub struct Graphics {
    ordering: usize,
    draw_rects: [DrawRect; MAX_DRAW_ARRAY_SIZE],
    draw_rects_index: usize,
    draw_action_colors: [DrawActionColor; MAX_DRAW_ARRAY_SIZE],
    draw_action_colors_index: usize,
}

impl Graphics {
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn new() -> Graphics {
        Graphics {
            ordering: 0,
            draw_rects: [DrawRect::EMPTY; MAX_DRAW_ARRAY_SIZE],
            draw_rects_index: 0,
            draw_action_colors: [DrawActionColor::EMPTY; MAX_DRAW_ARRAY_SIZE],
            draw_action_colors_index: 0,
        }
    }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn draw_rects_ptr(&self) -> *const DrawRect { self.draw_rects.as_ptr() }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn draw_rects_len(&self) -> usize { self.draw_rects_index }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn draw_action_colors_ptr(&self) -> *const DrawActionColor { self.draw_action_colors.as_ptr() }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn draw_action_colors_len(&self) -> usize { self.draw_action_colors_index }

    /// Clearing Graphics for the next frame
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn reset(&mut self) {
        self.ordering = 0;
        self.draw_rects_index = 0;
        self.draw_action_colors_index = 0;
    }

    pub fn draw_rect(&mut self, pos_x: i32, pos_y: i32, width: u16, height: u16, color: Color) {
        assert!(self.draw_rects_index < MAX_DRAW_ARRAY_SIZE);

        self.draw_rects[self.draw_rects_index] = DrawRect {
            ordering: self.ordering as i32,
            pos_x,
            pos_y,
            width: width as i32,
            height: height as i32,
        };
        self.draw_rects_index += 1;

        self.set_color(color);

        self.ordering += 1;
    }

    /// Sets the color if it is different from the last draw color change
    fn set_color(&mut self, color: Color) {
        // if no color is set yet or the last color is not the same as this color
        if self.draw_action_colors_index == 0 ||
            !self.draw_action_colors[self.draw_action_colors_index - 1].same_color(color)
        {
            assert!(self.draw_action_colors_index < MAX_DRAW_ARRAY_SIZE);

            self.draw_action_colors[self.draw_action_colors_index] = DrawActionColor {
                ordering: self.ordering as u8,
                color_r: color[0],
                color_g: color[1],
                color_b: color[2],
                alpha: color[3],
            };
            self.draw_action_colors_index += 1;
        }

    }
}
