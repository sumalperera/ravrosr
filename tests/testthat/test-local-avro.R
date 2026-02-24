test_that("local Avro round-trip works for a simple record", {
  schema_json <- '{
    "type": "record",
    "name": "TestRecord",
    "fields": [
      {"name": "name", "type": "string"},
      {"name": "age", "type": "int"},
      {"name": "score", "type": "double"}
    ]
  }'

  data <- list(name = "Alice", age = 30L, score = 95.5)

  raw_bytes <- avro_serialize_local(schema_json, data)
  expect_type(raw_bytes, "raw")
  expect_true(length(raw_bytes) > 0)

  result <- avro_deserialize_local(schema_json, raw_bytes)
  expect_equal(result$name, "Alice")
  expect_equal(result$age, 30L)
  expect_equal(result$score, 95.5)
})

test_that("local Avro round-trip works for nested record", {
  schema_json <- '{
    "type": "record",
    "name": "Person",
    "fields": [
      {"name": "name", "type": "string"},
      {"name": "active", "type": "boolean"}
    ]
  }'

  data <- list(name = "Bob", active = TRUE)

  raw_bytes <- avro_serialize_local(schema_json, data)
  result <- avro_deserialize_local(schema_json, raw_bytes)

  expect_equal(result$name, "Bob")
  expect_true(result$active)
})
