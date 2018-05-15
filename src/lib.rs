//! # wasm-rgame
//! A Rust framework for building browser games with WASM.
//!
//! This framework was inspired by the [Game of Life Rust + WASM Tutorial](https://rust-lang-nursery.github.io/rust-wasm/game-of-life/introduction.html). You can dive right in and start writing a wasm-rgame application with the [tutorial](TODO link tutorial)!
//!
//! ### How It Works
//! This framework abstracts away writing custom HTML/Javascript and provides Rust types that interface with the Javascript bundled in the  [wasm-rgame-js](https://github.com/DarrenTsung/wasm-rgame-js) respository.
//!
//! The Rust API provides types for:
//! * Keyboard events
//! * Mouse position and events
//! * Rendering API to a 2D canvas
//! * Spawning new objects in the Application
//!
//! Also, a build tool ([wasm-rgame-tools](https://github.com/DarrenTsung/wasm-rgame-tools)) is provided that builds the project (targeting wasm32-unknown-unknown), runs wasm-bindgen, and bundles together the generated WASM binaries and Javascript/HTML.
//!
//! ### Goals
//! This project is mainly experimental, but has been used to make non-trivial applications (see [wrg-snake](https://github.com/DarrenTsung/wrg-snake)).
//!
//! * Users only need to write Rust.
//! * Minimal calls between Rust / Javascript.
//!
//! ### Example API Usage
//! These examples are taken from [wrg-snake](https://github.com/DarrenTsung/wrg-snake). Also note that these examples can't be run stand-alone as they require the Javascript/HTML framework.
//!
//! Rendering a transparent overlay:
//! ```rust,ignore
//! use wasm_rgame::{Delegate, Graphics};
//!
//! impl Delegate for MyObject {
//!     fn tick(..) {}
//!
//!     fn render(&self, graphics: &mut Graphics) {
//!         // draw a transparent overlay over the game
//!         graphics.draw_rect(
//!             0.0,
//!             0.0,
//!             CANVAS.width() as f32,
//!             CANVAS.height() as f32,
//!             [255, 255, 255, 150]
//!         );
//!     }
//! }
//! ```
//!
//! Checking keyboard input:
//! ```rust,ignore
//! use wasm_rgame::{KeyManager, key_codes};
//!
//! pub fn store_direction_change(&mut self, key_manager: &KeyManager) {
//!     // Don't let direction change if already going opposite direction
//!     let wanted_direction = if key_manager.key_down(key_codes::W) {
//!         Direction::Up
//!     } else if key_manager.key_down(key_codes::D) {
//!         Direction::Right
//!     } else if key_manager.key_down(key_codes::S) {
//!         Direction::Down
//!     } else if key_manager.key_down(key_codes::A) {
//!         Direction::Left
//!     } else {
//!         return;
//!     };
//!
//!     ...
//! }
//! ```

#[macro_use] extern crate lazy_static;

extern crate dmsort;
extern crate raii_counter;

mod application;
mod canvas;
mod graphics;
pub mod delegate_prelude;

pub use application::{
    Application,
    ApplicationContext,

    DelegateSpawner,
    Delegate,
    SpawnableDelegate,
    SpawnHandle,

    KeyManager,
    key_codes,
    KeyCodeState,

    MouseManager,
    MouseState,
    MouseButton,
    MouseButtonState,
};

pub use canvas::{CANVAS, Canvas};

pub use graphics::{
    Graphics,
    Color,
};
