#![no_std]

use sentinel::sslice;

mod c;

/// Handles a panic coming from our crate.
///
/// # Notes
///
/// This is a bug. This function should normally *never* be called.
#[panic_handler]
fn handle_panic(_info: &core::panic::PanicInfo) -> ! {
    // TODO:
    //  Print the error message in debug mode and nothing in release mode.
    c::abort();
}

/// This static variable will be added to the `.init_array` section, requesting the C runtime to
/// call it before the hijacked normal `main` function.
#[link_section = ".init_array"]
#[used]
static ENTRY_POINT: extern "C" fn() = init;

/// The entry point of our library.
extern "C" fn init() {
    let _ = c::puts(sslice!("Hello, World!"));
}
