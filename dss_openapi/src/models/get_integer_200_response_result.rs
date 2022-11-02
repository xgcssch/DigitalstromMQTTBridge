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
pub struct GetInteger200ResponseResult {
    /// integer value of the property
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl GetInteger200ResponseResult {
    pub fn new() -> GetInteger200ResponseResult {
        GetInteger200ResponseResult {
            value: None,
        }
    }
}

