use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SocketMessage {
    pub service: String,
    #[serde(rename = "type")]
    pub message_type: String,
    // TODO heartbeat fields
    // TODO stateChanged fields
    pub payload: Option<SocketEvent>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event_name")]
pub enum SocketEvent {
    AchievementEarned(AchievementEarnedProperties),
    BattleRankUp(BattleRankUpProperties),
    Death(DeathProperties),
    ItemAdded(ItemAddedProperties),
    MetagameEvent(MetagameEventProperties),
    PlayerFacilityCapture(PlayerFacilityCaptureProperties),
    PlayerFacilityDefend(PlayerFacilityDefendProperties),
    PlayerLogin(PlayerLoginProperties),
    PlayerLogout(PlayerLogoutProperties),
    SkillAdded(SkillAddedProperties),
    VehicleDestroy(VehicleDestroyProperties),
    FacilityControl(FacilityControlProperties),
    Unknown(String)
}

#[derive(Debug, Deserialize)]
pub struct AchievementEarnedProperties {
   character_id: String,
   timestamp: String,
   world_id: String,
   zone_id: String,
   achievement_id: String,
}

#[derive(Debug, Deserialize)]
pub struct BattleRankUpProperties {
    character_id: String,
    timestamp: String,
    world_id: String,
    zone_id: String,
    battle_rank: String,
}

#[derive(Debug, Deserialize)]
pub struct DeathProperties {
    character_id: String,
    timestamp: String,
    world_id: String,
    zone_id: String,
    attacker_character_id: String,
    attacker_fire_mode_id: String,
    attacker_loadout_id: String,
    attacker_vehicle_id: String,
    attacker_weapon_id: String,
    loadout_id: String,
    critical: String,
    headshot: String,
    vehicle_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemAddedProperties {
    character_id: String,
    timestamp: String,
    world_id: String,
    zone_id: String,
    context: String,
    item_count: String,
    item_id: String,
}

#[derive(Debug, Deserialize)]
pub struct MetagameEventProperties {
    timestamp: String,
    world_id: String,
    zone_id: String,
    experience_bonus: String,
    faction_nc: String,
    faction_tr: String,
    faction_vs: String,
    metagame_event_id: String,
    metagame_event_state: String,
}

#[derive(Debug, Deserialize)]
pub struct PlayerFacilityCaptureProperties {
    character_id: String,
    timestamp: String,
    world_id: String,
    zone_id: String,
    facility_id: String,
    outfit_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PlayerFacilityDefendProperties {
    character_id: String,
    timestamp: String,
    world_id: String,
    zone_id: String,
    facility_id: String,
    outfit_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PlayerLoginProperties {
    pub character_id: String,
    pub timestamp: String,
    pub world_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PlayerLogoutProperties {
    character_id: String,
    timestamp: String,
    world_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SkillAddedProperties {
    character_id: String,
    timestamp: String,
    world_id: String,
    zone_id: String,
    skill_id: String,
}

#[derive(Debug, Deserialize)]
pub struct VehicleDestroyProperties {
    character_id: String,
    timestamp: String,
    world_id: String,
    zone_id: String,
    attacker_character_id: String,
    attacker_loadout_id: String,
    attacker_vehicle_id: String,
    attacker_weapon_id: String,
    facility_id: String,
    faction_id: String,
    vehicle_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FacilityControlProperties {
    timestamp: String,
    world_id: String,
    zone_id: String,
    facility_id: String,
    duration_held: String,
    old_faction_id: String,
    new_faction_id: String,
    outfit_id: String,
}
