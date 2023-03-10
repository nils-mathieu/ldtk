# Information

A simple tool to hijack processes when they start.

LDTK expects to be loaded as a dynamic library using the `LD_PRELOAD` environment variable within
a shell (such as Bash or Zsh). In its initialization code, the library will mess with the hijacked
program's memory and functions.

# Tutorial

## Installation

This program is only supported on Linux.

### Using A Pre-Built Binary

If you are using Linux and a x86_64 CPU, you can use the pre-built binary available in github
[releases](https://github.com/nils-mathieu/ldtk/releases).

### Building From Source

To build LDTK from source, you'll need `cargo` (and a recent-enough version of the Rust toolchain),
as well as `nasm` to compile `.asm` files.

```bash
git clone git@github.com:nils-mathieu/ldtk.git
cd  ldtk
cargo build --release
```

The output binary will be located in your default target location, usually `target/release/libldtk.so`.

## Usage

Either you want the tool to be loaded in every program you start, in which case you can export the
`LD_PRELOAD` environment variable.

```bash
export LD_PRELOAD=path/to/libldtk.so
```

Or you can try it on a single process, using this syntax (works in sh-like shells):

```bash
LD_PRELOAD=path/to/libldtk.so cat Cargo.toml
```

## Configuration

LDTK looks for the `LDTK_ACTION` environment variable to determine what to do. Here is the list of
supported actions.

When no `LDTK_ACTION` is specified, or when the action is not recognized, nothing happens.

### Mute

When **LDTK_ACTION** is `"mute"`, LDTK hijacks libc's `write` function and make it do nothing for
file descriptors 1 and 2 (standard output and standard error), ensuring that nothing is ever
displayed to the terminal.

Writing to files is intentionally preserved, however.

Example:

```txt
 >_ LD_PRELOAD=path/to/libldtk.so LDTK_ACTION=mute cat Cargo.toml
```

## Spawn

When **LDTK_ACTION** is `"spawn"`, LDTK simply invoke a command using the `/bin/sh` shell. The
executed command is taken from the **LDTK_SPAWN** environment variable.

Example:

```
 >_ LD_PRELOAD=path/to/libldtk.so LDTK_ACTION=spawn LDTK_SPAWN="echo abc | rev" cat Cargo.toml
cba
[package]
name = "ldtk"
...
```

Here, the `/bin/sh -c "echo abc | rev"` process has been invoked.

## Examples

Mute all programs.

```bash
export LD_PRELOAD=path/to/lib ldtk.so
export LDTK_ACTION=mute
```

Print a message before every command.

```bash
export LD_PRELOAD=path/to/lib ldtk.so
export LDTK_ACTION=spawn
export LDTK_SPAWN="echo i'm in ur address space"
```

Wait a bit before actually starting.

```bash
export LD_PRELOAD=path/to/lib ldtk.so
export LDTK_ACTION=spawn
export LDTK_SPAWN="sleep 1"
```
