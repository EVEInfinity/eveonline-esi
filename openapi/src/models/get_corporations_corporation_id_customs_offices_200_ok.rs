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

/// GetCorporationsCorporationIdCustomsOffices200Ok : 200 ok object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdCustomsOffices200Ok {
    /// Only present if alliance access is allowed
    #[serde(rename = "alliance_tax_rate", skip_serializing_if = "Option::is_none")]
    pub alliance_tax_rate: Option<f32>,
    /// standing_level and any standing related tax rate only present when this is true
    #[serde(rename = "allow_access_with_standings")]
    pub allow_access_with_standings: bool,
    /// allow_alliance_access boolean
    #[serde(rename = "allow_alliance_access")]
    pub allow_alliance_access: bool,
    /// bad_standing_tax_rate number
    #[serde(rename = "bad_standing_tax_rate", skip_serializing_if = "Option::is_none")]
    pub bad_standing_tax_rate: Option<f32>,
    /// corporation_tax_rate number
    #[serde(rename = "corporation_tax_rate", skip_serializing_if = "Option::is_none")]
    pub corporation_tax_rate: Option<f32>,
    /// Tax rate for entities with excellent level of standing, only present if this level is allowed, same for all other standing related tax rates
    #[serde(rename = "excellent_standing_tax_rate", skip_serializing_if = "Option::is_none")]
    pub excellent_standing_tax_rate: Option<f32>,
    /// good_standing_tax_rate number
    #[serde(rename = "good_standing_tax_rate", skip_serializing_if = "Option::is_none")]
    pub good_standing_tax_rate: Option<f32>,
    /// neutral_standing_tax_rate number
    #[serde(rename = "neutral_standing_tax_rate", skip_serializing_if = "Option::is_none")]
    pub neutral_standing_tax_rate: Option<f32>,
    /// unique ID of this customs office
    #[serde(rename = "office_id")]
    pub office_id: i64,
    /// reinforce_exit_end integer
    #[serde(rename = "reinforce_exit_end")]
    pub reinforce_exit_end: i32,
    /// Together with reinforce_exit_end, marks a 2-hour period where this customs office could exit reinforcement mode during the day after initial attack
    #[serde(rename = "reinforce_exit_start")]
    pub reinforce_exit_start: i32,
    /// Access is allowed only for entities with this level of standing or better
    #[serde(rename = "standing_level", skip_serializing_if = "Option::is_none")]
    pub standing_level: Option<StandingLevel>,
    /// ID of the solar system this customs office is located in
    #[serde(rename = "system_id")]
    pub system_id: i32,
    /// terrible_standing_tax_rate number
    #[serde(rename = "terrible_standing_tax_rate", skip_serializing_if = "Option::is_none")]
    pub terrible_standing_tax_rate: Option<f32>,
}

impl GetCorporationsCorporationIdCustomsOffices200Ok {
    /// 200 ok object
    pub fn new(allow_access_with_standings: bool, allow_alliance_access: bool, office_id: i64, reinforce_exit_end: i32, reinforce_exit_start: i32, system_id: i32) -> GetCorporationsCorporationIdCustomsOffices200Ok {
        GetCorporationsCorporationIdCustomsOffices200Ok {
            alliance_tax_rate: None,
            allow_access_with_standings,
            allow_alliance_access,
            bad_standing_tax_rate: None,
            corporation_tax_rate: None,
            excellent_standing_tax_rate: None,
            good_standing_tax_rate: None,
            neutral_standing_tax_rate: None,
            office_id,
            reinforce_exit_end,
            reinforce_exit_start,
            standing_level: None,
            system_id,
            terrible_standing_tax_rate: None,
        }
    }
}
/// Access is allowed only for entities with this level of standing or better
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StandingLevel {
    #[serde(rename = "bad")]
    Bad,
    #[serde(rename = "excellent")]
    Excellent,
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "terrible")]
    Terrible,
}

impl Default for StandingLevel {
    fn default() -> StandingLevel {
        Self::Bad
    }
}

