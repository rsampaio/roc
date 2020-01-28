/*
 * Open Service Broker API
 *
 * The Open Service Broker API defines an HTTP(S) interface between Platforms and Service Brokers.
 *
 * The version of the OpenAPI document: master - might contain changes that are not yet released
 * Contact: open-service-broker-api@googlegroups.com
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
use std::rc::Rc;

use reqwest;

use super::{configuration, Error};

pub struct ServiceBindingsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ServiceBindingsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ServiceBindingsApiClient {
        ServiceBindingsApiClient {
            configuration: configuration,
        }
    }
}

pub trait ServiceBindingsApi {
    fn service_binding_binding(
        &self,
        x_broker_api_version: &str,
        instance_id: &str,
        binding_id: &str,
        service_binding_request: crate::models::ServiceBindingRequest,
        x_broker_api_originating_identity: &str,
        accepts_incomplete: bool,
    ) -> Result<crate::models::ServiceBindingResponse, Error>;
    fn service_binding_get(
        &self,
        x_broker_api_version: &str,
        instance_id: &str,
        binding_id: &str,
        x_broker_api_originating_identity: &str,
        service_id: &str,
        plan_id: &str,
    ) -> Result<crate::models::ServiceBindingResource, Error>;
    fn service_binding_last_operation_get(
        &self,
        x_broker_api_version: &str,
        instance_id: &str,
        binding_id: &str,
        service_id: &str,
        plan_id: &str,
        operation: &str,
    ) -> Result<crate::models::LastOperationResource, Error>;
    fn service_binding_unbinding(
        &self,
        x_broker_api_version: &str,
        instance_id: &str,
        binding_id: &str,
        service_id: &str,
        plan_id: &str,
        x_broker_api_originating_identity: &str,
        accepts_incomplete: bool,
    ) -> Result<serde_json::Value, Error>;
}

impl ServiceBindingsApi for ServiceBindingsApiClient {
    fn service_binding_binding(
        &self,
        x_broker_api_version: &str,
        instance_id: &str,
        binding_id: &str,
        service_binding_request: crate::models::ServiceBindingRequest,
        x_broker_api_originating_identity: &str,
        accepts_incomplete: bool,
    ) -> Result<crate::models::ServiceBindingResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/v2/service_instances/{instance_id}/service_bindings/{binding_id}",
            configuration.base_path,
            instance_id = crate::apis::urlencode(instance_id),
            binding_id = crate::apis::urlencode(binding_id)
        );
        let mut req_builder = client.put(uri_str.as_str());

        req_builder = req_builder.query(&[("accepts_incomplete", &accepts_incomplete.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("X-Broker-API-Version", x_broker_api_version.to_string());
        req_builder = req_builder.header(
            "X-Broker-API-Originating-Identity",
            x_broker_api_originating_identity.to_string(),
        );
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&service_binding_request);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn service_binding_get(
        &self,
        x_broker_api_version: &str,
        instance_id: &str,
        binding_id: &str,
        x_broker_api_originating_identity: &str,
        service_id: &str,
        plan_id: &str,
    ) -> Result<crate::models::ServiceBindingResource, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/v2/service_instances/{instance_id}/service_bindings/{binding_id}",
            configuration.base_path,
            instance_id = crate::apis::urlencode(instance_id),
            binding_id = crate::apis::urlencode(binding_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("service_id", &service_id.to_string())]);
        req_builder = req_builder.query(&[("plan_id", &plan_id.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("X-Broker-API-Version", x_broker_api_version.to_string());
        req_builder = req_builder.header(
            "X-Broker-API-Originating-Identity",
            x_broker_api_originating_identity.to_string(),
        );
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn service_binding_last_operation_get(
        &self,
        x_broker_api_version: &str,
        instance_id: &str,
        binding_id: &str,
        service_id: &str,
        plan_id: &str,
        operation: &str,
    ) -> Result<crate::models::LastOperationResource, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/v2/service_instances/{instance_id}/service_bindings/{binding_id}/last_operation",
            configuration.base_path,
            instance_id = crate::apis::urlencode(instance_id),
            binding_id = crate::apis::urlencode(binding_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("service_id", &service_id.to_string())]);
        req_builder = req_builder.query(&[("plan_id", &plan_id.to_string())]);
        req_builder = req_builder.query(&[("operation", &operation.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("X-Broker-API-Version", x_broker_api_version.to_string());
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn service_binding_unbinding(
        &self,
        x_broker_api_version: &str,
        instance_id: &str,
        binding_id: &str,
        service_id: &str,
        plan_id: &str,
        x_broker_api_originating_identity: &str,
        accepts_incomplete: bool,
    ) -> Result<serde_json::Value, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/v2/service_instances/{instance_id}/service_bindings/{binding_id}",
            configuration.base_path,
            instance_id = crate::apis::urlencode(instance_id),
            binding_id = crate::apis::urlencode(binding_id)
        );
        let mut req_builder = client.delete(uri_str.as_str());

        req_builder = req_builder.query(&[("service_id", &service_id.to_string())]);
        req_builder = req_builder.query(&[("plan_id", &plan_id.to_string())]);
        req_builder = req_builder.query(&[("accepts_incomplete", &accepts_incomplete.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("X-Broker-API-Version", x_broker_api_version.to_string());
        req_builder = req_builder.header(
            "X-Broker-API-Originating-Identity",
            x_broker_api_originating_identity.to_string(),
        );
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}