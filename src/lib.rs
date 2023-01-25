#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use action::Action;
use c::Fd;

mod action;
mod actions;
mod c;

/// Writes an error message to the standard error.
///
/// On release builds, this function does nothing.
fn print_error(err: &[u8]) {
    if cfg!(debug_assertions) {
        let _ = Fd::STDERR.write(b"ldtk: ");
        let _ = Fd::STDERR.write(err);
    }
}

/// Handles a panic coming from our crate.
///
/// # Notes
///
/// This is a bug. This function should normally *never* be called.
#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        print_error(msg.as_bytes());
    }

    c::abort();
}

/// This static variable will be added to the `.init_array` section, requesting the C runtime to
/// call it before the hijacked program's normal `main` function.
#[link_section = ".init_array"]
#[used]
static ENTRY_POINT: extern "C" fn() = init;

/// The entry point of our library.
extern "C" fn init() {
    let action = match Action::from_env() {
        Some(action) => action,
        None => return,
    };

    let result = match action {
        Action::Mute => actions::do_mute(),
        Action::Spawn(cmd) => actions::do_spawn(cmd),
    };

    match result {
        Ok(()) => (),
        Err(err) => print_error(err.description().as_slice()),
    }
}
