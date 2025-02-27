#[derive(Debug, serde::Deserialize)]
pub struct PathFinderSettings {
    cache_accept_path_end_distance_ratio: f64,
    cache_accept_path_start_distance_ratio: f64,
    cache_max_connect_to_cache_steps_multiplier: u32,
    cache_path_end_distance_rating_multiplier: f64,
    cache_path_start_distance_rating_multiplier: f64,
    direct_distance_to_consider_short_request: u32,
    enemy_with_different_destination_collision_penalty: f64,
    extended_collision_penalty: f64,
    #[serde(rename = "fwd2bwd_ratio")]
    fwd_2bwd_ratio: u32,
    general_entity_collision_penalty: f64,
    general_entity_subsequent_collision_penalty: f64,
    goal_pressure_ratio: f64,
    ignore_moving_enemy_collision_distance: f64,
    long_cache_min_cacheable_distance: f64,
    long_cache_size: u32,
    max_clients_to_accept_any_new_request: u32,
    max_clients_to_accept_short_new_request: u32,
    max_steps_worked_per_tick: f64,
    max_work_done_per_tick: u32,
    min_steps_to_check_path_find_termination: u32,
    negative_cache_accept_path_end_distance_ratio: f64,
    negative_cache_accept_path_start_distance_ratio: f64,
    negative_path_cache_delay_interval: u32,
    overload_levels: crate::vec::Vec<u32>,
    overload_multipliers: crate::vec::Vec<f64>,
    short_cache_min_algo_steps_to_cache: u32,
    short_cache_min_cacheable_distance: f64,
    short_cache_size: u32,
    short_request_max_steps: u32,
    short_request_ratio: f64,
    stale_enemy_with_same_destination_collision_penalty: f64,
    start_to_goal_cost_multiplier_to_terminate_path_find: f64,
    use_path_cache: bool,
}
