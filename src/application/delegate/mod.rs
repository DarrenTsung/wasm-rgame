use graphics::Graphics;
use raii_counter::Counter;
use std::ops::{Deref, DerefMut};
use super::{ApplicationContext, KeyManager, MouseManager};

mod manager;
pub use self::manager::{DelegateManager, DelegateSpawner};

/// Trait for defining an interface for objects receiving updates from the application
///
/// The order of objects receiving updates is opaque and subject to change.
pub trait Delegate {
    fn tick(
        &mut self,
        context: &mut ApplicationContext,
        key_manager: &KeyManager,
        mouse_manager: &MouseManager,
        delegate_spawner: &mut DelegateSpawner,
        graphics: &mut Graphics,
    );
}

/// Trait for delegates that can be "spawned" on an DelegateManager
/// This trait defines the method of communciation with the spawned object
/// as ownership is passed to the DelegateManager
pub trait SpawnableDelegate : Delegate {
    type Handle: Clone;

    fn handle(&self) -> Self::Handle;
}

#[derive(Clone)]
pub struct SpawnHandle<H> {
    handle: H,
    _counter: Counter,
}

impl<H> Deref for SpawnHandle<H> {
    type Target = H;

    fn deref(&self) -> &H {
        &self.handle
    }
}

impl<H> DerefMut for SpawnHandle<H> {
    fn deref_mut(&mut self) -> &mut H {
        &mut self.handle
    }
}
