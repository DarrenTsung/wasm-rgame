use graphics::Graphics;
use super::{
    Delegate,
    ApplicationContext,
    KeyManager,
    MouseManager,
    SpawnableDelegate,
    SpawnHandle,
};
use raii_counter::WeakCounter;

mod spawner;
pub use self::spawner::DelegateSpawner;

pub struct DelegateManager {
    /// Because the real DelegateManager is ticking the delegates,
    /// delegates are given access to a DelegateSpawner so that they
    /// can spawn / spawn_root other delegates
    spawner: DelegateSpawner,

    root_delegates: Vec<Box<Delegate>>,
    delegates: Vec<SpawnedDelegate>,
}

struct SpawnedDelegate {
    inner: Box<Delegate>,
    handles_counter: WeakCounter,
}

impl DelegateManager {
    pub fn new() -> DelegateManager {
        DelegateManager {
            spawner: DelegateSpawner::new(),
            root_delegates: Vec::new(),
            delegates: Vec::new(),
        }
    }

    pub fn as_spawner(&mut self) -> &mut DelegateSpawner {
        &mut self.spawner
    }

    /// Takes ownership of a Delegate, it is held onto for the
    /// lifetime of the DelegateManager (the entire Application lifetime)
    pub fn spawn_root<D: 'static + Delegate>(&mut self, delegate: D) {
        self.root_delegates.push(Box::new(delegate));
    }

    pub(in application) fn tick(
        &mut self,
        context: &mut ApplicationContext,
        key_manager: &KeyManager,
        mouse_manager: &MouseManager,
        graphics: &mut Graphics,
    ) {
        // if a delegate no longer has handles, they are dropped
        self.delegates.retain(|d| d.handles_counter.count() > 0);

        for mut delegate in self.root_delegates.iter_mut() {
            (*delegate).tick(context, key_manager, mouse_manager, &mut self.spawner, graphics);
        }

        for mut delegate in self.delegates.iter_mut() {
            (*delegate.inner).tick(context, key_manager, mouse_manager, &mut self.spawner, graphics);
        }

        // Collect delegates spawned by the delegate spawner
        for root_delegate in self.spawner.spawned_root_delegates() {
            self.root_delegates.push(root_delegate);
        }

        for delegate in self.spawner.spawned_delegates() {
            self.delegates.push(delegate);
        }
    }
}
