//! The required types when implementing a delegate.
//! You can glob import this to save time with:
//! `use wasm_rgame::delegate_prelude::*;`
pub use super::{
    Delegate,
    SpawnableDelegate,
    SpawnHandle,
    SpawnHandles,

    ApplicationContext,

    KeyManager,

    MouseState,
    MouseButton,

    DelegateSpawner,

    Graphics,
};
