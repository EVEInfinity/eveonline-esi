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

/// GetFwLeaderboardsActiveTotalActiveTotal1 : active_total object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFwLeaderboardsActiveTotalActiveTotal1 {
    /// Amount of victory points
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// faction_id integer
    #[serde(rename = "faction_id", skip_serializing_if = "Option::is_none")]
    pub faction_id: Option<i32>,
}

impl GetFwLeaderboardsActiveTotalActiveTotal1 {
    /// active_total object
    pub fn new() -> GetFwLeaderboardsActiveTotalActiveTotal1 {
        GetFwLeaderboardsActiveTotalActiveTotal1 {
            amount: None,
            faction_id: None,
        }
    }
}

