use extendr_api::prelude::*;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord, DefaultProducerContext, Producer};
use std::time::Duration;

pub struct KafkaProducer {
    producer: BaseProducer<DefaultProducerContext>,
}

/// Create a Kafka producer from a named list of config values.
/// Config keys are passed directly to librdkafka.
#[extendr]
pub fn kafka_producer_new(config: List) -> Robj {
    let mut client_config = ClientConfig::new();
    for (key, value) in config.iter() {
        let v = value
            .as_str()
            .expect("Config values must be strings");
        client_config.set(key, v);
    }
    let producer: BaseProducer<DefaultProducerContext> = client_config
        .create()
        .expect("Failed to create Kafka producer");
    ExternalPtr::new(KafkaProducer { producer }).into_robj()
}

/// Produce a raw byte message to a Kafka topic.
/// value must be a raw vector (e.g. from avro_serialize).
/// key is an optional character string.
#[extendr]
pub fn kafka_produce(producer: Robj, topic: &str, value: Raw, key: Nullable<&str>) {
    let ptr: ExternalPtr<KafkaProducer> = producer
        .try_into()
        .expect("Expected a Kafka producer (created with kafka_producer)");

    let mut record = BaseRecord::to(topic).payload(value.as_slice());
    if let Nullable::NotNull(k) = key {
        record = record.key(k);
    }

    if let Err((err, _)) = ptr.producer.send(record) {
        panic!("Failed to produce message: {}", err);
    }
    ptr.producer.poll(Duration::from_millis(0));
}

/// Produce a character string message to a Kafka topic.
/// For sending JSON or other text payloads.
#[extendr]
pub fn kafka_produce_text(producer: Robj, topic: &str, value: &str, key: Nullable<&str>) {
    let ptr: ExternalPtr<KafkaProducer> = producer
        .try_into()
        .expect("Expected a Kafka producer (created with kafka_producer)");

    let mut record = BaseRecord::to(topic).payload(value.as_bytes());
    if let Nullable::NotNull(k) = key {
        record = record.key(k);
    }

    if let Err((err, _)) = ptr.producer.send(record) {
        panic!("Failed to produce message: {}", err);
    }
    ptr.producer.poll(Duration::from_millis(0));
}

/// Flush the producer, waiting up to timeout_ms for all messages to be delivered.
#[extendr]
pub fn kafka_flush(producer: Robj, timeout_ms: i32) {
    let ptr: ExternalPtr<KafkaProducer> = producer
        .try_into()
        .expect("Expected a Kafka producer (created with kafka_producer)");

    ptr.producer
        .flush(Duration::from_millis(timeout_ms as u64))
        .expect("Failed to flush producer");
}

extendr_module! {
    mod kafka_producer;
    fn kafka_producer_new;
    fn kafka_produce;
    fn kafka_produce_text;
    fn kafka_flush;
}
