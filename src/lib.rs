#![no_std]

use action::Action;

mod action;
mod actions;
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
    match Action::from_env().unwrap_or_default() {
        Action::Mute => {
            let _ = actions::do_mute();
        }
    }
}
