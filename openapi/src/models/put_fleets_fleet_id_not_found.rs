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

/// PutFleetsFleetIdNotFound : Not found
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutFleetsFleetIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl PutFleetsFleetIdNotFound {
    /// Not found
    pub fn new() -> PutFleetsFleetIdNotFound {
        PutFleetsFleetIdNotFound {
            error: None,
        }
    }
}

