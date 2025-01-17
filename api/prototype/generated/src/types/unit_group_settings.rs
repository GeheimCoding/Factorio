#[derive(serde::Deserialize)]
pub struct UnitGroupSettings {
    max_gathering_unit_groups: u32,
    max_group_gathering_time: u32,
    max_group_member_fallback_factor: f64,
    max_group_radius: f64,
    max_group_slowdown_factor: f64,
    max_member_slowdown_when_ahead: f64,
    max_member_speedup_when_behind: f64,
    max_unit_group_size: u32,
    max_wait_time_for_late_members: u32,
    member_disown_distance: f64,
    min_group_gathering_time: u32,
    min_group_radius: f64,
    tick_tolerance_when_member_arrives: u32,
}
