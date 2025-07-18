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

/// GetCorporationsCorporationIdContactsLabels200Ok : 200 ok object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdContactsLabels200Ok {
    /// label_id integer
    #[serde(rename = "label_id")]
    pub label_id: i64,
    /// label_name string
    #[serde(rename = "label_name")]
    pub label_name: String,
}

impl GetCorporationsCorporationIdContactsLabels200Ok {
    /// 200 ok object
    pub fn new(label_id: i64, label_name: String) -> GetCorporationsCorporationIdContactsLabels200Ok {
        GetCorporationsCorporationIdContactsLabels200Ok {
            label_id,
            label_name,
        }
    }
}

