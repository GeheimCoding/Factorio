pub struct MainSound {
    activity_to_speed_modifiers: ActivityMatchingModifiers,
    activity_to_volume_modifiers: ActivityMatchingModifiers,
    audible_distance_modifier: f64,
    fade_in_ticks: u32,
    fade_out_ticks: u32,
    match_progress_to_activity: bool,
    match_speed_to_activity: bool,
    match_volume_to_activity: bool,
    play_for_working_visualisations: Vec<String>,
    probability: f64,
    sound: Sound,
    volume_smoothing_window_size: u32,
}
