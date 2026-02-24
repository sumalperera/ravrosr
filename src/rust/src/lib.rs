use extendr_api::prelude::*;

mod runtime;
mod conversions;
mod client;
mod serde_local;
mod registry_ops;
mod serde_avro;

extendr_module! {
    mod ravrosr;
    use client;
    use registry_ops;
    use serde_local;
    use serde_avro;
}
