#' Create a Schema Registry client
#'
#' @param url The Schema Registry URL
#' @param api_key API key for authentication (optional)
#' @param api_secret API secret for authentication (optional)
#' @return An external pointer to the client object
#' @export
sr_connect <- function(url, api_key = NULL, api_secret = NULL) {
  .Call(wrap__sr_connect, url, api_key, api_secret)
}

#' List all subjects in the Schema Registry
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @return A character vector of subject names
#' @export
sr_list_subjects <- function(client) {
  .Call(wrap__sr_list_subjects, client)
}

#' Get schema JSON for a subject
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param subject The subject name
#' @param version The schema version (NULL for latest)
#' @return A string containing the schema JSON
#' @export
sr_get_schema <- function(client, subject, version = NULL) {
  .Call(wrap__sr_get_schema, client, subject, version)
}

#' Get schema JSON by global ID
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param id The global schema ID (integer, as returned by \code{sr_register_schema})
#' @return A string containing the schema JSON
#' @export
sr_get_schema_by_id <- function(client, id) {
  .Call(wrap__sr_get_schema_by_id, client, id)
}

#' Register a schema under a subject
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param subject The subject name
#' @param schema_json The Avro schema as a JSON string
#' @return The schema ID (integer)
#' @export
sr_register_schema <- function(client, subject, schema_json) {
  .Call(wrap__sr_register_schema, client, subject, schema_json)
}

#' Check compatibility of a schema with a subject
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param subject The subject name
#' @param schema_json The Avro schema as a JSON string
#' @return Logical indicating compatibility
#' @export
sr_check_compatibility <- function(client, subject, schema_json) {
  .Call(wrap__sr_check_compatibility, client, subject, schema_json)
}

#' Delete a subject from the Schema Registry
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param subject The subject name
#' @return Logical indicating success
#' @export
sr_delete_subject <- function(client, subject) {
  .Call(wrap__sr_delete_subject, client, subject)
}

#' Serialize R data to Avro with Confluent wire format
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param subject The subject name (used to look up the schema)
#' @param data An R list to serialize
#' @return A raw vector containing the serialized data
#' @export
avro_serialize <- function(client, subject, data) {
  .Call(wrap__avro_serialize, client, subject, data)
}

#' Deserialize Avro data with Confluent wire format
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param raw_bytes A raw vector of Confluent wire format Avro data
#' @return An R list containing the deserialized data
#' @export
avro_deserialize <- function(client, raw_bytes) {
  .Call(wrap__avro_deserialize, client, raw_bytes)
}

#' Serialize R data to Avro binary (no registry)
#'
#' @param schema_json The Avro schema as a JSON string
#' @param data An R list to serialize
#' @return A raw vector containing the serialized data
#' @export
avro_serialize_local <- function(schema_json, data) {
  .Call(wrap__avro_serialize_local, schema_json, data)
}

#' Deserialize Avro binary data (no registry)
#'
#' @param schema_json The Avro schema as a JSON string
#' @param raw_bytes A raw vector of Avro data
#' @return An R list containing the deserialized data
#' @export
avro_deserialize_local <- function(schema_json, raw_bytes) {
  .Call(wrap__avro_deserialize_local, schema_json, raw_bytes)
}
