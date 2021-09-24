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
}

pub struct AchievementEarnedProperties {
   character_id: u64,
   timestamp: u64,
   world_id: u8,
   zone_id: u8,
   achievement_id: u32,
}

pub struct BattleRankUpProperties {
    character_id: u64,
    timestamp: u64,
    world_id: u8,
    zone_id: u8,
    battle_rank: u8,
}

pub struct DeathProperties {
    character_id: u64,
    timestamp: u64,
    world_id: u8,
    zone_id: u8,
    attacker_id: u64,
    attacker_fire_mode_id: u8,
    attacker_loadout_id: u8,
    attacker_vehicle_id: u8,
    attacker_weapon_id: u32,
    loadout_id: u8,
    critical: bool,
    headshot: bool,
    vehicle_id: u8,
}

pub struct ItemAddedProperties {
    character_id: u64,
    timestamp: u64,
    world_id: u8,
    zone_id: u8,
    context: String,
    item_count: i32,
    item_id: u32,
}

pub struct MetagameEventProperties {
    timestamp: u64,
    world_id: u8,
    zone_id: u8,
    experience_bonus: f32,
    faction_nc: f32,
    faction_tr: f32,
    faction_vs: f32,
    metagame_event_id: i32,
    metagame_event_state: i32,
}

pub struct PlayerFacilityCaptureProperties {
    character_id: u64,
    timestamp: u64,
    world_id: u8,
    zone_id: u8,
    facility_id: u32,
    outfit_id: u64,
}

pub struct PlayerFacilityDefendProperties {
    character_id: u64,
    timestamp: u64,
    world_id: u8,
    zone_id: u8,
    facility_id: u32,
    outfit_id: u64,
}

pub struct PlayerLoginProperties {
    character_id: u64,
    timestamp: u64,
    world_id: u8,
}

pub struct PlayerLogoutProperties {
    character_id: u64,
    timestamp: u64,
    world_id: u8,
}

pub struct SkillAddedProperties {
    character_id: u64,
    timestamp: u64,
    world_id: u8,
    zone_id: u8,
    skill_id: i32,
}

pub struct VehicleDestroyProperties {
    character_id: u64,
    timestamp: u64,
    world_id: u8,
    zone_id: u8,
    attacker_character_id: u64,
    attacker_loadout_id: u8,
    attacker_vehicle_id: u8,
    attacker_weapon_id: u32,
    facility_id: u32,
    faction_id: u8,
    vehicle_id: u32,
}

pub struct FacilityControlProperties {
    timestamp: u64,
    world_id: u8,
    zone_id: u8,
    facility_id: u32,
    duration_held: u32,
    old_faction_id: u8,
    new_faction_id: u8,
    outfit_id: u64,
}
