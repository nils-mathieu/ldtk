# Information

A simple tool to hijack processes when they start.

## How it works

This program expects to be loaded as a dynamic library using the `LD_PRELOAD` environment variable
within a shell (such as Bash or Zsh). Within its initialization code, it will mess with the
hijacked program's memory, files and functions.
