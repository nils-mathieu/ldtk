use crate::c;
use crate::c::Errno;

/// The code of our new `write` function.
const NEW_WRITE: &[u8] = include_bytes!("mute_write");

/// Hijacks libc's `write` function and makes it return `1` inconditionally.
pub fn do_mute() -> Result<(), Errno> {
    // Get the address of the `write` function.
    let write_addr = libc::write as *mut u8;

    let page_size = c::sysconf_pagesize()?;
    let page_addr = (write_addr as usize & !page_size.wrapping_sub(1)) as *mut u8;

    // Change the protection of the page that holds the `write` function, ensuring that we are
    // allowed to modify it.
    // SAFETY:
    //  We are keeping the `EXEC` flag to ensure that the code still is executable (we might
    //  currently be executing code within that page).
    unsafe { c::mprotect(page_addr, page_size, c::Prot::EXEC | c::Prot::WRITE)? };

    // Write some code at the begining of the hijacked function, forcing it to return our own
    // code.
    // SAFETY:
    //  This is not really safe, or at least, I'm not sure it is. Let's hope that `NEW_WRITE` is
    //  small enough to fit in the original `write` function.
    unsafe { core::ptr::copy_nonoverlapping(NEW_WRITE.as_ptr(), write_addr, NEW_WRITE.len()) };

    // SAFETY:
    //  We are restoring the previous flags. Those were fine before the modification!
    let _ = unsafe { c::mprotect(page_addr, page_size, c::Prot::EXEC) };

    Ok(())
}
