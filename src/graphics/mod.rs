use std::mem;

mod drawables;
use self::drawables::{DrawRect, StringProperties, DrawActionColor, DrawRotation};

// NOTE: this must match in render.js
const MAX_DRAW_ARRAY_SIZE : usize = 500;
const MAX_STRING_ARRAY_SIZE : usize = 1_000;

pub type Color = [u8; 4];

pub struct Graphics {
    ordering: usize,
    draw_rects: [DrawRect; MAX_DRAW_ARRAY_SIZE],
    draw_rects_index: usize,
    draw_action_colors: [DrawActionColor; MAX_DRAW_ARRAY_SIZE],
    draw_action_colors_index: usize,
    strings: [u8; MAX_STRING_ARRAY_SIZE],
    strings_index: usize,
    string_properties: [StringProperties; MAX_DRAW_ARRAY_SIZE],
    string_properties_index: usize,
    rotations: [DrawRotation; MAX_DRAW_ARRAY_SIZE],
    rotations_index: usize,
}

impl Graphics {
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn new() -> Graphics {
        Graphics {
            ordering: 1,
            draw_rects: [DrawRect::EMPTY; MAX_DRAW_ARRAY_SIZE],
            draw_rects_index: 0,
            draw_action_colors: [DrawActionColor::EMPTY; MAX_DRAW_ARRAY_SIZE],
            draw_action_colors_index: 0,
            strings: [0; MAX_STRING_ARRAY_SIZE],
            strings_index: 0,
            string_properties: [StringProperties::EMPTY; MAX_DRAW_ARRAY_SIZE],
            string_properties_index: 0,
            rotations: [DrawRotation::EMPTY; MAX_DRAW_ARRAY_SIZE],
            rotations_index: 0,
        }
    }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn draw_rects_ptr(&self) -> *const f32 { unsafe { mem::transmute::<*const DrawRect, *const f32>(self.draw_rects.as_ptr()) } }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn draw_rects_len(&self) -> usize { self.draw_rects_index }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn draw_action_colors_ptr(&self) -> *const u8 { unsafe { mem::transmute::<*const DrawActionColor, *const u8>(self.draw_action_colors.as_ptr()) } }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn draw_action_colors_len(&self) -> usize { self.draw_action_colors_index }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn strings_ptr(&self) -> *const u8 { self.strings.as_ptr() }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn string_properties_ptr(&self) -> *const f32 { unsafe { mem::transmute::<*const StringProperties, *const f32>(self.string_properties.as_ptr()) } }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn string_properties_len(&self) -> usize { self.string_properties_index }

    /// WARNING: JS Exported Function - not intended for normal use
    pub fn rotations_ptr(&self) -> *const f32 { unsafe { mem::transmute::<*const DrawRotation, *const f32>(self.rotations.as_ptr()) } }

    /// Clearing Graphics for the next frame
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn reset(&mut self) {
        self.ordering = 1;
        self.draw_rects[0] = DrawRect::EMPTY;
        self.draw_rects_index = 0;
        self.draw_action_colors[0] = DrawActionColor::EMPTY;
        self.draw_action_colors_index = 0;
        self.string_properties[0] = StringProperties::EMPTY;
        self.string_properties_index = 0;
        self.strings_index = 0;
        self.rotations[0] = DrawRotation::EMPTY;
        self.rotations_index = 0;
    }

    pub fn draw_rect(&mut self, pos_x: f32, pos_y: f32, width: f32, height: f32, color: Color, angle: f32) {
        assert!(self.draw_rects_index < MAX_DRAW_ARRAY_SIZE);

        self.draw_rects[self.draw_rects_index] = DrawRect {
            ordering: self.ordering as f32,
            pos_x,
            pos_y,
            width,
            height,
        };
        self.draw_rects_index += 1;

        self.set_color(color);
        self.set_angle(angle);
        self.ordering += 1;
    }

    pub fn draw_string(&mut self, s: &str, pos_x: f32, pos_y: f32, font_size: f32, color: Color) {
        for byte in s.as_bytes() {
            self.strings[self.strings_index] = *byte;
            self.strings_index += 1;
        }
        // terminate the string with 0
        self.strings[self.strings_index] = 0;
        self.strings_index += 1;

        self.string_properties[self.string_properties_index] = StringProperties {
            ordering: self.ordering as f32,
            pos_x,
            pos_y,
            font_size,
        };
        self.string_properties_index += 1;

        self.set_color(color);
        // TODO (darren): see Javascript, rotation does not
        // work properly for strings right now
        // self.set_angle(angle);
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

    fn set_angle(&mut self, angle: f32) {
        if self.rotations_index == 0 ||
            self.rotations[self.rotations_index - 1].angle != angle
        {
            assert!(self.rotations_index < MAX_DRAW_ARRAY_SIZE);

            self.rotations[self.rotations_index] = DrawRotation {
                ordering: self.ordering as f32,
                angle,
            };
            self.rotations_index += 1;
            self.rotations[self.rotations_index] = DrawRotation::EMPTY;
        }
    }
}
