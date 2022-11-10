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
pub struct GetDsid200ResponseAllOf {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::GetDsid200ResponseAllOfResult>>,
}

impl GetDsid200ResponseAllOf {
    pub fn new() -> GetDsid200ResponseAllOf {
        GetDsid200ResponseAllOf {
            result: None,
        }
    }
}

