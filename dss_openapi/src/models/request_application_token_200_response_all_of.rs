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
pub struct RequestApplicationToken200ResponseAllOf {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::RequestApplicationToken200ResponseAllOfResult>>,
}

impl RequestApplicationToken200ResponseAllOf {
    pub fn new() -> RequestApplicationToken200ResponseAllOf {
        RequestApplicationToken200ResponseAllOf {
            result: None,
        }
    }
}

