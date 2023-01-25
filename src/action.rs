use sentinel::{sslice, SSlice};

use crate::c;

/// Represents what the LDTK binary is able to do.
pub enum Action {
    /// Mute the hijacked program by making the `write` function always return 1 without doing
    /// anything.
    ///
    /// **LDTK_ACTION:** `mute`
    Mute,
}

impl Action {
    /// The environment variable in which an [`Action`] instance can be parsed.
    pub const ENV_ACTION: &'static SSlice<u8> = sslice!("LDTK_ACTION");

    /// Parses the `LDTK_ACTION` variable and creates an isntance of [`Action`].
    ///
    /// # Errors
    ///
    /// If the variable is not defined, or if it contains an unrecognized string, the function
    /// returns `None`.
    pub fn from_env() -> Option<Self> {
        match c::getenv(Self::ENV_ACTION)?.as_slice() {
            b"mute" => Some(Self::Mute),
            _ => None,
        }
    }
}

impl Default for Action {
    #[inline(always)]
    fn default() -> Self {
        Self::Mute
    }
}
