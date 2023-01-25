# Changelog

## v0.1.1

- Added the `"spawn"` action, which simply spawns a command (using the `sh` shell) before handing
the control back to the original program.

## v0.1.0

- Added the `"mute"` action, which hijacks libc's `write` to make it return 1 all the time.
