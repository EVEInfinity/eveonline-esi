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

/// GetAlliancesAllianceIdIconsOk : 200 ok object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAlliancesAllianceIdIconsOk {
    /// px128x128 string
    #[serde(rename = "px128x128", skip_serializing_if = "Option::is_none")]
    pub px128x128: Option<String>,
    /// px64x64 string
    #[serde(rename = "px64x64", skip_serializing_if = "Option::is_none")]
    pub px64x64: Option<String>,
}

impl GetAlliancesAllianceIdIconsOk {
    /// 200 ok object
    pub fn new() -> GetAlliancesAllianceIdIconsOk {
        GetAlliancesAllianceIdIconsOk {
            px128x128: None,
            px64x64: None,
        }
    }
}

