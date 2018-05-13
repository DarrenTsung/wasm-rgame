// NOTE: This must match in graphics.js
const _DRAW_RECT_SIZE : usize = 5;
const _DRAW_ACTION_COLOR_SIZE : usize = 5;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawRect {
    pub ordering: i32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawActionColor {
    pub ordering: u8,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub alpha: u8,
}

impl DrawRect {
    pub const EMPTY: DrawRect = DrawRect {
        ordering: 0, pos_x: 0, pos_y: 0, width: 0, height: 0,
    };
}

impl DrawActionColor {
    pub const EMPTY: DrawActionColor = DrawActionColor {
        ordering: 0, color_r: 0, color_g: 0, color_b: 0, alpha: 0,
    };

    pub fn same_color(&self, color: [u8; 4]) -> bool {
        self.color_r == color[0] &&
            self.color_g == color[1] &&
            self.color_b == color[2] &&
            self.alpha == color[3]
    }
}
