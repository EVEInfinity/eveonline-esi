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

/// GetFwLeaderboardsCharactersVictoryPoints : Top 100 rankings of pilots by victory points from yesterday, last week and in total
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCharactersVictoryPoints {
    /// Top 100 ranking of pilots active in faction warfare by total victory points. A pilot is considered \"active\" if they have participated in faction warfare in the past 14 days
    #[serde(rename = "active_total")]
    pub active_total: Vec<models::GetFwLeaderboardsCharactersActiveTotalActiveTotal1>,
    /// Top 100 ranking of pilots by victory points in the past week
    #[serde(rename = "last_week")]
    pub last_week: Vec<models::GetFwLeaderboardsCharactersLastWeekLastWeek1>,
    /// Top 100 ranking of pilots by victory points in the past day
    #[serde(rename = "yesterday")]
    pub yesterday: Vec<models::GetFwLeaderboardsCharactersYesterdayYesterday1>,
}

impl GetFwLeaderboardsCharactersVictoryPoints {
    /// Top 100 rankings of pilots by victory points from yesterday, last week and in total
    pub fn new(active_total: Vec<models::GetFwLeaderboardsCharactersActiveTotalActiveTotal1>, last_week: Vec<models::GetFwLeaderboardsCharactersLastWeekLastWeek1>, yesterday: Vec<models::GetFwLeaderboardsCharactersYesterdayYesterday1>) -> GetFwLeaderboardsCharactersVictoryPoints {
        GetFwLeaderboardsCharactersVictoryPoints {
            active_total,
            last_week,
            yesterday,
        }
    }
}

