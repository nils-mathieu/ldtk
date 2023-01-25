//! This module provides safe wrappers to `libc`'s functions.

use sentinel::SSlice;

/// An error which may occur whilst interacting with the operating system.
pub struct Errno(libc::c_int);

impl Errno {
    /// Returns the calling thread's last error code.
    #[inline]
    pub fn last_error() -> Self {
        Self(unsafe { *libc::__errno_location() })
    }
}

/// Aborts the program's execution.
pub fn abort() -> ! {
    // SAFETY:
    //  This function is never unsafe to call.
    unsafe { libc::abort() };
}

/// Prints a message to the standard output, adding automatically a new-line `'\n'` character.
pub fn puts(s: &SSlice<u8>) -> Result<(), Errno> {
    let ret = unsafe { libc::puts(s.as_ptr() as *const libc::c_char) };

    if ret == libc::EOF {
        Err(Errno::last_error())
    } else {
        Ok(())
    }
}
