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


/// struct for typed errors of method [`delete_characters_character_id_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCharactersCharacterIdContactsError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_alliances_alliance_id_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdContactsError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_alliances_alliance_id_contacts_labels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdContactsLabelsError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdContactsError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_contacts_labels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdContactsLabelsError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_corporations_corporation_id_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCorporationsCorporationIdContactsError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_corporations_corporation_id_contacts_labels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCorporationsCorporationIdContactsLabelsError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_characters_character_id_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCharactersCharacterIdContactsError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    Status520(models::PostCharactersCharacterIdContactsError520),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_characters_character_id_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutCharactersCharacterIdContactsError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status420(models::ErrorLimited),
    Status500(models::InternalServerError),
    Status503(models::ServiceUnavailable),
    Status504(models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}


/// Bulk delete contacts  --- Alternate route: `/dev/characters/{character_id}/contacts/`  Alternate route: `/legacy/characters/{character_id}/contacts/`  Alternate route: `/v1/characters/{character_id}/contacts/`  Alternate route: `/v2/characters/{character_id}/contacts/` 
pub async fn delete_characters_character_id_contacts(configuration: &configuration::Configuration, character_id: i32, contact_ids: Vec<i32>, datasource: Option<&str>, token: Option<&str>) -> Result<(), Error<DeleteCharactersCharacterIdContactsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_character_id = character_id;
    let p_contact_ids = contact_ids;
    let p_datasource = datasource;
    let p_token = token;

    let uri_str = format!("{}/characters/{character_id}/contacts/", configuration.base_path, character_id=p_character_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    req_builder = match "csv" {
        "multi" => req_builder.query(&p_contact_ids.into_iter().map(|p| ("contact_ids".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => req_builder.query(&[("contact_ids", &p_contact_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_token {
        req_builder = req_builder.query(&[("token", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteCharactersCharacterIdContactsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return contacts of an alliance  --- Alternate route: `/dev/alliances/{alliance_id}/contacts/`  Alternate route: `/v2/alliances/{alliance_id}/contacts/`  --- This route is cached for up to 300 seconds
pub async fn get_alliances_alliance_id_contacts(configuration: &configuration::Configuration, alliance_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, page: Option<i32>, token: Option<&str>) -> Result<Vec<models::GetAlliancesAllianceIdContacts200Ok>, Error<GetAlliancesAllianceIdContactsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_alliance_id = alliance_id;
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;
    let p_page = page;
    let p_token = token;

    let uri_str = format!("{}/alliances/{alliance_id}/contacts/", configuration.base_path, alliance_id=p_alliance_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_token {
        req_builder = req_builder.query(&[("token", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GetAlliancesAllianceIdContacts200Ok&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::GetAlliancesAllianceIdContacts200Ok&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAlliancesAllianceIdContactsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return custom labels for an alliance's contacts  --- Alternate route: `/dev/alliances/{alliance_id}/contacts/labels/`  Alternate route: `/legacy/alliances/{alliance_id}/contacts/labels/`  Alternate route: `/v1/alliances/{alliance_id}/contacts/labels/`  --- This route is cached for up to 300 seconds
pub async fn get_alliances_alliance_id_contacts_labels(configuration: &configuration::Configuration, alliance_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, token: Option<&str>) -> Result<Vec<models::GetAlliancesAllianceIdContactsLabels200Ok>, Error<GetAlliancesAllianceIdContactsLabelsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_alliance_id = alliance_id;
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;
    let p_token = token;

    let uri_str = format!("{}/alliances/{alliance_id}/contacts/labels/", configuration.base_path, alliance_id=p_alliance_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_token {
        req_builder = req_builder.query(&[("token", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GetAlliancesAllianceIdContactsLabels200Ok&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::GetAlliancesAllianceIdContactsLabels200Ok&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAlliancesAllianceIdContactsLabelsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return contacts of a character  --- Alternate route: `/dev/characters/{character_id}/contacts/`  Alternate route: `/v2/characters/{character_id}/contacts/`  --- This route is cached for up to 300 seconds
pub async fn get_characters_character_id_contacts(configuration: &configuration::Configuration, character_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, page: Option<i32>, token: Option<&str>) -> Result<Vec<models::GetCharactersCharacterIdContacts200Ok>, Error<GetCharactersCharacterIdContactsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_character_id = character_id;
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;
    let p_page = page;
    let p_token = token;

    let uri_str = format!("{}/characters/{character_id}/contacts/", configuration.base_path, character_id=p_character_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_token {
        req_builder = req_builder.query(&[("token", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GetCharactersCharacterIdContacts200Ok&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::GetCharactersCharacterIdContacts200Ok&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCharactersCharacterIdContactsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return custom labels for a character's contacts  --- Alternate route: `/dev/characters/{character_id}/contacts/labels/`  Alternate route: `/legacy/characters/{character_id}/contacts/labels/`  Alternate route: `/v1/characters/{character_id}/contacts/labels/`  --- This route is cached for up to 300 seconds
pub async fn get_characters_character_id_contacts_labels(configuration: &configuration::Configuration, character_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, token: Option<&str>) -> Result<Vec<models::GetCharactersCharacterIdContactsLabels200Ok>, Error<GetCharactersCharacterIdContactsLabelsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_character_id = character_id;
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;
    let p_token = token;

    let uri_str = format!("{}/characters/{character_id}/contacts/labels/", configuration.base_path, character_id=p_character_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_token {
        req_builder = req_builder.query(&[("token", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GetCharactersCharacterIdContactsLabels200Ok&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::GetCharactersCharacterIdContactsLabels200Ok&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCharactersCharacterIdContactsLabelsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return contacts of a corporation  --- Alternate route: `/dev/corporations/{corporation_id}/contacts/`  Alternate route: `/v2/corporations/{corporation_id}/contacts/`  --- This route is cached for up to 300 seconds
pub async fn get_corporations_corporation_id_contacts(configuration: &configuration::Configuration, corporation_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, page: Option<i32>, token: Option<&str>) -> Result<Vec<models::GetCorporationsCorporationIdContacts200Ok>, Error<GetCorporationsCorporationIdContactsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_corporation_id = corporation_id;
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;
    let p_page = page;
    let p_token = token;

    let uri_str = format!("{}/corporations/{corporation_id}/contacts/", configuration.base_path, corporation_id=p_corporation_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_token {
        req_builder = req_builder.query(&[("token", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GetCorporationsCorporationIdContacts200Ok&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::GetCorporationsCorporationIdContacts200Ok&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCorporationsCorporationIdContactsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return custom labels for a corporation's contacts  --- Alternate route: `/dev/corporations/{corporation_id}/contacts/labels/`  Alternate route: `/legacy/corporations/{corporation_id}/contacts/labels/`  Alternate route: `/v1/corporations/{corporation_id}/contacts/labels/`  --- This route is cached for up to 300 seconds
pub async fn get_corporations_corporation_id_contacts_labels(configuration: &configuration::Configuration, corporation_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, token: Option<&str>) -> Result<Vec<models::GetCorporationsCorporationIdContactsLabels200Ok>, Error<GetCorporationsCorporationIdContactsLabelsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_corporation_id = corporation_id;
    let p_datasource = datasource;
    let p_if_none_match = if_none_match;
    let p_token = token;

    let uri_str = format!("{}/corporations/{corporation_id}/contacts/labels/", configuration.base_path, corporation_id=p_corporation_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_token {
        req_builder = req_builder.query(&[("token", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_if_none_match {
        req_builder = req_builder.header("If-None-Match", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GetCorporationsCorporationIdContactsLabels200Ok&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::GetCorporationsCorporationIdContactsLabels200Ok&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCorporationsCorporationIdContactsLabelsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Bulk add contacts with same settings  --- Alternate route: `/dev/characters/{character_id}/contacts/`  Alternate route: `/v2/characters/{character_id}/contacts/` 
pub async fn post_characters_character_id_contacts(configuration: &configuration::Configuration, character_id: i32, standing: f32, contact_ids: Vec<i32>, datasource: Option<&str>, label_ids: Option<Vec<i64>>, token: Option<&str>, watched: Option<bool>) -> Result<Vec<i32>, Error<PostCharactersCharacterIdContactsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_character_id = character_id;
    let p_standing = standing;
    let p_contact_ids = contact_ids;
    let p_datasource = datasource;
    let p_label_ids = label_ids;
    let p_token = token;
    let p_watched = watched;

    let uri_str = format!("{}/characters/{character_id}/contacts/", configuration.base_path, character_id=p_character_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_label_ids {
        req_builder = match "csv" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("label_ids".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("label_ids", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    req_builder = req_builder.query(&[("standing", &p_standing.to_string())]);
    if let Some(ref param_value) = p_token {
        req_builder = req_builder.query(&[("token", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_watched {
        req_builder = req_builder.query(&[("watched", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_contact_ids);

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
        let entity: Option<PostCharactersCharacterIdContactsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Bulk edit contacts with same settings  --- Alternate route: `/dev/characters/{character_id}/contacts/`  Alternate route: `/v2/characters/{character_id}/contacts/` 
pub async fn put_characters_character_id_contacts(configuration: &configuration::Configuration, character_id: i32, standing: f32, contact_ids: Vec<i32>, datasource: Option<&str>, label_ids: Option<Vec<i64>>, token: Option<&str>, watched: Option<bool>) -> Result<(), Error<PutCharactersCharacterIdContactsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_character_id = character_id;
    let p_standing = standing;
    let p_contact_ids = contact_ids;
    let p_datasource = datasource;
    let p_label_ids = label_ids;
    let p_token = token;
    let p_watched = watched;

    let uri_str = format!("{}/characters/{character_id}/contacts/", configuration.base_path, character_id=p_character_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref param_value) = p_datasource {
        req_builder = req_builder.query(&[("datasource", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_label_ids {
        req_builder = match "csv" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("label_ids".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("label_ids", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    req_builder = req_builder.query(&[("standing", &p_standing.to_string())]);
    if let Some(ref param_value) = p_token {
        req_builder = req_builder.query(&[("token", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_watched {
        req_builder = req_builder.query(&[("watched", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_contact_ids);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PutCharactersCharacterIdContactsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

