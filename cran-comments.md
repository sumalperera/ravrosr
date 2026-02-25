## R CMD check results

0 errors | 1 warning | 2 notes

### WARNING

* **Found 'abort', 'exit', '_exit' in compiled code**: These symbols originate
  from the Rust standard library (libstd). All exported Rust functions are
  wrapped by extendr's `catch_unwind` mechanism, which catches panics at the
  FFI boundary before they can propagate to R. The abort/exit paths are
  unreachable in normal operation and exist only as last-resort handlers in
  the Rust runtime. This is standard for all Rust-backed R packages
  (e.g., gifski, polars, string2path).

### NOTEs

* **New submission**: This is a first submission of ravrosr to CRAN.

* **Possibly misspelled words in DESCRIPTION**:
  Avro, async, deserialization, extendr -- these are all legitimate technical
  terms. Avro is the Apache Avro data serialization format, extendr is the
  Rust-R bridge framework, and async/deserialization are standard computing terms.

* **Hidden files and directories (src/rust/.cargo)**: This directory contains
  the Cargo configuration file (`config.toml`) that directs Cargo to use
  vendored dependencies instead of downloading from the internet. It is
  required for offline/CRAN-compliant Rust builds.

### Vendoring and offline build

All Rust dependencies are vendored in `src/rust/vendor.tar.xz` and extracted
during `configure`. The Cargo build uses `--offline` to ensure no network
access occurs during package installation.

### Package size

The package tarball is ~32 MB because Rust dependencies are vendored within
`src/rust/vendor.tar.xz`. This is required by CRAN policy to avoid network
access during package installation. The vendored dependencies include
platform-specific bindings (e.g. `windows-sys`, `linux-raw-sys`) needed for
cross-platform TLS support.

## Test environments

* macOS (local), R 4.5.2
* GitHub Actions: macOS-latest, ubuntu-latest (R release + devel), windows-latest
* win-builder (devel, release)

## Downstream dependencies

None (new package).
