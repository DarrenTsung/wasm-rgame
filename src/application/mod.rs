use graphics::Graphics;
use super::Canvas;

mod key;
pub use self::key::{KeyManager, key_codes};

mod mouse;
pub use self::mouse::{MouseState, MouseButton};
use self::mouse::MouseManager;

mod delegate;
pub use self::delegate::{DelegateSpawner, Delegate, SpawnableDelegate, SpawnHandle, SpawnHandles};
use self::delegate::DelegateManager;

mod context;
pub use self::context::ApplicationContext;

pub struct Application {
    context: ApplicationContext,
    key_manager: KeyManager,
    mouse_manager: MouseManager,
    delegate_manager: DelegateManager,
}

impl Application {
    /// Get the mutable reference to the DelegateSpawner. Used for spawning
    /// delegates at the EntryPoint of the application.
    pub fn as_spawner(&mut self) -> &mut DelegateSpawner {
        self.delegate_manager.as_spawner()
    }

    #[doc(hidden)]
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn new() -> Application {
        Application {
            context: ApplicationContext::new(),
            key_manager: KeyManager::new(),
            mouse_manager: MouseManager::new(),
            delegate_manager: DelegateManager::new(),
        }
    }

    #[doc(hidden)]
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn canvas_properties_ptr(&self) -> *const u32 { Canvas::instance().canvas_properties_ptr() }

    #[doc(hidden)]
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn keys_ptr(&self) -> *const u8 { self.key_manager.keys_ptr() }

    #[doc(hidden)]
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn mouse_events_ptr(&self) -> *const u32 { self.mouse_manager.mouse_events_ptr() }

    #[doc(hidden)]
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn tick(&mut self, graphics: &mut Graphics, delta_s: f64) {
        // Pre-delegates
        self.context.pre_tick(delta_s);
        self.mouse_manager.pre_tick_process_mouse_state();

        // Tick all the delegates spawn on this application
        self.delegate_manager.tick(&mut self.context, &self.key_manager, &self.mouse_manager, graphics);

        // Post-delegates
        self.key_manager.post_tick_update_key_states();
    }
}
