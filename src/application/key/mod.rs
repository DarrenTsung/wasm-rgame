pub mod key_codes;

// These must match key_listener.js
const KEY_CODE_MAX: usize = 300;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum KeyCodeState {
    None = 0,
    Down = 1,
    Held = 2,
    Up = 3,
}

/// The type managing the state of each KeyCode
pub struct KeyManager {
    /// Raw memory for javascript to write to, used to store the current
    /// state of the KeyCode (index)
    keys: [KeyCodeState; KEY_CODE_MAX],
}

impl KeyManager {
    pub(super) fn new() -> KeyManager {
        KeyManager {
            keys: [KeyCodeState::None; KEY_CODE_MAX],
        }
    }

    #[doc(hidden)]
    /// Get pointer to the keys rect
    /// WARNING: JS Exported Function - not intended for normal use
    pub fn keys_ptr(&self) -> *const u8 {
        unsafe { ::std::mem::transmute::<*const KeyCodeState, *const u8>(self.keys.as_ptr()) }
    }

    /// Transition key states as we only get KeyCodeState::Up && KeyCodeState::Down
    /// states set from the listener, it's up to us to transition them to KeyCodeState::None
    /// && KeyCodeState::Held at the end of the frame.
    pub(super) fn post_tick_update_key_states(&mut self) {
        for index in 0..KEY_CODE_MAX {
            let value = self.keys[index];
            match value {
                KeyCodeState::Up => self.keys[index] = KeyCodeState::None,
                KeyCodeState::Down => self.keys[index] = KeyCodeState::Held,
                _ => (),
            }
        }
    }

    /// Returns true if key was just pressed
    pub fn key_down(&self, key_code: usize) -> bool {
        self.key_state(key_code) == KeyCodeState::Down
    }

    /// Returns true if the key is pressed
    pub fn key(&self, key_code: usize) -> bool {
        match self.key_state(key_code) {
            KeyCodeState::Down | KeyCodeState::Held => true,
            _ => false,
        }
    }

    /// Returns true if key was just released
    pub fn key_up(&self, key_code: usize) -> bool {
        self.key_state(key_code) == KeyCodeState::Up
    }

    /// Returns the KeyCodeState for a given KeyCode
    fn key_state(&self, key_code: usize) -> KeyCodeState {
        self.keys[key_code]
    }
}
