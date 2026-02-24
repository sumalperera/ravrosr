use extendr_api::prelude::*;
use schema_registry_converter::async_impl::schema_registry::{
    get_all_subjects, get_schema_by_subject, post_schema,
};
use schema_registry_converter::schema_registry_common::{SubjectNameStrategy, SuppliedSchema, SchemaType};

use crate::client::SrClient;
use crate::runtime::TOKIO_RT;

/// List all subjects in the Schema Registry.
#[extendr]
pub fn sr_list_subjects(client: Robj) -> Strings {
    let ptr: ExternalPtr<SrClient> = client.try_into()
        .expect("Expected a Schema Registry client (created with sr_connect)");
    let subjects = TOKIO_RT.block_on(async {
        get_all_subjects(&ptr.settings).await
    }).expect("Failed to list subjects from Schema Registry");

    Strings::from_values(subjects)
}

/// Get schema JSON for a subject at a given version.
/// If version is NULL, gets the latest version.
#[extendr]
pub fn sr_get_schema(client: Robj, subject: &str, version: Nullable<i32>) -> String {
    let ptr: ExternalPtr<SrClient> = client.try_into()
        .expect("Expected a Schema Registry client (created with sr_connect)");
    let ver = match version {
        Nullable::NotNull(v) => v as u32,
        Nullable::Null => 0,
    };

    if ver > 0 {
        let url = format!("{}/subjects/{}/versions/{}", ptr.base_url, subject, ver);
        let http_client = reqwest::Client::new();
        let response = TOKIO_RT.block_on(async {
            let mut req = http_client.get(&url);
            if let Some(ref auth) = ptr.auth_header {
                req = req.header("Authorization", auth);
            }
            req.send().await
                .expect("HTTP request failed")
                .json::<serde_json::Value>().await
                .expect("Failed to parse response")
        });
        return response["schema"].as_str()
            .expect("No 'schema' field in response")
            .to_string();
    }

    let strategy = SubjectNameStrategy::TopicNameStrategy(subject.to_string(), false);
    let result = TOKIO_RT.block_on(async {
        get_schema_by_subject(&ptr.settings, &strategy).await
    }).expect("Failed to get schema from Schema Registry");

    result.schema
}

/// Register a schema under a subject. Returns the schema ID.
#[extendr]
pub fn sr_register_schema(client: Robj, subject: &str, schema_json: &str) -> i32 {
    let ptr: ExternalPtr<SrClient> = client.try_into()
        .expect("Expected a Schema Registry client (created with sr_connect)");

    let supplied = SuppliedSchema {
        name: Some(subject.to_string()),
        schema_type: SchemaType::Avro,
        schema: schema_json.to_string(),
        references: vec![],
        properties: None,
        tags: None,
    };

    let result = TOKIO_RT.block_on(async {
        post_schema(&ptr.settings, subject.to_string(), supplied).await
    });

    match result {
        Ok(registered) => registered.id as i32,
        Err(e) => {
            let url = format!("{}/subjects/{}/versions", ptr.base_url, subject);
            let http_client = reqwest::Client::new();
            let body = serde_json::json!({
                "schema": schema_json,
                "schemaType": "AVRO"
            });

            let response = TOKIO_RT.block_on(async {
                let mut req = http_client.post(&url)
                    .header("Content-Type", "application/vnd.schemaregistry.v1+json")
                    .json(&body);
                if let Some(ref auth) = ptr.auth_header {
                    req = req.header("Authorization", auth);
                }
                req.send().await
                    .expect("HTTP request failed")
                    .json::<serde_json::Value>().await
                    .expect("Failed to parse response")
            });

            response["id"].as_i64()
                .unwrap_or_else(|| panic!("Failed to register schema: {} - response: {}", e, response)) as i32
        }
    }
}

/// Check compatibility of a schema with a subject.
#[extendr]
pub fn sr_check_compatibility(client: Robj, subject: &str, schema_json: &str) -> bool {
    let ptr: ExternalPtr<SrClient> = client.try_into()
        .expect("Expected a Schema Registry client (created with sr_connect)");
    let url = format!("{}/compatibility/subjects/{}/versions/latest", ptr.base_url, subject);
    let http_client = reqwest::Client::new();
    let body = serde_json::json!({
        "schema": schema_json,
        "schemaType": "AVRO"
    });

    let response = TOKIO_RT.block_on(async {
        let mut req = http_client.post(&url)
            .header("Content-Type", "application/vnd.schemaregistry.v1+json")
            .json(&body);
        if let Some(ref auth) = ptr.auth_header {
            req = req.header("Authorization", auth);
        }
        req.send().await
            .expect("HTTP request failed")
            .json::<serde_json::Value>().await
            .expect("Failed to parse response")
    });

    response["is_compatible"].as_bool().unwrap_or(false)
}

/// Delete a subject from the Schema Registry.
#[extendr]
pub fn sr_delete_subject(client: Robj, subject: &str) -> bool {
    let ptr: ExternalPtr<SrClient> = client.try_into()
        .expect("Expected a Schema Registry client (created with sr_connect)");
    let url = format!("{}/subjects/{}", ptr.base_url, subject);
    let http_client = reqwest::Client::new();

    let response = TOKIO_RT.block_on(async {
        let mut req = http_client.delete(&url);
        if let Some(ref auth) = ptr.auth_header {
            req = req.header("Authorization", auth);
        }
        req.send().await
            .expect("HTTP request failed")
    });

    response.status().is_success()
}

extendr_module! {
    mod registry_ops;
    fn sr_list_subjects;
    fn sr_get_schema;
    fn sr_register_schema;
    fn sr_check_compatibility;
    fn sr_delete_subject;
}
