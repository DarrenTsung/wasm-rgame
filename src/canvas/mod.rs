use std::sync::{Arc, RwLock};

// These must match canvas_properties.js
const CANVAS_WIDTH_INDEX: usize = 0;
const CANVAS_HEIGHT_INDEX: usize = 1;

lazy_static! {
    static ref CANVAS: Canvas = Canvas {
        inner: Arc::new(RwLock::new(CanvasInner {
            canvas_properties: [0; 2],
        })),
    };
}

pub struct Canvas {
    inner: Arc<RwLock<CanvasInner>>,
}

struct CanvasInner {
    /// Raw memory for accessing the canvas, used by the Js to react to
    /// configuration changes without having to call between Js <-> Rust.
    canvas_properties: [u32; 2],
}

impl Canvas {
    /// Gets the Canvas instance for the application.
    /// At the moment only one Canvas per application is supported.
    pub fn instance() -> &'static Canvas {
        &CANVAS
    }

    /// Get pointer to the canvas properties rect
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn canvas_properties_ptr(&self) -> *const u32 {
        self.inner.read().unwrap_or_else(|e| e.into_inner()).canvas_properties.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.inner.read().unwrap_or_else(|e| e.into_inner()).canvas_properties[CANVAS_WIDTH_INDEX]
    }

    pub fn height(&self) -> u32 {
        self.inner.read().unwrap_or_else(|e| e.into_inner()).canvas_properties[CANVAS_HEIGHT_INDEX]
    }

    pub fn set_width(&self, val: u32) {
        self.inner.write().unwrap_or_else(|e| e.into_inner()).canvas_properties[CANVAS_WIDTH_INDEX] = val;
    }

    pub fn set_height(&self, val: u32) {
        self.inner.write().unwrap_or_else(|e| e.into_inner()).canvas_properties[CANVAS_HEIGHT_INDEX] = val;
    }
}
