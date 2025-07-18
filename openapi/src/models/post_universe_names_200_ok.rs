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

/// PostUniverseNames200Ok : 200 ok object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostUniverseNames200Ok {
    /// category string
    #[serde(rename = "category")]
    pub category: Category,
    /// id integer
    #[serde(rename = "id")]
    pub id: i32,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
}

impl PostUniverseNames200Ok {
    /// 200 ok object
    pub fn new(category: Category, id: i32, name: String) -> PostUniverseNames200Ok {
        PostUniverseNames200Ok {
            category,
            id,
            name,
        }
    }
}
/// category string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "alliance")]
    Alliance,
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "constellation")]
    Constellation,
    #[serde(rename = "corporation")]
    Corporation,
    #[serde(rename = "inventory_type")]
    InventoryType,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "solar_system")]
    SolarSystem,
    #[serde(rename = "station")]
    Station,
    #[serde(rename = "faction")]
    Faction,
}

impl Default for Category {
    fn default() -> Category {
        Self::Alliance
    }
}

