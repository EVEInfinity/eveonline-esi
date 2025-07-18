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

/// PostUniverseIdsInventoryType : inventory_type object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostUniverseIdsInventoryType {
    /// id integer
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostUniverseIdsInventoryType {
    /// inventory_type object
    pub fn new() -> PostUniverseIdsInventoryType {
        PostUniverseIdsInventoryType {
            id: None,
            name: None,
        }
    }
}

