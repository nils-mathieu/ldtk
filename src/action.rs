use sentinel::{sslice, SSlice};

use crate::c;

/// Represents what the LDTK binary is able to do.
pub enum Action<'a> {
    /// Mute the hijacked program by making the `write` function always return 1 without doing
    /// anything.
    ///
    /// **LDTK_ACTION:** `mute`
    Mute,
    /// Spawns another process, waits until it completes, then continue the execution as if nothing
    /// happened.
    Spawn(&'a SSlice<u8>),
}

impl<'a> Action<'a> {
    /// The environment variable in which an [`Action`] instance can be parsed.
    pub const ENV_ACTION: &SSlice<u8> = sslice!("LDTK_ACTION");

    /// The environment variable used to store the spawned command (used by [`Action::Spawn`]).
    pub const ENV_SPAWN: &SSlice<u8> = sslice!("LDTK_SPAWN");

    /// The default command executed by [`Action::Spawn`].
    pub const DEFAULT_SPAWN: &SSlice<u8> = sslice!("echo \"i'm in ur address space\"");

    /// Parses the `LDTK_ACTION` variable and creates an isntance of [`Action`].
    ///
    /// # Errors
    ///
    /// If the variable is not defined, or if it contains an unrecognized string, the function
    /// returns `None`.
    pub fn from_env() -> Option<Self> {
        match c::getenv(Self::ENV_ACTION)?.as_slice() {
            b"mute" => Some(Self::Mute),
            b"spawn" => {
                let command = c::getenv(Self::ENV_SPAWN).unwrap_or(Self::DEFAULT_SPAWN);
                Some(Self::Spawn(command))
            }
            _ => None,
        }
    }
}

impl<'a> Default for Action<'a> {
    #[inline(always)]
    fn default() -> Self {
        Self::Mute
    }
}
