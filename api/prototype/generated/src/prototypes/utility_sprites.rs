#[derive(Debug, serde::Deserialize)]
pub struct UtilitySprites {
    #[serde(flatten)]
    base_: crate::prototypes::PrototypeBase,
    achievement_label: crate::types::Sprite,
    achievement_label_completed: crate::types::Sprite,
    achievement_label_failed: crate::types::Sprite,
    achievement_warning: crate::types::Sprite,
    add: crate::types::Sprite,
    add_white: crate::types::Sprite,
    alert_arrow: crate::types::Sprite,
    ammo_damage_modifier_constant: Option<crate::types::Sprite>,
    ammo_damage_modifier_icon: crate::types::Sprite,
    ammo_icon: crate::types::Sprite,
    and_or: crate::types::Sprite,
    any_quality: crate::types::Sprite,
    area_icon: crate::types::Sprite,
    arrow_button: crate::types::Animation,
    artillery_range_modifier_constant: Option<crate::types::Sprite>,
    artillery_range_modifier_icon: crate::types::Sprite,
    asteroid_chunk_editor_icon: crate::types::Sprite,
    asteroid_collector_path_blocked_icon: crate::types::Sprite,
    backward_arrow: crate::types::Sprite,
    backward_arrow_black: crate::types::Sprite,
    bar_gray_pip: crate::types::Sprite,
    battery: crate::types::Sprite,
    beacon_distribution_modifier_constant: Option<crate::types::Sprite>,
    beacon_distribution_modifier_icon: crate::types::Sprite,
    belt_stack_size_bonus_modifier_constant: Option<crate::types::Sprite>,
    belt_stack_size_bonus_modifier_icon: crate::types::Sprite,
    bookmark: crate::types::Sprite,
    brush_circle_shape: crate::types::Sprite,
    brush_icon: crate::types::Sprite,
    brush_square_shape: crate::types::Sprite,
    buildability_collision: crate::types::Sprite,
    buildability_collision_elevated: crate::types::Sprite,
    buildability_elevated_collision_bottom: crate::types::Sprite,
    buildability_elevated_collision_line: crate::types::Sprite,
    buildability_elevated_collision_top: crate::types::Sprite,
    bulk_inserter_capacity_bonus_modifier_constant: Option<crate::types::Sprite>,
    bulk_inserter_capacity_bonus_modifier_icon: crate::types::Sprite,
    cable_editor_icon: crate::types::Sprite,
    cargo_bay_not_connected_icon: crate::types::Sprite,
    cargo_landing_pad_count_modifier_constant: Option<crate::types::Sprite>,
    cargo_landing_pad_count_modifier_icon: crate::types::Sprite,
    center: crate::types::Sprite,
    change_recipe: crate::types::Sprite,
    change_recipe_productivity_modifier_constant: Option<crate::types::Sprite>,
    change_recipe_productivity_modifier_icon: crate::types::Sprite,
    character_additional_mining_categories_modifier_constant: Option<crate::types::Sprite>,
    character_additional_mining_categories_modifier_icon: crate::types::Sprite,
    character_build_distance_modifier_constant: Option<crate::types::Sprite>,
    character_build_distance_modifier_icon: crate::types::Sprite,
    character_crafting_speed_modifier_constant: Option<crate::types::Sprite>,
    character_crafting_speed_modifier_icon: crate::types::Sprite,
    character_health_bonus_modifier_constant: Option<crate::types::Sprite>,
    character_health_bonus_modifier_icon: crate::types::Sprite,
    character_inventory_slots_bonus_modifier_constant: Option<crate::types::Sprite>,
    character_inventory_slots_bonus_modifier_icon: crate::types::Sprite,
    character_item_drop_distance_modifier_constant: Option<crate::types::Sprite>,
    character_item_drop_distance_modifier_icon: crate::types::Sprite,
    character_item_pickup_distance_modifier_constant: Option<crate::types::Sprite>,
    character_item_pickup_distance_modifier_icon: crate::types::Sprite,
    character_logistic_requests_modifier_constant: Option<crate::types::Sprite>,
    character_logistic_requests_modifier_icon: crate::types::Sprite,
    character_logistic_trash_slots_modifier_constant: Option<crate::types::Sprite>,
    character_logistic_trash_slots_modifier_icon: crate::types::Sprite,
    character_loot_pickup_distance_modifier_constant: Option<crate::types::Sprite>,
    character_loot_pickup_distance_modifier_icon: crate::types::Sprite,
    character_mining_speed_modifier_constant: Option<crate::types::Sprite>,
    character_mining_speed_modifier_icon: crate::types::Sprite,
    character_reach_distance_modifier_constant: Option<crate::types::Sprite>,
    character_reach_distance_modifier_icon: crate::types::Sprite,
    character_resource_reach_distance_modifier_constant: Option<crate::types::Sprite>,
    character_resource_reach_distance_modifier_icon: crate::types::Sprite,
    character_running_speed_modifier_constant: Option<crate::types::Sprite>,
    character_running_speed_modifier_icon: crate::types::Sprite,
    check_mark: crate::types::Sprite,
    check_mark_dark_green: crate::types::Sprite,
    check_mark_green: crate::types::Sprite,
    check_mark_white: crate::types::Sprite,
    circuit_network_panel: crate::types::Sprite,
    cliff_deconstruction_enabled_modifier_constant: Option<crate::types::Sprite>,
    cliff_deconstruction_enabled_modifier_icon: crate::types::Sprite,
    cliff_editor_icon: crate::types::Sprite,
    clock: crate::types::Sprite,
    clone: crate::types::Sprite,
    clone_editor_icon: crate::types::Sprite,
    close: crate::types::Sprite,
    close_black: crate::types::Sprite,
    close_fat: crate::types::Sprite,
    close_map_preview: crate::types::Sprite,
    clouds: crate::types::Animation,
    collapse: crate::types::Sprite,
    color_effect: crate::types::Sprite,
    color_picker: crate::types::Sprite,
    confirm_slot: crate::types::Sprite,
    construction_radius_visualization: crate::types::Sprite,
    controller_joycon_a: crate::types::Sprite,
    controller_joycon_b: crate::types::Sprite,
    controller_joycon_back: crate::types::Sprite,
    controller_joycon_black_a: crate::types::Sprite,
    controller_joycon_black_b: crate::types::Sprite,
    controller_joycon_black_back: crate::types::Sprite,
    controller_joycon_black_dpdown: crate::types::Sprite,
    controller_joycon_black_dpleft: crate::types::Sprite,
    controller_joycon_black_dpright: crate::types::Sprite,
    controller_joycon_black_dpup: crate::types::Sprite,
    controller_joycon_black_left_stick: crate::types::Sprite,
    controller_joycon_black_leftshoulder: crate::types::Sprite,
    controller_joycon_black_leftstick: crate::types::Sprite,
    controller_joycon_black_lefttrigger: crate::types::Sprite,
    #[serde(rename = "controller_joycon_black_paddle1")]
    controller_joycon_black_paddle_1: crate::types::Sprite,
    #[serde(rename = "controller_joycon_black_paddle2")]
    controller_joycon_black_paddle_2: crate::types::Sprite,
    #[serde(rename = "controller_joycon_black_paddle3")]
    controller_joycon_black_paddle_3: crate::types::Sprite,
    #[serde(rename = "controller_joycon_black_paddle4")]
    controller_joycon_black_paddle_4: crate::types::Sprite,
    controller_joycon_black_right_stick: crate::types::Sprite,
    controller_joycon_black_rightshoulder: crate::types::Sprite,
    controller_joycon_black_rightstick: crate::types::Sprite,
    controller_joycon_black_righttrigger: crate::types::Sprite,
    controller_joycon_black_start: crate::types::Sprite,
    controller_joycon_black_x: crate::types::Sprite,
    controller_joycon_black_y: crate::types::Sprite,
    controller_joycon_dpdown: crate::types::Sprite,
    controller_joycon_dpleft: crate::types::Sprite,
    controller_joycon_dpright: crate::types::Sprite,
    controller_joycon_dpup: crate::types::Sprite,
    controller_joycon_left_stick: crate::types::Sprite,
    controller_joycon_leftshoulder: crate::types::Sprite,
    controller_joycon_leftstick: crate::types::Sprite,
    controller_joycon_lefttrigger: crate::types::Sprite,
    #[serde(rename = "controller_joycon_paddle1")]
    controller_joycon_paddle_1: crate::types::Sprite,
    #[serde(rename = "controller_joycon_paddle2")]
    controller_joycon_paddle_2: crate::types::Sprite,
    #[serde(rename = "controller_joycon_paddle3")]
    controller_joycon_paddle_3: crate::types::Sprite,
    #[serde(rename = "controller_joycon_paddle4")]
    controller_joycon_paddle_4: crate::types::Sprite,
    controller_joycon_right_stick: crate::types::Sprite,
    controller_joycon_rightshoulder: crate::types::Sprite,
    controller_joycon_rightstick: crate::types::Sprite,
    controller_joycon_righttrigger: crate::types::Sprite,
    controller_joycon_start: crate::types::Sprite,
    controller_joycon_x: crate::types::Sprite,
    controller_joycon_y: crate::types::Sprite,
    controller_ps_a: crate::types::Sprite,
    controller_ps_b: crate::types::Sprite,
    controller_ps_back: crate::types::Sprite,
    controller_ps_black_a: crate::types::Sprite,
    controller_ps_black_b: crate::types::Sprite,
    controller_ps_black_back: crate::types::Sprite,
    controller_ps_black_dpdown: crate::types::Sprite,
    controller_ps_black_dpleft: crate::types::Sprite,
    controller_ps_black_dpright: crate::types::Sprite,
    controller_ps_black_dpup: crate::types::Sprite,
    controller_ps_black_left_stick: crate::types::Sprite,
    controller_ps_black_leftshoulder: crate::types::Sprite,
    controller_ps_black_leftstick: crate::types::Sprite,
    controller_ps_black_lefttrigger: crate::types::Sprite,
    controller_ps_black_right_stick: crate::types::Sprite,
    controller_ps_black_rightshoulder: crate::types::Sprite,
    controller_ps_black_rightstick: crate::types::Sprite,
    controller_ps_black_righttrigger: crate::types::Sprite,
    controller_ps_black_start: crate::types::Sprite,
    controller_ps_black_x: crate::types::Sprite,
    controller_ps_black_y: crate::types::Sprite,
    controller_ps_dpdown: crate::types::Sprite,
    controller_ps_dpleft: crate::types::Sprite,
    controller_ps_dpright: crate::types::Sprite,
    controller_ps_dpup: crate::types::Sprite,
    controller_ps_left_stick: crate::types::Sprite,
    controller_ps_leftshoulder: crate::types::Sprite,
    controller_ps_leftstick: crate::types::Sprite,
    controller_ps_lefttrigger: crate::types::Sprite,
    controller_ps_right_stick: crate::types::Sprite,
    controller_ps_rightshoulder: crate::types::Sprite,
    controller_ps_rightstick: crate::types::Sprite,
    controller_ps_righttrigger: crate::types::Sprite,
    controller_ps_start: crate::types::Sprite,
    controller_ps_x: crate::types::Sprite,
    controller_ps_y: crate::types::Sprite,
    controller_steamdeck_a: crate::types::Sprite,
    controller_steamdeck_b: crate::types::Sprite,
    controller_steamdeck_back: crate::types::Sprite,
    controller_steamdeck_black_a: crate::types::Sprite,
    controller_steamdeck_black_b: crate::types::Sprite,
    controller_steamdeck_black_back: crate::types::Sprite,
    controller_steamdeck_black_dpdown: crate::types::Sprite,
    controller_steamdeck_black_dpleft: crate::types::Sprite,
    controller_steamdeck_black_dpright: crate::types::Sprite,
    controller_steamdeck_black_dpup: crate::types::Sprite,
    controller_steamdeck_black_left_stick: crate::types::Sprite,
    controller_steamdeck_black_leftshoulder: crate::types::Sprite,
    controller_steamdeck_black_leftstick: crate::types::Sprite,
    controller_steamdeck_black_lefttrigger: crate::types::Sprite,
    #[serde(rename = "controller_steamdeck_black_paddle1")]
    controller_steamdeck_black_paddle_1: crate::types::Sprite,
    #[serde(rename = "controller_steamdeck_black_paddle2")]
    controller_steamdeck_black_paddle_2: crate::types::Sprite,
    #[serde(rename = "controller_steamdeck_black_paddle3")]
    controller_steamdeck_black_paddle_3: crate::types::Sprite,
    #[serde(rename = "controller_steamdeck_black_paddle4")]
    controller_steamdeck_black_paddle_4: crate::types::Sprite,
    controller_steamdeck_black_right_stick: crate::types::Sprite,
    controller_steamdeck_black_rightshoulder: crate::types::Sprite,
    controller_steamdeck_black_rightstick: crate::types::Sprite,
    controller_steamdeck_black_righttrigger: crate::types::Sprite,
    controller_steamdeck_black_start: crate::types::Sprite,
    controller_steamdeck_black_x: crate::types::Sprite,
    controller_steamdeck_black_y: crate::types::Sprite,
    controller_steamdeck_dpdown: crate::types::Sprite,
    controller_steamdeck_dpleft: crate::types::Sprite,
    controller_steamdeck_dpright: crate::types::Sprite,
    controller_steamdeck_dpup: crate::types::Sprite,
    controller_steamdeck_left_stick: crate::types::Sprite,
    controller_steamdeck_leftshoulder: crate::types::Sprite,
    controller_steamdeck_leftstick: crate::types::Sprite,
    controller_steamdeck_lefttrigger: crate::types::Sprite,
    #[serde(rename = "controller_steamdeck_paddle1")]
    controller_steamdeck_paddle_1: crate::types::Sprite,
    #[serde(rename = "controller_steamdeck_paddle2")]
    controller_steamdeck_paddle_2: crate::types::Sprite,
    #[serde(rename = "controller_steamdeck_paddle3")]
    controller_steamdeck_paddle_3: crate::types::Sprite,
    #[serde(rename = "controller_steamdeck_paddle4")]
    controller_steamdeck_paddle_4: crate::types::Sprite,
    controller_steamdeck_right_stick: crate::types::Sprite,
    controller_steamdeck_rightshoulder: crate::types::Sprite,
    controller_steamdeck_rightstick: crate::types::Sprite,
    controller_steamdeck_righttrigger: crate::types::Sprite,
    controller_steamdeck_start: crate::types::Sprite,
    controller_steamdeck_x: crate::types::Sprite,
    controller_steamdeck_y: crate::types::Sprite,
    controller_xbox_a: crate::types::Sprite,
    controller_xbox_b: crate::types::Sprite,
    controller_xbox_back: crate::types::Sprite,
    controller_xbox_black_a: crate::types::Sprite,
    controller_xbox_black_b: crate::types::Sprite,
    controller_xbox_black_back: crate::types::Sprite,
    controller_xbox_black_dpdown: crate::types::Sprite,
    controller_xbox_black_dpleft: crate::types::Sprite,
    controller_xbox_black_dpright: crate::types::Sprite,
    controller_xbox_black_dpup: crate::types::Sprite,
    controller_xbox_black_left_stick: crate::types::Sprite,
    controller_xbox_black_leftshoulder: crate::types::Sprite,
    controller_xbox_black_leftstick: crate::types::Sprite,
    controller_xbox_black_lefttrigger: crate::types::Sprite,
    controller_xbox_black_right_stick: crate::types::Sprite,
    controller_xbox_black_rightshoulder: crate::types::Sprite,
    controller_xbox_black_rightstick: crate::types::Sprite,
    controller_xbox_black_righttrigger: crate::types::Sprite,
    controller_xbox_black_start: crate::types::Sprite,
    controller_xbox_black_x: crate::types::Sprite,
    controller_xbox_black_y: crate::types::Sprite,
    controller_xbox_dpdown: crate::types::Sprite,
    controller_xbox_dpleft: crate::types::Sprite,
    controller_xbox_dpright: crate::types::Sprite,
    controller_xbox_dpup: crate::types::Sprite,
    controller_xbox_left_stick: crate::types::Sprite,
    controller_xbox_leftshoulder: crate::types::Sprite,
    controller_xbox_leftstick: crate::types::Sprite,
    controller_xbox_lefttrigger: crate::types::Sprite,
    controller_xbox_right_stick: crate::types::Sprite,
    controller_xbox_rightshoulder: crate::types::Sprite,
    controller_xbox_rightstick: crate::types::Sprite,
    controller_xbox_righttrigger: crate::types::Sprite,
    controller_xbox_start: crate::types::Sprite,
    controller_xbox_x: crate::types::Sprite,
    controller_xbox_y: crate::types::Sprite,
    copper_wire: crate::types::Sprite,
    copper_wire_highlight: crate::types::Sprite,
    copy: crate::types::Sprite,
    covered_chunk: crate::types::Sprite,
    crafting_machine_recipe_not_unlocked: crate::types::Sprite,
    create_ghost_on_entity_death_modifier_constant: Option<crate::types::Sprite>,
    create_ghost_on_entity_death_modifier_icon: crate::types::Sprite,
    cross_select: crate::types::Sprite,
    crosshair: crate::types::Sprite,
    cursor_box: CursorBoxSpecification,
    cursor_icon: crate::types::Sprite,
    custom_tag_icon: crate::types::Sprite,
    custom_tag_in_map_view: crate::types::Sprite,
    danger_icon: crate::types::Sprite,
    deconstruction_mark: crate::types::Sprite,
    deconstruction_time_to_live_modifier_constant: Option<crate::types::Sprite>,
    deconstruction_time_to_live_modifier_icon: crate::types::Sprite,
    decorative_editor_icon: crate::types::Sprite,
    default_ammo_damage_modifier_icon: crate::types::Sprite,
    default_gun_speed_modifier_icon: crate::types::Sprite,
    default_turret_attack_modifier_icon: crate::types::Sprite,
    destination_full_icon: crate::types::Sprite,
    destroyed_icon: crate::types::Sprite,
    down_arrow: crate::types::Sprite,
    downloaded: crate::types::Sprite,
    downloading: crate::types::Sprite,
    dropdown: crate::types::Sprite,
    editor_pause: crate::types::Sprite,
    editor_play: crate::types::Sprite,
    editor_selection: crate::types::Sprite,
    editor_speed_down: crate::types::Sprite,
    editor_speed_up: crate::types::Sprite,
    electricity_icon: crate::types::Sprite,
    electricity_icon_unplugged: crate::types::Sprite,
    empty_ammo_slot: crate::types::Sprite,
    empty_armor_slot: crate::types::Sprite,
    empty_drop_cargo_slot: crate::types::Sprite,
    empty_gun_slot: crate::types::Sprite,
    empty_inserter_hand_slot: crate::types::Sprite,
    empty_module_slot: crate::types::Sprite,
    empty_robot_material_slot: crate::types::Sprite,
    empty_robot_slot: crate::types::Sprite,
    empty_trash_slot: crate::types::Sprite,
    enemy_force_icon: crate::types::Sprite,
    enter: crate::types::Sprite,
    entity_editor_icon: crate::types::Sprite,
    entity_info_dark_background: crate::types::Sprite,
    equipment_collision: crate::types::Sprite,
    equipment_grid: crate::types::Sprite,
    equipment_slot: crate::types::Sprite,
    expand: crate::types::Sprite,
    expand_dots: crate::types::Sprite,
    explosion_chart_visualization: crate::types::Animation,
    export: crate::types::Sprite,
    export_slot: crate::types::Sprite,
    feedback: crate::types::Sprite,
    filter_blacklist: crate::types::Sprite,
    fluid_icon: crate::types::Sprite,
    fluid_indication_arrow: crate::types::Sprite,
    fluid_indication_arrow_both_ways: crate::types::Sprite,
    fluid_visualization_connection: crate::types::Sprite,
    fluid_visualization_connection_both_ways: crate::types::Sprite,
    fluid_visualization_connection_underground: crate::types::Sprite,
    fluid_visualization_extent_arrow: crate::types::Sprite,
    follower_robot_lifetime_modifier_constant: Option<crate::types::Sprite>,
    follower_robot_lifetime_modifier_icon: crate::types::Sprite,
    force_editor_icon: crate::types::Sprite,
    force_ghost_cursor: crate::types::Sprite,
    force_tile_ghost_cursor: crate::types::Sprite,
    forward_arrow: crate::types::Sprite,
    forward_arrow_black: crate::types::Sprite,
    frozen_icon: crate::types::Sprite,
    fuel_icon: crate::types::Sprite,
    game_stopped_visualization: crate::types::Sprite,
    ghost_bar_pip: crate::types::Sprite,
    ghost_cursor: crate::types::Sprite,
    give_item_modifier_constant: Option<crate::types::Sprite>,
    give_item_modifier_icon: crate::types::Sprite,
    go_to_arrow: crate::types::Sprite,
    gps_map_icon: crate::types::Sprite,
    gradient: crate::types::Sprite,
    green_circle: crate::types::Sprite,
    green_dot: crate::types::Sprite,
    green_wire: crate::types::Sprite,
    green_wire_highlight: crate::types::Sprite,
    grey_placement_indicator_leg: crate::types::Sprite,
    grey_rail_signal_placement_indicator: crate::types::Sprite,
    grid_view: crate::types::Sprite,
    gun_speed_modifier_constant: Option<crate::types::Sprite>,
    gun_speed_modifier_icon: crate::types::Sprite,
    hand: crate::types::Sprite,
    hand_black: crate::types::Sprite,
    health_bar_green_pip: crate::types::Sprite,
    health_bar_red_pip: crate::types::Sprite,
    health_bar_yellow_pip: crate::types::Sprite,
    heat_exchange_indication: crate::types::Sprite,
    hint_arrow_down: crate::types::Sprite,
    hint_arrow_left: crate::types::Sprite,
    hint_arrow_right: crate::types::Sprite,
    hint_arrow_up: crate::types::Sprite,
    import: crate::types::Sprite,
    import_slot: crate::types::Sprite,
    indication_arrow: crate::types::Sprite,
    indication_line: crate::types::Sprite,
    inserter_stack_size_bonus_modifier_constant: Option<crate::types::Sprite>,
    inserter_stack_size_bonus_modifier_icon: crate::types::Sprite,
    item_editor_icon: crate::types::Sprite,
    item_to_be_delivered_symbol: crate::types::Sprite,
    laboratory_productivity_modifier_constant: Option<crate::types::Sprite>,
    laboratory_productivity_modifier_icon: crate::types::Sprite,
    laboratory_speed_modifier_constant: Option<crate::types::Sprite>,
    laboratory_speed_modifier_icon: crate::types::Sprite,
    left_arrow: crate::types::Sprite,
    light_cone: crate::types::Sprite,
    light_medium: crate::types::Sprite,
    light_small: crate::types::Sprite,
    lightning_warning_icon: crate::types::Sprite,
    line_icon: crate::types::Sprite,
    list_view: crate::types::Sprite,
    logistic_network_panel_black: crate::types::Sprite,
    logistic_network_panel_white: crate::types::Sprite,
    logistic_radius_visualization: crate::types::Sprite,
    lua_snippet_tool_icon: crate::types::Sprite,
    map: crate::types::Sprite,
    map_exchange_string: crate::types::Sprite,
    max_distance_underground_remove_belts: crate::types::Sprite,
    max_failed_attempts_per_tick_per_construction_queue_modifier_constant:
        Option<crate::types::Sprite>,
    max_failed_attempts_per_tick_per_construction_queue_modifier_icon: crate::types::Sprite,
    max_successful_attempts_per_tick_per_construction_queue_modifier_constant:
        Option<crate::types::Sprite>,
    max_successful_attempts_per_tick_per_construction_queue_modifier_icon: crate::types::Sprite,
    maximum_following_robots_count_modifier_constant: Option<crate::types::Sprite>,
    maximum_following_robots_count_modifier_icon: crate::types::Sprite,
    medium_gui_arrow: crate::types::Sprite,
    mining_drill_productivity_bonus_modifier_constant: Option<crate::types::Sprite>,
    mining_drill_productivity_bonus_modifier_icon: crate::types::Sprite,
    mining_with_fluid_modifier_constant: Option<crate::types::Sprite>,
    mining_with_fluid_modifier_icon: crate::types::Sprite,
    missing_icon: crate::types::Sprite,
    missing_mod_icon: crate::types::Sprite,
    mod_category: crate::types::Sprite,
    mod_dependency_arrow: crate::types::Sprite,
    mod_downloads_count: crate::types::Sprite,
    mod_last_updated: crate::types::Sprite,
    mouse_cursor: crate::types::Sprite,
    move_tag: crate::types::Sprite,
    multiplayer_waiting_icon: crate::types::Sprite,
    nature_icon: crate::types::Sprite,
    navmesh_pending_icon: crate::types::Animation,
    neutral_force_icon: crate::types::Sprite,
    no_building_material_icon: crate::types::Sprite,
    no_nature_icon: crate::types::Sprite,
    no_path_icon: crate::types::Sprite,
    no_platform_storage_space_icon: crate::types::Sprite,
    no_roboport_storage_space_icon: crate::types::Sprite,
    no_storage_space_icon: crate::types::Sprite,
    none_editor_icon: crate::types::Sprite,
    not_available: crate::types::Sprite,
    not_available_black: crate::types::Sprite,
    not_enough_construction_robots_icon: crate::types::Sprite,
    not_enough_repair_packs_icon: crate::types::Sprite,
    not_played_yet_dark_green: crate::types::Sprite,
    not_played_yet_green: crate::types::Sprite,
    nothing_modifier_constant: Option<crate::types::Sprite>,
    nothing_modifier_icon: crate::types::Sprite,
    notification: crate::types::Sprite,
    output_console_gradient: crate::types::Sprite,
    paint_bucket_icon: crate::types::Sprite,
    parametrise: crate::types::Sprite,
    pause: crate::types::Sprite,
    pin_arrow: crate::types::Sprite,
    pin_center: crate::types::Sprite,
    pipeline_disabled_icon: crate::types::Sprite,
    placement_indicator_leg: crate::types::Sprite,
    platform_entity_build_animations: EntityBuildAnimations,
    play: crate::types::Sprite,
    played_dark_green: crate::types::Sprite,
    played_green: crate::types::Sprite,
    player_force_icon: crate::types::Sprite,
    preset: crate::types::Sprite,
    pump_cannot_connect_icon: crate::types::Sprite,
    questionmark: crate::types::Sprite,
    rail_path_not_possible: crate::types::Sprite,
    rail_planner_allow_elevated_rails_modifier_constant: Option<crate::types::Sprite>,
    rail_planner_allow_elevated_rails_modifier_icon: crate::types::Sprite,
    rail_planner_indication_arrow: crate::types::Sprite,
    rail_planner_indication_arrow_anchored: crate::types::Sprite,
    rail_planner_indication_arrow_too_far: crate::types::Sprite,
    rail_signal_placement_indicator: crate::types::Sprite,
    rail_support_on_deep_oil_ocean_modifier_constant: Option<crate::types::Sprite>,
    rail_support_on_deep_oil_ocean_modifier_icon: crate::types::Sprite,
    rail_support_placement_indicator: crate::types::Sprite,
    reassign: crate::types::Sprite,
    rebuild_mark: crate::types::Sprite,
    recharge_icon: crate::types::Sprite,
    recipe_arrow: crate::types::Sprite,
    red_wire: crate::types::Sprite,
    red_wire_highlight: crate::types::Sprite,
    reference_point: crate::types::Sprite,
    refresh: crate::types::Sprite,
    refresh_white: crate::types::Animation,
    rename_icon: crate::types::Sprite,
    reset: crate::types::Sprite,
    reset_white: crate::types::Sprite,
    resource_editor_icon: crate::types::Sprite,
    resources_depleted_icon: crate::types::Sprite,
    right_arrow: crate::types::Sprite,
    robot_slot: crate::types::Sprite,
    scripting_editor_icon: crate::types::Sprite,
    search: crate::types::Sprite,
    search_icon: crate::types::Sprite,
    select_icon_black: crate::types::Sprite,
    select_icon_white: crate::types::Sprite,
    set_bar_slot: crate::types::Sprite,
    shield_bar_pip: crate::types::Sprite,
    shoot_cursor_green: crate::types::Sprite,
    shoot_cursor_red: crate::types::Sprite,
    short_indication_line: crate::types::Sprite,
    short_indication_line_green: crate::types::Sprite,
    show_electric_network_in_map_view: crate::types::Sprite,
    show_logistics_network_in_map_view: crate::types::Sprite,
    show_pipelines_in_map_view: crate::types::Sprite,
    show_player_names_in_map_view: crate::types::Sprite,
    show_rail_signal_states_in_map_view: crate::types::Sprite,
    show_recipe_icons_in_map_view: crate::types::Sprite,
    show_tags_in_map_view: crate::types::Sprite,
    show_train_station_names_in_map_view: crate::types::Sprite,
    show_turret_range_in_map_view: crate::types::Sprite,
    show_worker_robots_in_map_view: crate::types::Sprite,
    shuffle: crate::types::Sprite,
    side_menu_achievements_icon: crate::types::Sprite,
    side_menu_blueprint_library_icon: crate::types::Sprite,
    side_menu_bonus_icon: crate::types::Sprite,
    side_menu_factoriopedia_icon: crate::types::Sprite,
    side_menu_logistic_networks_icon: crate::types::Sprite,
    side_menu_map_icon: crate::types::Sprite,
    side_menu_menu_icon: crate::types::Sprite,
    side_menu_players_icon: crate::types::Sprite,
    side_menu_production_icon: crate::types::Sprite,
    side_menu_space_platforms_icon: crate::types::Sprite,
    side_menu_technology_icon: crate::types::Sprite,
    side_menu_train_icon: crate::types::Sprite,
    side_menu_tutorials_icon: crate::types::Sprite,
    slot: crate::types::Sprite,
    slots_view: crate::types::Sprite,
    small_gui_arrow: crate::types::Sprite,
    sort_by_name: crate::types::Sprite,
    sort_by_time: crate::types::Sprite,
    space_age_icon: crate::types::Sprite,
    spawn_flag: crate::types::Sprite,
    speed_down: crate::types::Sprite,
    speed_up: crate::types::Sprite,
    spray_icon: crate::types::Sprite,
    starmap_platform_moving: crate::types::Sprite,
    starmap_platform_moving_clicked: crate::types::Sprite,
    starmap_platform_moving_hovered: crate::types::Sprite,
    starmap_platform_stacked: crate::types::Sprite,
    starmap_platform_stacked_clicked: crate::types::Sprite,
    starmap_platform_stacked_hovered: crate::types::Sprite,
    starmap_platform_stopped: crate::types::Sprite,
    starmap_platform_stopped_clicked: crate::types::Sprite,
    starmap_platform_stopped_hovered: crate::types::Sprite,
    starmap_star: crate::types::Sprite,
    station_name: crate::types::Sprite,
    status_blue: crate::types::Sprite,
    status_inactive: crate::types::Sprite,
    status_not_working: crate::types::Sprite,
    status_working: crate::types::Sprite,
    status_yellow: crate::types::Sprite,
    stop: crate::types::Sprite,
    surface_editor_icon: crate::types::Sprite,
    sync_mods: crate::types::Sprite,
    technology_white: crate::types::Sprite,
    tick_custom: crate::types::Sprite,
    tick_once: crate::types::Sprite,
    tick_sixty: crate::types::Sprite,
    tile_editor_icon: crate::types::Sprite,
    tile_ghost_cursor: crate::types::Sprite,
    time_editor_icon: crate::types::Sprite,
    tip_icon: crate::types::Sprite,
    too_far: crate::types::Sprite,
    too_far_from_roboport_icon: crate::types::Sprite,
    tooltip_category_spoilable: crate::types::Sprite,
    track_button: crate::types::Sprite,
    track_button_white: crate::types::Sprite,
    train_braking_force_bonus_modifier_constant: Option<crate::types::Sprite>,
    train_braking_force_bonus_modifier_icon: crate::types::Sprite,
    train_stop_disabled_in_map_view: crate::types::Sprite,
    train_stop_full_in_map_view: crate::types::Sprite,
    train_stop_in_map_view: crate::types::Sprite,
    train_stop_placement_indicator: crate::types::Sprite,
    trash: crate::types::Sprite,
    trash_white: crate::types::Sprite,
    turret_attack_modifier_constant: Option<crate::types::Sprite>,
    turret_attack_modifier_icon: crate::types::Sprite,
    unclaimed_cargo_icon: crate::types::Sprite,
    underground_pipe_connection: crate::types::Sprite,
    underground_remove_belts: crate::types::Sprite,
    underground_remove_pipes: crate::types::Sprite,
    unlock_circuit_network_modifier_constant: Option<crate::types::Sprite>,
    unlock_circuit_network_modifier_icon: crate::types::Sprite,
    unlock_quality_modifier_constant: Option<crate::types::Sprite>,
    unlock_quality_modifier_icon: crate::types::Sprite,
    unlock_recipe_modifier_constant: Option<crate::types::Sprite>,
    unlock_recipe_modifier_icon: crate::types::Sprite,
    unlock_space_location_modifier_constant: Option<crate::types::Sprite>,
    unlock_space_location_modifier_icon: crate::types::Sprite,
    unlock_space_platforms_modifier_constant: Option<crate::types::Sprite>,
    unlock_space_platforms_modifier_icon: crate::types::Sprite,
    upgrade_blueprint: crate::types::Sprite,
    upgrade_mark: crate::types::Sprite,
    variations_tool_icon: crate::types::Sprite,
    vehicle_logistics_modifier_constant: Option<crate::types::Sprite>,
    vehicle_logistics_modifier_icon: crate::types::Sprite,
    warning: crate::types::Sprite,
    warning_icon: crate::types::Sprite,
    warning_white: crate::types::Sprite,
    white_mask: crate::types::Sprite,
    white_square: crate::types::Sprite,
    white_square_icon: crate::types::Sprite,
    wire_shadow: crate::types::Sprite,
    worker_robot_battery_modifier_constant: Option<crate::types::Sprite>,
    worker_robot_battery_modifier_icon: crate::types::Sprite,
    worker_robot_speed_modifier_constant: Option<crate::types::Sprite>,
    worker_robot_speed_modifier_icon: crate::types::Sprite,
    worker_robot_storage_modifier_constant: Option<crate::types::Sprite>,
    worker_robot_storage_modifier_icon: crate::types::Sprite,
}
#[derive(Debug, serde::Deserialize)]
pub struct CursorBoxSpecification {
    blueprint_snap_rectangle: Vec<crate::types::BoxSpecification>,
    copy: Vec<crate::types::BoxSpecification>,
    electricity: Vec<crate::types::BoxSpecification>,
    logistics: Vec<crate::types::BoxSpecification>,
    multiplayer_selection: Vec<crate::types::BoxSpecification>,
    not_allowed: Vec<crate::types::BoxSpecification>,
    pair: Vec<crate::types::BoxSpecification>,
    regular: Vec<crate::types::BoxSpecification>,
    rts_selected: Vec<crate::types::BoxSpecification>,
    rts_to_be_selected: Vec<crate::types::BoxSpecification>,
    train_visualization: Vec<crate::types::BoxSpecification>,
}
#[derive(Debug, serde::Deserialize)]
pub struct EntityBuildAnimations {
    back_left: crate::types::EntityBuildAnimationPiece,
    back_right: crate::types::EntityBuildAnimationPiece,
    front_left: crate::types::EntityBuildAnimationPiece,
    front_right: crate::types::EntityBuildAnimationPiece,
}
