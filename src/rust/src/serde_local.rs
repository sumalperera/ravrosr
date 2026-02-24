use apache_avro::{Schema, Writer, Reader};
use extendr_api::prelude::*;
use crate::conversions::{robj_to_avro, avro_to_robj};

/// Serialize an R list to Avro binary (no registry, no wire format).
#[extendr]
pub fn avro_serialize_local(schema_json: &str, data: Robj) -> Robj {
    let schema = Schema::parse_str(schema_json)
        .expect("Failed to parse Avro schema JSON");

    let avro_value = robj_to_avro(&data, &schema)
        .expect("Failed to convert R data to Avro value");

    let mut writer = Writer::new(&schema, Vec::new());
    writer.append(avro_value).expect("Failed to write Avro value");
    let encoded = writer.into_inner().expect("Failed to flush Avro writer");

    // Return as raw vector via Vec<u8> -> Robj conversion
    encoded.into_robj()
}

/// Deserialize Avro binary to an R list (no registry, no wire format).
#[extendr]
pub fn avro_deserialize_local(schema_json: &str, raw_bytes: Raw) -> Robj {
    let schema = Schema::parse_str(schema_json)
        .expect("Failed to parse Avro schema JSON");

    let reader = Reader::with_schema(&schema, raw_bytes.as_slice())
        .expect("Failed to create Avro reader");

    let mut results: Vec<Robj> = Vec::new();
    for value in reader {
        let avro_val = value.expect("Failed to read Avro value");
        let robj = avro_to_robj(&avro_val)
            .expect("Failed to convert Avro value to R");
        results.push(robj);
    }

    if results.len() == 1 {
        results.into_iter().next().unwrap()
    } else {
        let list = List::from_values(results);
        list.into_robj()
    }
}

extendr_module! {
    mod serde_local;
    fn avro_serialize_local;
    fn avro_deserialize_local;
}
