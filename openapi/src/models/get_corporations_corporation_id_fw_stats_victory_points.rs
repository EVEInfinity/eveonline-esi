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

/// GetCorporationsCorporationIdFwStatsVictoryPoints : Summary of victory points gained by the given corporation for the enlisted faction
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdFwStatsVictoryPoints {
    /// Last week's victory points gained by members of the given corporation
    #[serde(rename = "last_week")]
    pub last_week: i32,
    /// Total victory points gained since the given corporation enlisted
    #[serde(rename = "total")]
    pub total: i32,
    /// Yesterday's victory points gained by members of the given corporation
    #[serde(rename = "yesterday")]
    pub yesterday: i32,
}

impl GetCorporationsCorporationIdFwStatsVictoryPoints {
    /// Summary of victory points gained by the given corporation for the enlisted faction
    pub fn new(last_week: i32, total: i32, yesterday: i32) -> GetCorporationsCorporationIdFwStatsVictoryPoints {
        GetCorporationsCorporationIdFwStatsVictoryPoints {
            last_week,
            total,
            yesterday,
        }
    }
}

