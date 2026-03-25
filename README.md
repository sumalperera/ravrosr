# ravrosr

<!-- badges: start -->
[![R-CMD-check](https://github.com/sumalperera/ravrosr/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/sumalperera/ravrosr/actions/workflows/R-CMD-check.yaml)
[![lint](https://github.com/sumalperera/ravrosr/actions/workflows/lint.yaml/badge.svg)](https://github.com/sumalperera/ravrosr/actions/workflows/lint.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
<!-- badges: end -->

`ravrosr` brings Avro serialization, Schema Registry support, and Kafka producing to R, with a Rust backend via [extendr](https://extendr.rs/).

It supports Confluent Schema Registry, Redpanda, and other API-compatible registries.

## What this package does

- Serialize and deserialize Avro data locally (no registry required)
- Serialize and deserialize Confluent wire-format payloads (`0x00 + schema_id + avro`)
- Connect to Schema Registry and manage subjects/schemas from R
- Produce messages to Kafka/Redpanda with native raw byte support (Avro, JSON, or text)

## Requirements

- R `>= 4.2`
- [Rust toolchain](https://rust-lang.org/tools/install/) (`rustc >= 1.67`, `cargo`)
- [CMake](https://cmake.org/) (builds bundled librdkafka, OpenSSL, and SASL from source)
  - Debian/Ubuntu: `apt-get install cmake`
  - macOS: `brew install cmake` or included with Xcode
  - RHEL/Fedora: `dnf install cmake`
- Build tools:
  - macOS uses Xcode Command Line Tools (`xcode-select --install`)
  - Windows uses Rtools
  - Linux uses a standard C/C++ build toolchain (`build-essential` or equivalent)

`ravrosr` is currently not on CRAN.

## Installation

```r
if (!requireNamespace("remotes", quietly = TRUE)) {
  install.packages("remotes")
}

remotes::install_git("https://github.com/sumalperera/ravrosr")
```

If the above fails (e.g. due to offline/CRAN build restrictions), you can install from a local clone:

```bash
git clone https://github.com/sumalperera/ravrosr /tmp/ravrosr
NOT_CRAN=true R CMD INSTALL /tmp/ravrosr
```

## Quick start

### 1) Local Avro (no registry)

```r
library(ravrosr)

schema <- '{
  "type": "record",
  "name": "User",
  "fields": [
    {"name": "name", "type": "string"},
    {"name": "age", "type": "int"},
    {"name": "score", "type": "double"}
  ]
}'

payload <- list(name = "Alice", age = 30L, score = 95.5)

raw_bytes <- avro_serialize_local(schema, payload)
decoded <- avro_deserialize_local(schema, raw_bytes)

decoded$name
#> [1] "Alice"
```

### 2) Schema Registry workflow

```r
library(ravrosr)

schema <- '{
  "type": "record",
  "name": "User",
  "fields": [
    {"name": "name", "type": "string"},
    {"name": "age", "type": "int"}
  ]
}'

payload <- list(name = "Bob", age = 42L)

# Use one connection style:

# Local/Redpanda (no auth)
client <- sr_connect("http://localhost:8081")

# Confluent Cloud (API key + secret)
# client <- sr_connect(
#   url = "https://<your-registry-endpoint>",
#   api_key = Sys.getenv("SR_API_KEY"),
#   api_secret = Sys.getenv("SR_API_SECRET")
# )

schema_id <- sr_register_schema(client, "user-value", schema)
wire_bytes <- avro_serialize(client, "user-value", payload)
decoded <- avro_deserialize(client, wire_bytes)

# Serialize using a specific schema version
wire_bytes <- avro_serialize(client, "user-value", payload, version = 1)
```

### 3) Kafka/Redpanda producer

```r
library(ravrosr)

# Connect to schema registry
client <- sr_connect(
  url = Sys.getenv("SCHEMA_REGISTRY_ENDPOINT"),
  api_key = Sys.getenv("REDPANDA_SASL_USERNAME"),
  api_secret = Sys.getenv("REDPANDA_SASL_PASSWORD")
)

# Create a Kafka producer (config passed directly to librdkafka)
producer <- kafka_producer(list(
  "bootstrap.servers" = Sys.getenv("REDPANDA_BOOTSTRAP_SERVERS"),
  "security.protocol" = Sys.getenv("REDPANDA_SECURITY_PROTOCOL"),
  "sasl.mechanism"    = Sys.getenv("REDPANDA_SASL_MECHANISM"),
  "sasl.username"     = Sys.getenv("REDPANDA_SASL_USERNAME"),
  "sasl.password"     = Sys.getenv("REDPANDA_SASL_PASSWORD")
))

# Serialize to Avro and produce raw bytes
data <- list(name = "Alice", age = 30L)
raw_bytes <- avro_serialize(client, "my-topic-value", data, version = 1)
kafka_produce(producer, "my-topic", raw_bytes, key = "my-key")

# Or produce a JSON/text message directly
kafka_produce_text(producer, "my-topic", '{"hello": "world"}', key = "my-key")

# Flush to ensure delivery
kafka_flush(producer)
```

## Common Schema Registry operations

```r
# List all subjects
sr_list_subjects(client)

# Fetch latest schema for a subject
sr_get_schema(client, "user-value")

# Fetch a specific version
sr_get_schema(client, "user-value", version = 1)

# Fetch schema by global schema ID
sr_get_schema_by_id(client, id = schema_id)

# Check compatibility before registering
sr_check_compatibility(client, "user-value", schema)

# Delete a subject
sr_delete_subject(client, "user-value")
```

## API reference

| Function | Description |
|---|---|
| `sr_connect(url, api_key, api_secret)` | Create a Schema Registry client |
| `sr_list_subjects(client)` | List subjects |
| `sr_get_schema(client, subject, version)` | Get schema JSON (`NULL` = latest) |
| `sr_get_schema_by_id(client, id)` | Get schema JSON by global ID |
| `sr_register_schema(client, subject, schema_json)` | Register schema and return ID |
| `sr_check_compatibility(client, subject, schema_json)` | Check compatibility |
| `sr_delete_subject(client, subject)` | Delete subject |
| `avro_serialize(client, subject, data, version)` | Serialize using Confluent wire format (`NULL` = latest) |
| `avro_deserialize(client, raw_bytes)` | Deserialize Confluent wire format |
| `avro_serialize_local(schema_json, data)` | Serialize Avro without registry |
| `avro_deserialize_local(schema_json, raw_bytes)` | Deserialize Avro without registry |
| `kafka_producer(config)` | Create a Kafka producer |
| `kafka_produce(producer, topic, value, key)` | Produce a raw byte message (`value` is a `raw` vector, e.g. from `avro_serialize`) |
| `kafka_produce_text(producer, topic, value, key)` | Produce a text message (`value` is a `character` string, e.g. JSON) |
| `kafka_flush(producer, timeout)` | Flush pending messages |

## Avro type mapping

| Avro type | R type |
|---|---|
| `null` | `NULL` |
| `boolean` | `logical` |
| `int` | `integer` |
| `long` | `double` (R has no native 64-bit integer scalar) |
| `float` | `double` |
| `double` | `double` |
| `string` | `character` |
| `bytes` | `raw` |
| `record` | named `list` |
| `array` | `list` |
| `map` | named `list` |
| `enum` | `character` |
| `union` | auto-matched |
| `fixed` | `raw` |

## Troubleshooting

- Rust not found during install: confirm `rustc --version` and `cargo --version` work in the same shell used by R.
- CMake not found: install with `brew install cmake` (macOS) or `apt install cmake` (Linux).
- Build failures on macOS: run `xcode-select --install`.
- Authentication errors with Confluent Cloud: verify endpoint URL, API key/secret, and network access to the registry endpoint.
- Kafka producer connection issues: check that `bootstrap.servers`, `security.protocol`, and SASL credentials are correct. The config list is passed directly to [librdkafka](https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md).

## License

MIT
