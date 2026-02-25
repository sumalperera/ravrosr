## R CMD check results

0 errors | 0 warnings | 2 notes

### NOTEs

* **New submission**: This is a first submission of ravrosr to CRAN.

* **Found the following (possibly) misspelled words in DESCRIPTION**:
  Avro, async, deserialization, extendr — these are all legitimate technical terms.
  Avro is the Apache Avro data serialization format, extendr is the Rust-R
  bridge framework, and async/deserialization are standard computing terms.

* **Found 'abort', 'exit', '_exit' in compiled code**: These symbols originate
  from the Rust standard library (libstd). All exported Rust functions are
  wrapped by extendr's `catch_unwind` mechanism, which catches panics at the
  FFI boundary before they can propagate to R. The abort/exit paths are
  unreachable in normal operation and exist only as last-resort handlers in
  the Rust runtime.

## Test environments

* macOS (local), R 4.5
* win-builder (devel, release)

## Downstream dependencies

None (new package).
