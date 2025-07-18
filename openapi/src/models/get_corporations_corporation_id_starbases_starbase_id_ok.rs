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

/// GetCorporationsCorporationIdStarbasesStarbaseIdOk : 200 ok object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdStarbasesStarbaseIdOk {
    /// allow_alliance_members boolean
    #[serde(rename = "allow_alliance_members")]
    pub allow_alliance_members: bool,
    /// allow_corporation_members boolean
    #[serde(rename = "allow_corporation_members")]
    pub allow_corporation_members: bool,
    /// Who can anchor starbase (POS) and its structures
    #[serde(rename = "anchor")]
    pub anchor: Anchor,
    /// attack_if_at_war boolean
    #[serde(rename = "attack_if_at_war")]
    pub attack_if_at_war: bool,
    /// attack_if_other_security_status_dropping boolean
    #[serde(rename = "attack_if_other_security_status_dropping")]
    pub attack_if_other_security_status_dropping: bool,
    /// Starbase (POS) will attack if target's security standing is lower than this value
    #[serde(rename = "attack_security_status_threshold", skip_serializing_if = "Option::is_none")]
    pub attack_security_status_threshold: Option<f32>,
    /// Starbase (POS) will attack if target's standing is lower than this value
    #[serde(rename = "attack_standing_threshold", skip_serializing_if = "Option::is_none")]
    pub attack_standing_threshold: Option<f32>,
    /// Who can take fuel blocks out of the starbase (POS)'s fuel bay
    #[serde(rename = "fuel_bay_take")]
    pub fuel_bay_take: FuelBayTake,
    /// Who can view the starbase (POS)'s fule bay. Characters either need to have required role or belong to the starbase (POS) owner's corporation or alliance, as described by the enum, all other access settings follows the same scheme
    #[serde(rename = "fuel_bay_view")]
    pub fuel_bay_view: FuelBayView,
    /// Fuel blocks and other things that will be consumed when operating a starbase (POS)
    #[serde(rename = "fuels", skip_serializing_if = "Option::is_none")]
    pub fuels: Option<Vec<models::GetCorporationsCorporationIdStarbasesStarbaseIdFuel>>,
    /// Who can offline starbase (POS) and its structures
    #[serde(rename = "offline")]
    pub offline: Offline,
    /// Who can online starbase (POS) and its structures
    #[serde(rename = "online")]
    pub online: Online,
    /// Who can unanchor starbase (POS) and its structures
    #[serde(rename = "unanchor")]
    pub unanchor: Unanchor,
    /// True if the starbase (POS) is using alliance standings, otherwise using corporation's
    #[serde(rename = "use_alliance_standings")]
    pub use_alliance_standings: bool,
}

impl GetCorporationsCorporationIdStarbasesStarbaseIdOk {
    /// 200 ok object
    pub fn new(allow_alliance_members: bool, allow_corporation_members: bool, anchor: Anchor, attack_if_at_war: bool, attack_if_other_security_status_dropping: bool, fuel_bay_take: FuelBayTake, fuel_bay_view: FuelBayView, offline: Offline, online: Online, unanchor: Unanchor, use_alliance_standings: bool) -> GetCorporationsCorporationIdStarbasesStarbaseIdOk {
        GetCorporationsCorporationIdStarbasesStarbaseIdOk {
            allow_alliance_members,
            allow_corporation_members,
            anchor,
            attack_if_at_war,
            attack_if_other_security_status_dropping,
            attack_security_status_threshold: None,
            attack_standing_threshold: None,
            fuel_bay_take,
            fuel_bay_view,
            fuels: None,
            offline,
            online,
            unanchor,
            use_alliance_standings,
        }
    }
}
/// Who can anchor starbase (POS) and its structures
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Anchor {
    #[serde(rename = "alliance_member")]
    AllianceMember,
    #[serde(rename = "config_starbase_equipment_role")]
    ConfigStarbaseEquipmentRole,
    #[serde(rename = "corporation_member")]
    CorporationMember,
    #[serde(rename = "starbase_fuel_technician_role")]
    StarbaseFuelTechnicianRole,
}

impl Default for Anchor {
    fn default() -> Anchor {
        Self::AllianceMember
    }
}
/// Who can take fuel blocks out of the starbase (POS)'s fuel bay
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FuelBayTake {
    #[serde(rename = "alliance_member")]
    AllianceMember,
    #[serde(rename = "config_starbase_equipment_role")]
    ConfigStarbaseEquipmentRole,
    #[serde(rename = "corporation_member")]
    CorporationMember,
    #[serde(rename = "starbase_fuel_technician_role")]
    StarbaseFuelTechnicianRole,
}

impl Default for FuelBayTake {
    fn default() -> FuelBayTake {
        Self::AllianceMember
    }
}
/// Who can view the starbase (POS)'s fule bay. Characters either need to have required role or belong to the starbase (POS) owner's corporation or alliance, as described by the enum, all other access settings follows the same scheme
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FuelBayView {
    #[serde(rename = "alliance_member")]
    AllianceMember,
    #[serde(rename = "config_starbase_equipment_role")]
    ConfigStarbaseEquipmentRole,
    #[serde(rename = "corporation_member")]
    CorporationMember,
    #[serde(rename = "starbase_fuel_technician_role")]
    StarbaseFuelTechnicianRole,
}

impl Default for FuelBayView {
    fn default() -> FuelBayView {
        Self::AllianceMember
    }
}
/// Who can offline starbase (POS) and its structures
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Offline {
    #[serde(rename = "alliance_member")]
    AllianceMember,
    #[serde(rename = "config_starbase_equipment_role")]
    ConfigStarbaseEquipmentRole,
    #[serde(rename = "corporation_member")]
    CorporationMember,
    #[serde(rename = "starbase_fuel_technician_role")]
    StarbaseFuelTechnicianRole,
}

impl Default for Offline {
    fn default() -> Offline {
        Self::AllianceMember
    }
}
/// Who can online starbase (POS) and its structures
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Online {
    #[serde(rename = "alliance_member")]
    AllianceMember,
    #[serde(rename = "config_starbase_equipment_role")]
    ConfigStarbaseEquipmentRole,
    #[serde(rename = "corporation_member")]
    CorporationMember,
    #[serde(rename = "starbase_fuel_technician_role")]
    StarbaseFuelTechnicianRole,
}

impl Default for Online {
    fn default() -> Online {
        Self::AllianceMember
    }
}
/// Who can unanchor starbase (POS) and its structures
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Unanchor {
    #[serde(rename = "alliance_member")]
    AllianceMember,
    #[serde(rename = "config_starbase_equipment_role")]
    ConfigStarbaseEquipmentRole,
    #[serde(rename = "corporation_member")]
    CorporationMember,
    #[serde(rename = "starbase_fuel_technician_role")]
    StarbaseFuelTechnicianRole,
}

impl Default for Unanchor {
    fn default() -> Unanchor {
        Self::AllianceMember
    }
}

