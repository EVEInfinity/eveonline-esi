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

/// GetCharactersCharacterIdAssets200Ok : 200 ok object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdAssets200Ok {
    /// is_blueprint_copy boolean
    #[serde(rename = "is_blueprint_copy", skip_serializing_if = "Option::is_none")]
    pub is_blueprint_copy: Option<bool>,
    /// is_singleton boolean
    #[serde(rename = "is_singleton")]
    pub is_singleton: bool,
    /// item_id integer
    #[serde(rename = "item_id")]
    pub item_id: i64,
    /// location_flag string
    #[serde(rename = "location_flag")]
    pub location_flag: LocationFlag,
    /// location_id integer
    #[serde(rename = "location_id")]
    pub location_id: i64,
    /// location_type string
    #[serde(rename = "location_type")]
    pub location_type: LocationType,
    /// quantity integer
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetCharactersCharacterIdAssets200Ok {
    /// 200 ok object
    pub fn new(is_singleton: bool, item_id: i64, location_flag: LocationFlag, location_id: i64, location_type: LocationType, quantity: i32, type_id: i32) -> GetCharactersCharacterIdAssets200Ok {
        GetCharactersCharacterIdAssets200Ok {
            is_blueprint_copy: None,
            is_singleton,
            item_id,
            location_flag,
            location_id,
            location_type,
            quantity,
            type_id,
        }
    }
}
/// location_flag string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationFlag {
    #[serde(rename = "AssetSafety")]
    AssetSafety,
    #[serde(rename = "AutoFit")]
    AutoFit,
    #[serde(rename = "BoosterBay")]
    BoosterBay,
    #[serde(rename = "Cargo")]
    Cargo,
    #[serde(rename = "CorporationGoalDeliveries")]
    CorporationGoalDeliveries,
    #[serde(rename = "CorpseBay")]
    CorpseBay,
    #[serde(rename = "Deliveries")]
    Deliveries,
    #[serde(rename = "DroneBay")]
    DroneBay,
    #[serde(rename = "FighterBay")]
    FighterBay,
    #[serde(rename = "FighterTube0")]
    FighterTube0,
    #[serde(rename = "FighterTube1")]
    FighterTube1,
    #[serde(rename = "FighterTube2")]
    FighterTube2,
    #[serde(rename = "FighterTube3")]
    FighterTube3,
    #[serde(rename = "FighterTube4")]
    FighterTube4,
    #[serde(rename = "FleetHangar")]
    FleetHangar,
    #[serde(rename = "FrigateEscapeBay")]
    FrigateEscapeBay,
    #[serde(rename = "Hangar")]
    Hangar,
    #[serde(rename = "HangarAll")]
    HangarAll,
    #[serde(rename = "HiSlot0")]
    HiSlot0,
    #[serde(rename = "HiSlot1")]
    HiSlot1,
    #[serde(rename = "HiSlot2")]
    HiSlot2,
    #[serde(rename = "HiSlot3")]
    HiSlot3,
    #[serde(rename = "HiSlot4")]
    HiSlot4,
    #[serde(rename = "HiSlot5")]
    HiSlot5,
    #[serde(rename = "HiSlot6")]
    HiSlot6,
    #[serde(rename = "HiSlot7")]
    HiSlot7,
    #[serde(rename = "HiddenModifiers")]
    HiddenModifiers,
    #[serde(rename = "Implant")]
    Implant,
    #[serde(rename = "InfrastructureHangar")]
    InfrastructureHangar,
    #[serde(rename = "LoSlot0")]
    LoSlot0,
    #[serde(rename = "LoSlot1")]
    LoSlot1,
    #[serde(rename = "LoSlot2")]
    LoSlot2,
    #[serde(rename = "LoSlot3")]
    LoSlot3,
    #[serde(rename = "LoSlot4")]
    LoSlot4,
    #[serde(rename = "LoSlot5")]
    LoSlot5,
    #[serde(rename = "LoSlot6")]
    LoSlot6,
    #[serde(rename = "LoSlot7")]
    LoSlot7,
    #[serde(rename = "Locked")]
    Locked,
    #[serde(rename = "MedSlot0")]
    MedSlot0,
    #[serde(rename = "MedSlot1")]
    MedSlot1,
    #[serde(rename = "MedSlot2")]
    MedSlot2,
    #[serde(rename = "MedSlot3")]
    MedSlot3,
    #[serde(rename = "MedSlot4")]
    MedSlot4,
    #[serde(rename = "MedSlot5")]
    MedSlot5,
    #[serde(rename = "MedSlot6")]
    MedSlot6,
    #[serde(rename = "MedSlot7")]
    MedSlot7,
    #[serde(rename = "MobileDepotHold")]
    MobileDepotHold,
    #[serde(rename = "MoonMaterialBay")]
    MoonMaterialBay,
    #[serde(rename = "QuafeBay")]
    QuafeBay,
    #[serde(rename = "RigSlot0")]
    RigSlot0,
    #[serde(rename = "RigSlot1")]
    RigSlot1,
    #[serde(rename = "RigSlot2")]
    RigSlot2,
    #[serde(rename = "RigSlot3")]
    RigSlot3,
    #[serde(rename = "RigSlot4")]
    RigSlot4,
    #[serde(rename = "RigSlot5")]
    RigSlot5,
    #[serde(rename = "RigSlot6")]
    RigSlot6,
    #[serde(rename = "RigSlot7")]
    RigSlot7,
    #[serde(rename = "ShipHangar")]
    ShipHangar,
    #[serde(rename = "Skill")]
    Skill,
    #[serde(rename = "SpecializedAmmoHold")]
    SpecializedAmmoHold,
    #[serde(rename = "SpecializedAsteroidHold")]
    SpecializedAsteroidHold,
    #[serde(rename = "SpecializedCommandCenterHold")]
    SpecializedCommandCenterHold,
    #[serde(rename = "SpecializedFuelBay")]
    SpecializedFuelBay,
    #[serde(rename = "SpecializedGasHold")]
    SpecializedGasHold,
    #[serde(rename = "SpecializedIceHold")]
    SpecializedIceHold,
    #[serde(rename = "SpecializedIndustrialShipHold")]
    SpecializedIndustrialShipHold,
    #[serde(rename = "SpecializedLargeShipHold")]
    SpecializedLargeShipHold,
    #[serde(rename = "SpecializedMaterialBay")]
    SpecializedMaterialBay,
    #[serde(rename = "SpecializedMediumShipHold")]
    SpecializedMediumShipHold,
    #[serde(rename = "SpecializedMineralHold")]
    SpecializedMineralHold,
    #[serde(rename = "SpecializedOreHold")]
    SpecializedOreHold,
    #[serde(rename = "SpecializedPlanetaryCommoditiesHold")]
    SpecializedPlanetaryCommoditiesHold,
    #[serde(rename = "SpecializedSalvageHold")]
    SpecializedSalvageHold,
    #[serde(rename = "SpecializedShipHold")]
    SpecializedShipHold,
    #[serde(rename = "SpecializedSmallShipHold")]
    SpecializedSmallShipHold,
    #[serde(rename = "StructureDeedBay")]
    StructureDeedBay,
    #[serde(rename = "SubSystemBay")]
    SubSystemBay,
    #[serde(rename = "SubSystemSlot0")]
    SubSystemSlot0,
    #[serde(rename = "SubSystemSlot1")]
    SubSystemSlot1,
    #[serde(rename = "SubSystemSlot2")]
    SubSystemSlot2,
    #[serde(rename = "SubSystemSlot3")]
    SubSystemSlot3,
    #[serde(rename = "SubSystemSlot4")]
    SubSystemSlot4,
    #[serde(rename = "SubSystemSlot5")]
    SubSystemSlot5,
    #[serde(rename = "SubSystemSlot6")]
    SubSystemSlot6,
    #[serde(rename = "SubSystemSlot7")]
    SubSystemSlot7,
    #[serde(rename = "Unlocked")]
    Unlocked,
    #[serde(rename = "Wardrobe")]
    Wardrobe,
}

impl Default for LocationFlag {
    fn default() -> LocationFlag {
        Self::AssetSafety
    }
}
/// location_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationType {
    #[serde(rename = "station")]
    Station,
    #[serde(rename = "solar_system")]
    SolarSystem,
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "other")]
    Other,
}

impl Default for LocationType {
    fn default() -> LocationType {
        Self::Station
    }
}

