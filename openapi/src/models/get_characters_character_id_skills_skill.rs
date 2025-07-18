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

/// GetCharactersCharacterIdSkillsSkill : skill object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdSkillsSkill {
    /// active_skill_level integer
    #[serde(rename = "active_skill_level")]
    pub active_skill_level: i32,
    /// skill_id integer
    #[serde(rename = "skill_id")]
    pub skill_id: i32,
    /// skillpoints_in_skill integer
    #[serde(rename = "skillpoints_in_skill")]
    pub skillpoints_in_skill: i64,
    /// trained_skill_level integer
    #[serde(rename = "trained_skill_level")]
    pub trained_skill_level: i32,
}

impl GetCharactersCharacterIdSkillsSkill {
    /// skill object
    pub fn new(active_skill_level: i32, skill_id: i32, skillpoints_in_skill: i64, trained_skill_level: i32) -> GetCharactersCharacterIdSkillsSkill {
        GetCharactersCharacterIdSkillsSkill {
            active_skill_level,
            skill_id,
            skillpoints_in_skill,
            trained_skill_level,
        }
    }
}

