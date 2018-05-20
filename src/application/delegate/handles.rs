use super::SpawnHandle;

use std::any::Any;

/// Helper type to compact holding onto multiple SpawnHandle<_>s
/// with a single reference (at the cost of boxing). Given that
/// spawning should be relatively not often, the allocation cost
/// should be fine for the ergonomical trade-off.
pub struct SpawnHandles {
    handles: Vec<Box<Any>>,
}

impl SpawnHandles {
    pub fn new() -> SpawnHandles {
        SpawnHandles { handles: Vec::new() }
    }

    pub fn with<T: 'static>(mut self, handle: SpawnHandle<T>) -> Self {
        self.handles.push(Box::new(handle));
        self
    }
}
