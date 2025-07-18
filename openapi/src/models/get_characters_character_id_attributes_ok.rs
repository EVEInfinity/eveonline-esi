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

/// GetCharactersCharacterIdAttributesOk : 200 ok object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdAttributesOk {
    /// Neural remapping cooldown after a character uses remap accrued over time
    #[serde(rename = "accrued_remap_cooldown_date", skip_serializing_if = "Option::is_none")]
    pub accrued_remap_cooldown_date: Option<String>,
    /// Number of available bonus character neural remaps
    #[serde(rename = "bonus_remaps", skip_serializing_if = "Option::is_none")]
    pub bonus_remaps: Option<i32>,
    /// charisma integer
    #[serde(rename = "charisma")]
    pub charisma: i32,
    /// intelligence integer
    #[serde(rename = "intelligence")]
    pub intelligence: i32,
    /// Datetime of last neural remap, including usage of bonus remaps
    #[serde(rename = "last_remap_date", skip_serializing_if = "Option::is_none")]
    pub last_remap_date: Option<String>,
    /// memory integer
    #[serde(rename = "memory")]
    pub memory: i32,
    /// perception integer
    #[serde(rename = "perception")]
    pub perception: i32,
    /// willpower integer
    #[serde(rename = "willpower")]
    pub willpower: i32,
}

impl GetCharactersCharacterIdAttributesOk {
    /// 200 ok object
    pub fn new(charisma: i32, intelligence: i32, memory: i32, perception: i32, willpower: i32) -> GetCharactersCharacterIdAttributesOk {
        GetCharactersCharacterIdAttributesOk {
            accrued_remap_cooldown_date: None,
            bonus_remaps: None,
            charisma,
            intelligence,
            last_remap_date: None,
            memory,
            perception,
            willpower,
        }
    }
}

