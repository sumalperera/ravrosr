use extendr_api::prelude::*;
use schema_registry_converter::async_impl::schema_registry::SrSettings;

/// Wrapper around the Schema Registry connection settings.
/// Stored as an ExternalPtr in R.
pub struct SrClient {
    pub settings: SrSettings,
    pub base_url: String,
    pub auth_header: Option<String>,
}

impl SrClient {
    pub fn new(url: &str, api_key: Option<&str>, api_secret: Option<&str>) -> Self {
        let (settings, auth_header) = if let (Some(key), Some(secret)) = (api_key, api_secret) {
            use base64::Engine;
            let credentials = format!("{}:{}", key, secret);
            let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
            let header_val = format!("Basic {}", encoded);

            let s = SrSettings::new_builder(url.to_string())
                .set_basic_authorization(key, Some(secret))
                .build()
                .expect("Failed to build SrSettings with auth");
            (s, Some(header_val))
        } else {
            (SrSettings::new(url.to_string()), None)
        };

        SrClient {
            settings,
            base_url: url.to_string(),
            auth_header,
        }
    }
}

/// Create a new Schema Registry client.
/// Returns an external pointer that can be passed to other functions.
#[extendr]
pub fn sr_connect(url: &str, api_key: Nullable<&str>, api_secret: Nullable<&str>) -> Robj {
    let key = match api_key {
        Nullable::NotNull(k) => Some(k),
        Nullable::Null => None,
    };
    let secret = match api_secret {
        Nullable::NotNull(s) => Some(s),
        Nullable::Null => None,
    };

    let client = SrClient::new(url, key, secret);
    let ptr = ExternalPtr::new(client);
    ptr.into_robj()
}

extendr_module! {
    mod client;
    fn sr_connect;
}
