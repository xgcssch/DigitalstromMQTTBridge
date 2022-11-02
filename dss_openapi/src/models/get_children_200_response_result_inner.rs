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
pub struct GetChildren200ResponseResultInner {
    /// name of the childnode
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// type of the childnode
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GetChildren200ResponseResultInner {
    pub fn new() -> GetChildren200ResponseResultInner {
        GetChildren200ResponseResultInner {
            name: None,
            r#type: None,
        }
    }
}

