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

/// GetCharactersCharacterIdFwStatsKills : Summary of kills done by the given character against enemy factions
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdFwStatsKills {
    /// Last week's total number of kills by a given character against enemy factions
    #[serde(rename = "last_week")]
    pub last_week: i32,
    /// Total number of kills by a given character against enemy factions since the character enlisted
    #[serde(rename = "total")]
    pub total: i32,
    /// Yesterday's total number of kills by a given character against enemy factions
    #[serde(rename = "yesterday")]
    pub yesterday: i32,
}

impl GetCharactersCharacterIdFwStatsKills {
    /// Summary of kills done by the given character against enemy factions
    pub fn new(last_week: i32, total: i32, yesterday: i32) -> GetCharactersCharacterIdFwStatsKills {
        GetCharactersCharacterIdFwStatsKills {
            last_week,
            total,
            yesterday,
        }
    }
}

