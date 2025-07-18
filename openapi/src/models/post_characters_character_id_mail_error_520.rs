/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.33
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PostCharactersCharacterIdMailError520 : Error 520
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCharactersCharacterIdMailError520 {
    /// Error 520 message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl PostCharactersCharacterIdMailError520 {
    /// Error 520
    pub fn new() -> PostCharactersCharacterIdMailError520 {
        PostCharactersCharacterIdMailError520 {
            error: None,
        }
    }
}

