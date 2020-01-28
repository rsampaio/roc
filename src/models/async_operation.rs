/*
 * Open Service Broker API
 *
 * The Open Service Broker API defines an HTTP(S) interface between Platforms and Service Brokers.
 *
 * The version of the OpenAPI document: master - might contain changes that are not yet released
 * Contact: open-service-broker-api@googlegroups.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AsyncOperation {
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
}

impl AsyncOperation {
    pub fn new() -> AsyncOperation {
        AsyncOperation { operation: None }
    }
}