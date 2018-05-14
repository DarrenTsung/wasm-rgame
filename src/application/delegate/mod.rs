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
    /// Optional method to do things on spawn
    fn on_spawn(
        &mut self,
        _delegate_spawner: &mut DelegateSpawner,
    ) {}

    /// The order in which tick is called is opaque, do not depend on it.
    fn tick(
        &mut self,
        context: &mut ApplicationContext,
        key_manager: &KeyManager,
        mouse_manager: &MouseManager,
        delegate_spawner: &mut DelegateSpawner,
    );

    fn render(&self, graphics: &mut Graphics);

    /// Objects that have a higher render_order will be rendered
    /// on top of objects with a lower order
    fn render_order(&self) -> i32 { 0 }
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
