<div align="center">
  <h1><code>cap-directories</code></h1>

  <p>
    <strong>Capability-oriented standard directories for config, cache and other data</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://cirrus-ci.com/github/sunfishcode/cap-std"><img src="https://api.cirrus-ci.com/github/sunfishcode/cap-std.svg" alt="Cirrus CI Status" /></a>
    <a href="https://travis-ci.com/sunfishcode/cap-std"><img src="https://travis-ci.com/sunfishcode/cap-std.svg?branch=main" alt="Travis CI Status" /></a>
    <a href="https://crates.io/crates/cap-directories"><img src="https://img.shields.io/crates/v/cap-directories.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/cap-directories"><img src="https://docs.rs/cap-directories/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

The `cap-directories` crate provides utilities for accessing standard
directories via the [`directories`] crate, but which provide [`Dir`]s instead of
`Path`s.

[`directories`]: https://crates.io/crates/directories
[`Dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html
