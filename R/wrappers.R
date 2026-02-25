#' Create a Schema Registry client
#'
#' @param url The Schema Registry URL
#' @param api_key API key for authentication (optional)
#' @param api_secret API secret for authentication (optional)
#' @return An external pointer to the client object
#' @examples
#' \dontrun{
#' client <- sr_connect("http://localhost:8081")
#' client <- sr_connect("https://registry.example.com", "my-key", "my-secret")
#' }
#' @export
sr_connect <- function(url, api_key = NULL, api_secret = NULL) {
  .Call(wrap__sr_connect, url, api_key, api_secret)
}

#' List all subjects in the Schema Registry
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @return A character vector of subject names
#' @examples
#' \dontrun{
#' client <- sr_connect("http://localhost:8081")
#' subjects <- sr_list_subjects(client)
#' }
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
#' @examples
#' \dontrun{
#' client <- sr_connect("http://localhost:8081")
#' schema <- sr_get_schema(client, "my-topic-value")
#' schema_v1 <- sr_get_schema(client, "my-topic-value", version = 1L)
#' }
#' @export
sr_get_schema <- function(client, subject, version = NULL) {
  .Call(wrap__sr_get_schema, client, subject, version)
}

#' Get schema JSON by global ID
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param id The global schema ID (integer, as returned by \code{sr_register_schema})
#' @return A string containing the schema JSON
#' @examples
#' \dontrun{
#' client <- sr_connect("http://localhost:8081")
#' schema <- sr_get_schema_by_id(client, 1L)
#' }
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
#' @examples
#' \dontrun{
#' client <- sr_connect("http://localhost:8081")
#' schema <- '{"type":"record","name":"Test","fields":[{"name":"id","type":"int"}]}'
#' id <- sr_register_schema(client, "test-value", schema)
#' }
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
#' @examples
#' \dontrun{
#' client <- sr_connect("http://localhost:8081")
#' schema <- '{"type":"record","name":"Test","fields":[{"name":"id","type":"int"}]}'
#' is_compat <- sr_check_compatibility(client, "test-value", schema)
#' }
#' @export
sr_check_compatibility <- function(client, subject, schema_json) {
  .Call(wrap__sr_check_compatibility, client, subject, schema_json)
}

#' Delete a subject from the Schema Registry
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param subject The subject name
#' @return Logical indicating success
#' @examples
#' \dontrun{
#' client <- sr_connect("http://localhost:8081")
#' sr_delete_subject(client, "test-value")
#' }
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
#' @examples
#' \dontrun{
#' client <- sr_connect("http://localhost:8081")
#' raw_bytes <- avro_serialize(client, "my-topic-value", list(id = 1L, name = "test"))
#' }
#' @export
avro_serialize <- function(client, subject, data) {
  .Call(wrap__avro_serialize, client, subject, data)
}

#' Deserialize Avro data with Confluent wire format
#'
#' @param client A Schema Registry client created with \code{sr_connect}
#' @param raw_bytes A raw vector of Confluent wire format Avro data
#' @return An R list containing the deserialized data
#' @examples
#' \dontrun{
#' client <- sr_connect("http://localhost:8081")
#' record <- avro_deserialize(client, raw_bytes)
#' }
#' @export
avro_deserialize <- function(client, raw_bytes) {
  .Call(wrap__avro_deserialize, client, raw_bytes)
}

#' Serialize R data to Avro binary (no registry)
#'
#' @param schema_json The Avro schema as a JSON string
#' @param data An R list to serialize
#' @return A raw vector containing the serialized data
#' @examples
#' schema <- '{"type":"record","name":"User","fields":[
#'   {"name":"name","type":"string"},
#'   {"name":"age","type":"int"}
#' ]}'
#' raw <- avro_serialize_local(schema, list(name = "Alice", age = 30L))
#' @export
avro_serialize_local <- function(schema_json, data) {
  .Call(wrap__avro_serialize_local, schema_json, data)
}

#' Deserialize Avro binary data (no registry)
#'
#' @param schema_json The Avro schema as a JSON string
#' @param raw_bytes A raw vector of Avro data
#' @return An R list containing the deserialized data
#' @examples
#' schema <- '{"type":"record","name":"User","fields":[
#'   {"name":"name","type":"string"},
#'   {"name":"age","type":"int"}
#' ]}'
#' raw <- avro_serialize_local(schema, list(name = "Alice", age = 30L))
#' result <- avro_deserialize_local(schema, raw)
#' @export
avro_deserialize_local <- function(schema_json, raw_bytes) {
  .Call(wrap__avro_deserialize_local, schema_json, raw_bytes)
}
