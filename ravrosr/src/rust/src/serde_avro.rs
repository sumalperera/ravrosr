use apache_avro::Schema;
use extendr_api::prelude::*;
use schema_registry_converter::async_impl::schema_registry::{get_schema_by_id, get_schema_by_subject};
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

use crate::client::SrClient;
use crate::conversions::{robj_to_avro, avro_to_robj};
use crate::runtime::TOKIO_RT;

/// Serialize R data to Avro binary with Confluent wire format (5-byte header).
/// The wire format is: magic byte (0x00) + 4-byte schema ID + Avro binary.
#[extendr]
pub fn avro_serialize(client: Robj, subject: &str, data: Robj) -> Robj {
    let ptr: ExternalPtr<SrClient> = client.try_into()
        .expect("Expected a Schema Registry client (created with sr_connect)");
    let strategy = SubjectNameStrategy::TopicNameStrategy(subject.to_string(), false);

    let schema_result = TOKIO_RT.block_on(async {
        get_schema_by_subject(&ptr.settings, &strategy).await
    }).expect("Failed to get schema for subject");

    let schema_str = &schema_result.schema;
    let schema_id = schema_result.id;
    let schema = Schema::parse_str(schema_str)
        .expect("Failed to parse schema");

    let avro_value = robj_to_avro(&data, &schema)
        .expect("Failed to convert R data to Avro value");

    let datum = apache_avro::to_avro_datum(&schema, avro_value)
        .expect("Failed to encode Avro datum");

    // Build wire format: 0x00 + 4-byte big-endian schema ID + avro datum
    let mut wire = Vec::with_capacity(5 + datum.len());
    wire.push(0u8);
    wire.extend_from_slice(&schema_id.to_be_bytes());
    wire.extend_from_slice(&datum);

    wire.into_robj()
}

/// Deserialize Avro binary with Confluent wire format back to R data.
#[extendr]
pub fn avro_deserialize(client: Robj, raw_bytes: Raw) -> Robj {
    let ptr: ExternalPtr<SrClient> = client.try_into()
        .expect("Expected a Schema Registry client (created with sr_connect)");
    let bytes = raw_bytes.as_slice();

    if bytes.len() < 5 || bytes[0] != 0 {
        panic!("Invalid Confluent wire format: missing magic byte or too short");
    }

    let schema_id = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
    let avro_data = &bytes[5..];

    let schema_result = TOKIO_RT.block_on(async {
        get_schema_by_id(schema_id, &ptr.settings).await
    }).expect("Failed to get schema by ID");

    let schema = Schema::parse_str(&schema_result.schema)
        .expect("Failed to parse schema");

    let avro_value = apache_avro::from_avro_datum(&schema, &mut &avro_data[..], None)
        .expect("Failed to decode Avro datum");

    avro_to_robj(&avro_value)
        .expect("Failed to convert Avro value to R")
}

extendr_module! {
    mod serde_avro;
    fn avro_serialize;
    fn avro_deserialize;
}
