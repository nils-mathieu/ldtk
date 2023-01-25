use crate::c;
use crate::c::{Errno, Fork, Pid, Strs};

use sentinel::{sslice, SSlice};

/// Spawns a command and waits until it completes.
pub fn do_spawn(command: &SSlice<u8>) -> Result<(), Errno> {
    match Pid::fork()? {
        Fork::Parent { child } => {
            child.wait()?;
            Ok(())
        }
        Fork::Child => {
            let args = [
                Some(sslice!("/bin/sh")),
                Some(sslice!("-c")),
                Some(command),
                None,
            ];
            let args: Strs = SSlice::from_slice(&args).unwrap();

            // We can't really duplicate the environment of the current program because that would
            // also duplicate the `LD_PRELOAD` environment variable, which probably spawned us in
            // the first place. The only thing we really need here is the `PATH`, so we'll just
            // carry that to the child process.
            let env = [c::getenv(sslice!("PATH")), None];
            let env = SSlice::from_slice(&env).unwrap();

            c::execve(sslice!("/bin/sh"), args, env)?;
            Ok(()) // unreachable
        }
    }
}
