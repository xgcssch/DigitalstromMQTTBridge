/*
 * dSS API
 *
 * dSS Server JSON Interface
 *
 * The version of the OpenAPI document: 1.6.0
 * Contact: xgcssch@github.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestApplicationToken200ResponseAllOfResult {
    /// application token as string
    #[serde(rename = "applicationToken")]
    pub application_token: String,
}

impl RequestApplicationToken200ResponseAllOfResult {
    pub fn new(application_token: String) -> RequestApplicationToken200ResponseAllOfResult {
        RequestApplicationToken200ResponseAllOfResult {
            application_token,
        }
    }
}

