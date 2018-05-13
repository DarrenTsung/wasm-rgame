// These must match canvas_properties.js
const CANVAS_WIDTH_INDEX: usize = 0;
const CANVAS_HEIGHT_INDEX: usize = 1;

pub struct ApplicationContext {
    /// Total time in seconds that the application has been running
    total_s: f64,
    /// Delta time since last frame
    delta_s: f64,

    /// Raw memory for accessing the canvas, used by the Js to react to
    /// configuration changes without having to call between Js <-> Rust
    canvas_properties: [u32; 2],
}

impl ApplicationContext {
    pub fn new(canvas_width: u32, canvas_height: u32) -> ApplicationContext {
        let mut canvas_properties = [0; 2];
        canvas_properties[CANVAS_WIDTH_INDEX] = canvas_width;
        canvas_properties[CANVAS_HEIGHT_INDEX] = canvas_height;

        ApplicationContext {
            total_s: 0.0,
            delta_s: 0.0,

            canvas_properties,
        }
    }

    /// Get pointer to the canvas properties rect
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn canvas_properties_ptr(&self) -> *const u32 {
        self.canvas_properties.as_ptr()
    }

    pub(super) fn pre_tick(&mut self, delta_s: f64) {
        self.delta_s = delta_s;
        self.total_s += delta_s;
    }

    pub fn total_s(&self) -> f64 { self.total_s }
    pub fn canvas_width(&self) -> u32 { self.canvas_properties[CANVAS_WIDTH_INDEX] }
    pub fn set_canvas_width(&mut self, val: u32) { self.canvas_properties[CANVAS_WIDTH_INDEX] = val; }
    pub fn canvas_height(&self) -> u32 { self.canvas_properties[CANVAS_HEIGHT_INDEX] }
    pub fn set_canvas_height(&mut self, val: u32) { self.canvas_properties[CANVAS_HEIGHT_INDEX] = val; }
}
