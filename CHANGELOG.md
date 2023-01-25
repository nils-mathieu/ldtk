# Changelog

## v0.1.3

- The `"mute"` action did not mute the standard error output. This is now fixed!

## v0.1.2

- The `"mute"` action is no longer the default. When nothing is specified in `LDTK_ACTION`, the
library does nothing.

- The `"mute"` action now preserves the hijacked program's ability to write to files. Only the
standard output and standard error output are removed.

## v0.1.1

- Added the `"spawn"` action, which simply spawns a command (using the `sh` shell) before handing
the control back to the original program.

## v0.1.0

- Added the `"mute"` action, which hijacks libc's `write` to make it return 1 all the time.
