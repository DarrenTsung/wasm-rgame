use super::{
    Delegate,
    ApplicationContext,
    KeyManager,
    MouseState,
    SpawnableDelegate,
    SpawnHandle,
};

use dmsort;
use graphics::Graphics;
use raii_counter::WeakCounter;
use std::cmp;

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
    pub(in application) fn as_spawner(&mut self) -> &mut DelegateSpawner {
        &mut self.spawner
    }

    pub(in application) fn new() -> DelegateManager {
        DelegateManager {
            spawner: DelegateSpawner::new(),
            root_delegates: Vec::new(),
            delegates: Vec::new(),
        }
    }

    pub(in application) fn tick(
        &mut self,
        context: &mut ApplicationContext,
        key_manager: &KeyManager,
        mouse_state: &MouseState,
        graphics: &mut Graphics,
    ) {
        // if a spawned delegate no longer has handles, they are dropped
        self.delegates.retain(|d| d.handles_counter.count() > 0);

        // go through the delegates in render_order() ordering and tick / render
        {
            let mut root_delegates_index = 0;
            let mut delegates_index = 0;

            loop {
                // no more elements in root_delegates
                if root_delegates_index >= self.root_delegates.len() {
                    for index in delegates_index..self.delegates.len() {
                        self.delegates[index].inner.tick(context, key_manager, mouse_state, &mut self.spawner);
                        self.delegates[index].inner.render(graphics);
                    }

                    break;
                } else if delegates_index >= self.delegates.len() {
                    for index in root_delegates_index..self.root_delegates.len() {
                        self.root_delegates[index].tick(context, key_manager, mouse_state, &mut self.spawner);
                        self.root_delegates[index].render(graphics);
                    }

                    break;
                } else {
                    let order = cmp::min(
                        self.root_delegates[root_delegates_index].render_order(),
                        self.delegates[delegates_index].inner.render_order(),
                    );

                    for index in delegates_index..self.delegates.len() {
                        if self.delegates[index].inner.render_order() != order {
                            break;
                        }

                        self.delegates[index].inner.tick(context, key_manager, mouse_state, &mut self.spawner);
                        self.delegates[index].inner.render(graphics);

                        delegates_index += 1;
                    }

                    for index in root_delegates_index..self.root_delegates.len() {
                        if self.root_delegates[index].render_order() != order {
                            break;
                        }

                        self.root_delegates[index].tick(context, key_manager, mouse_state, &mut self.spawner);
                        self.root_delegates[index].render(graphics);

                        root_delegates_index += 1;
                    }
                }
            }
        }

        // Collect delegates spawned by the delegate spawner
        self.collect_from_spawner();
    }

    fn collect_from_spawner(&mut self) {
        let prev_root_len = self.root_delegates.len();
        let prev_delegates_len = self.delegates.len();

        for root_delegate in self.spawner.spawned_root_delegates() {
            self.root_delegates.push(root_delegate);
        }

        for delegate in self.spawner.spawned_delegates() {
            self.delegates.push(delegate);
        }

        // Only run the sort if new things were added
        if prev_root_len != self.root_delegates.len() {
            // dmsort is supposed to be better on arrays that are almost sorted
            // TODO: benchmark and compare if this is an issue
            dmsort::sort_by_key(&mut self.root_delegates, |d| d.render_order());
        }

        if prev_delegates_len != self.delegates.len() {
            dmsort::sort_by_key(&mut self.delegates, |d| d.inner.render_order());
        }
    }
}
