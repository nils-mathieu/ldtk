#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use action::Action;

mod action;
mod actions;
mod c;

/// Writes an error message to the standard error.
///
/// On release builds, this function does nothing.
fn print_error(err: &[u8]) {
    if cfg!(debug_assertions) {
        let _ = c::write(c::STDERR, b"ldtk: ");
        let _ = c::write(c::STDERR, err);
    }
}

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
/// call it before the hijacked program's normal `main` function.
#[link_section = ".init_array"]
#[used]
static ENTRY_POINT: extern "C" fn() = init;

/// The entry point of our library.
extern "C" fn init() {
    let result = match Action::from_env().unwrap_or_default() {
        Action::Mute => actions::do_mute(),
        Action::Spawn(cmd) => actions::do_spawn(cmd),
    };

    match result {
        Ok(()) => (),
        Err(err) => print_error(err.description().as_slice()),
    }
}
