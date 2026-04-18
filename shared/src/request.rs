use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateSecretRequest {
    pub key_name: String,
    pub service_name: String,
    pub key_value: String
}