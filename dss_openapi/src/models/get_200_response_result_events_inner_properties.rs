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
pub struct Get200ResponseResultEventsInnerProperties {
    #[serde(rename = "callOrigin", skip_serializing_if = "Option::is_none")]
    pub call_origin: Option<String>,
    #[serde(rename = "groupID", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "sceneID", skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<String>,
    #[serde(rename = "zoneID", skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(rename = "originDSUID", skip_serializing_if = "Option::is_none")]
    pub origin_dsuid: Option<String>,
    #[serde(rename = "originToken", skip_serializing_if = "Option::is_none")]
    pub origin_token: Option<String>,
    #[serde(rename = "buttonIndex", skip_serializing_if = "Option::is_none")]
    pub button_index: Option<String>,
    #[serde(rename = "clickType", skip_serializing_if = "Option::is_none")]
    pub click_type: Option<String>,
}

impl Get200ResponseResultEventsInnerProperties {
    pub fn new() -> Get200ResponseResultEventsInnerProperties {
        Get200ResponseResultEventsInnerProperties {
            call_origin: None,
            group_id: None,
            scene_id: None,
            zone_id: None,
            origin_dsuid: None,
            origin_token: None,
            button_index: None,
            click_type: None,
        }
    }
}


