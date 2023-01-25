# Information

A simple tool to hijack processes when they start.

## How it works

This program expects to be loaded as a dynamic library using the `LD_PRELOAD` environment variable
within a shell (such as Bash or Zsh). In its initialization code, the library will mess with the
hijacked program's memory, files and functions.

# Tutorial

## Installation

This program is only supported on Linux.

At the moment, you can only build the binary from source. You will need Cargo and a recent-enough
Rust toolchain.

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

LDTK looks for the `LDTK_ACTION` environment variable to determine what to do. At the moment,
however, only one action is supported.

### Mute

When **LDTK_ACTION** is `"mute"`, LDTK hijacks libc's `write` function and make it return 1
inconditionally, ensuring that nothing is ever displayed to the standard input (or in any other
file, for that matter).
