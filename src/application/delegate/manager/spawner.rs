use raii_counter::Counter;
use std::vec::Drain;
use super::{Delegate, SpawnedDelegate, SpawnHandle, SpawnableDelegate};

/// Type used to spawn / spawn_root new delegates onto the Application
pub struct DelegateSpawner {
    spawned_delegates: Vec<SpawnedDelegate>,
    spawned_root_delegates: Vec<Box<Delegate>>,
}

impl DelegateSpawner {
    pub fn new() -> DelegateSpawner {
        DelegateSpawner {
            spawned_delegates: Vec::new(),
            spawned_root_delegates: Vec::new(),
        }
    }

    /// Takes ownership of a Delegate, returns a SpawnHandle containing the
    /// Handle type of the SpawnableDelegate
    ///
    /// The spawned object cannot be found by the object spawning it, therefore it
    /// should use the Handle returned to get information
    ///
    /// Dropping all instances of the SpawnHandle will cause the delegate to be dropped.
    pub fn spawn<D>(
        &mut self,
        spawnable_delegate: D
    ) -> SpawnHandle<D::Handle>
    where
        D: 'static + SpawnableDelegate
    {
        let handle = spawnable_delegate.handle();
        let handle_counter = Counter::new();

        self.spawned_delegates.push(SpawnedDelegate {
            inner: Box::new(spawnable_delegate),
            handles_counter: handle_counter.clone().downgrade(),
        });

        SpawnHandle {
            handle,
            _counter: handle_counter,
        }
    }

    /// Takes ownership of a Delegate, it is held for the
    /// lifetime of the entire Application
    pub fn spawn_root<D: 'static + Delegate>(&mut self, delegate: D) {
        self.spawned_root_delegates.push(Box::new(delegate));
    }

    pub(super) fn spawned_delegates(&mut self) -> Drain<SpawnedDelegate> {
        self.spawned_delegates.drain(0..)
    }

    pub(super) fn spawned_root_delegates(&mut self) -> Drain<Box<Delegate>> {
        self.spawned_root_delegates.drain(0..)
    }
}
