/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.33
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_alliances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesError {
    Status400(models::BadRequest),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_alliances_alliance_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdError {
    Status400(models::BadRequest),
    Status404(models::GetAlliancesAllianceIdNotFound),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_alliances_alliance_id_corporations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdCorporationsError {
    Status400(models::BadRequest),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_alliances_alliance_id_icons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdIconsError {
    Status400(models::BadRequest),
    Status404(models::GetAlliancesAllianceIdIconsNotFound),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}


/// List all active player alliances  --- Alternate route: `/dev/alliances/`  Alternate route: `/legacy/alliances/`  Alternate route: `/v1/alliances/`  Alternate route: `/v2/alliances/`  --- This route is cached for up to 3600 seconds
pub async fn get_alliances(configuration: &configuration::Configuration, datasource: Option<&str>, if_none_match: Option<&str>) -> Result<Vec<i32>, Error<GetAlliancesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;

    let uri_str = format!("{}/alliances/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;i32&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;i32&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAlliancesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Public information about an alliance  --- Alternate route: `/dev/alliances/{alliance_id}/`  Alternate route: `/legacy/alliances/{alliance_id}/`  Alternate route: `/v3/alliances/{alliance_id}/`  Alternate route: `/v4/alliances/{alliance_id}/`  --- This route is cached for up to 3600 seconds
pub async fn get_alliances_alliance_id(configuration: &configuration::Configuration, alliance_id: i32, datasource: Option<&str>, if_none_match: Option<&str>) -> Result<models::GetAlliancesAllianceIdOk, Error<GetAlliancesAllianceIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_alliance_id = alliance_id;
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;

    let uri_str = format!("{}/alliances/{alliance_id}/", configuration.base_path, alliance_id=p_alliance_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetAlliancesAllianceIdOk`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetAlliancesAllianceIdOk`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAlliancesAllianceIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// List all current member corporations of an alliance  --- Alternate route: `/dev/alliances/{alliance_id}/corporations/`  Alternate route: `/legacy/alliances/{alliance_id}/corporations/`  Alternate route: `/v1/alliances/{alliance_id}/corporations/`  Alternate route: `/v2/alliances/{alliance_id}/corporations/`  --- This route is cached for up to 3600 seconds
pub async fn get_alliances_alliance_id_corporations(configuration: &configuration::Configuration, alliance_id: i32, datasource: Option<&str>, if_none_match: Option<&str>) -> Result<Vec<i32>, Error<GetAlliancesAllianceIdCorporationsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_alliance_id = alliance_id;
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;

    let uri_str = format!("{}/alliances/{alliance_id}/corporations/", configuration.base_path, alliance_id=p_alliance_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;i32&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;i32&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAlliancesAllianceIdCorporationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get the icon urls for a alliance  --- Alternate route: `/dev/alliances/{alliance_id}/icons/`  Alternate route: `/legacy/alliances/{alliance_id}/icons/`  Alternate route: `/v1/alliances/{alliance_id}/icons/`  Alternate route: `/v2/alliances/{alliance_id}/icons/`  --- This route expires daily at 11:05
pub async fn get_alliances_alliance_id_icons(configuration: &configuration::Configuration, alliance_id: i32, datasource: Option<&str>, if_none_match: Option<&str>) -> Result<models::GetAlliancesAllianceIdIconsOk, Error<GetAlliancesAllianceIdIconsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_alliance_id = alliance_id;
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;

    let uri_str = format!("{}/alliances/{alliance_id}/icons/", configuration.base_path, alliance_id=p_alliance_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetAlliancesAllianceIdIconsOk`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetAlliancesAllianceIdIconsOk`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAlliancesAllianceIdIconsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

