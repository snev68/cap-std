This crate provides a capability-based version of [`std`]. It provides all the
interfaces you are used to, but in a capability-based version.

[`std`]: https://doc.rust-lang.org/std/index.html

It is a work in progress and many things aren't implemented yet.

The two most interesting features are `fs::Dir` and `net::Catalog` (name TBD).
Dirs represent capabilities for accessing files beneath them, and "catalogs"
represent capabilities for creating network connections.

This library has two potential uses in the WASI ecosystem. First, an implementation
abstraction within wasi-common, abstracting over some yanix/winx differences. And
second, a user library, for writing applications that use std-like APIs but that
don't require a preopen-like layer.

Things to think about:
 - Should `try_clone` and other methods that consume resources require
   a capability?
 - Should we provide a capability-oriented [`std::process::Command`]?
 - Rust's `Path` has several ambient-authority methods: `metadata`,
   `read_link`, `read_dir`, `symlink_metadata`, `canonicalize`. Is it
   worth having our own version of `Path` just to exclude those? Such a
   thing could also exclude absolute paths.
 - utf8-std (with [arf strings])? utf8-cap-std? utf8-cap-async-std?
   Where is this going?
 - Should we provide any of Rust's Unix-specific APIs on Windows, using
   winx and emulation?

[arf strings]: https://github.com/bytecodealliance/arf-strings/
[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
