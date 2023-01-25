//! This module provides safe wrappers to `libc`'s functions.

use core::cell::Cell;

use sentinel::SSlice;

/// An error which may occur whilst interacting with the operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Errno(libc::c_int);

impl Errno {
    /// Returns a pointer to the thread-local errno.
    ///
    /// This function is not intended to be used outside of the `Errno` class and mainly exists for
    /// correctness.
    fn cell() -> &'static Cell<libc::c_int> {
        // SAFETY:
        //  - `__errno_location` is always safe to call.
        //  - The pointer it returns can be safely mutated through a cell.
        unsafe { &*(libc::__errno_location() as *const Cell<libc::c_int>) }
    }

    /// Returns the calling thread's last error code.
    #[inline]
    pub fn last_error() -> Self {
        Self(Self::cell().get())
    }

    /// Sets the last error code.
    #[inline]
    pub fn make_last_error(self) {
        Self::cell().set(self.0)
    }
}

impl Errno {
    /// Indicates that no error occured.
    pub const SUCCESS: Self = Self(0);
}

/// Aborts the program's execution.
pub fn abort() -> ! {
    // SAFETY:
    //  This function is never unsafe to call.
    unsafe { libc::abort() };
}

/// Returns the value of an environment variable. If the variable does not exist, `None` is
/// returned.
pub fn getenv(s: &SSlice<u8>) -> Option<&'static SSlice<u8>> {
    // SAFETY:
    //  The invariants of `SSlice` ensure that the string is valid and properly null-terminated.
    let ret = unsafe { libc::getenv(s.as_ptr() as *const libc::c_char) };

    if ret.is_null() {
        None
    } else {
        // SAFETY:
        //  The `libc::getenv` function always either return a null pointer (which we already
        //  handled) or a valid null-terminated string.
        Some(unsafe { SSlice::from_ptr(ret as *const u8) })
    }
}

/// Calls the `sysconf` function and properly handles errors.
fn sysconf(name: libc::c_int) -> Result<usize, Errno> {
    Errno::SUCCESS.make_last_error();

    let size = unsafe { libc::sysconf(name) };
    if size == -1 {
        let errno = Errno::last_error();
        if errno != Errno::SUCCESS {
            return Err(errno);
        }
    }

    Ok(size as usize)
}

/// Returns the size of a memory page on the current system.
pub fn sysconf_pagesize() -> Result<usize, Errno> {
    sysconf(libc::_SC_PAGESIZE)
}

bitflags::bitflags! {
    /// The protection flags of a memory page.
    pub struct Prot: libc::c_int {
        /// No flags.
        const NONE = libc::PROT_NONE;

        /// The page can be read from.
        const READ = libc::PROT_READ;

        /// The page can be written to.
        const WRITE = libc::PROT_WRITE;

        /// The page can be executed.
        const EXEC = libc::PROT_EXEC;
    }
}

/// Modifies the protection flags of a memory page.
///
/// # Safety
///
/// This function can change assumptions safe code makes about memory pages.
pub unsafe fn mprotect(addr: *mut u8, len: usize, flags: Prot) -> Result<(), Errno> {
    let ret = unsafe { libc::mprotect(addr as *mut libc::c_void, len, flags.bits()) };
    if ret == -1 {
        Err(Errno::last_error())
    } else {
        Ok(())
    }
}
