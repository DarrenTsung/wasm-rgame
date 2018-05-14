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
    MouseEvent,
    MouseState,
    MouseButton,
    MouseButtonState,
};

pub use canvas::{CANVAS, Canvas};

pub use graphics::{
    Graphics,
    DrawRect,
    DrawActionColor,
    Color,
};
