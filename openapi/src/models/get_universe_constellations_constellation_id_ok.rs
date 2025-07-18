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

/// GetUniverseConstellationsConstellationIdOk : 200 ok object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUniverseConstellationsConstellationIdOk {
    /// constellation_id integer
    #[serde(rename = "constellation_id")]
    pub constellation_id: i32,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "position")]
    pub position: Box<models::GetUniverseConstellationsConstellationIdPosition>,
    /// The region this constellation is in
    #[serde(rename = "region_id")]
    pub region_id: i32,
    /// systems array
    #[serde(rename = "systems")]
    pub systems: Vec<i32>,
}

impl GetUniverseConstellationsConstellationIdOk {
    /// 200 ok object
    pub fn new(constellation_id: i32, name: String, position: models::GetUniverseConstellationsConstellationIdPosition, region_id: i32, systems: Vec<i32>) -> GetUniverseConstellationsConstellationIdOk {
        GetUniverseConstellationsConstellationIdOk {
            constellation_id,
            name,
            position: Box::new(position),
            region_id,
            systems,
        }
    }
}

