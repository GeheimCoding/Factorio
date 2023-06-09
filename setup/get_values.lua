function get_difficulty_settings_values()
    return {
        recipe_difficulty = 0,
        technology_difficulty = 0,
        technology_price_multiplier = 0,
        research_queue_setting = 0,
    }
end

function get_map_settings_values()
    return {
        pollution = 0,
        enemy_evolution = 0,
        enemy_expansion = 0,
        unit_group = 0,
        steering = 0,
        path_finder = 0,
        max_failed_behavior_count = 0,
    }
end

function get_map_settings_pollution_values()
    return {
        enabled = 0,
        diffusion_ratio = 0,
        min_to_diffuse = 0,
        ageing = 0,
        expected_max_per_chunk = 0,
        min_to_show_per_chunk = 0,
        min_pollution_to_damage_trees = 0,
        pollution_with_max_forest_damage = 0,
        pollution_per_tree_damage = 0,
        pollution_restored_per_tree_damage = 0,
        max_pollution_to_restore_trees = 0,
        enemy_attack_pollution_consumption_modifier = 0,
    }
end

function get_map_settings_enemy_evolution_values()
    return {
        enabled = 0,
        time_factor = 0,
        destroy_factor = 0,
        pollution_factor = 0,
    }
end

function get_map_settings_enemy_expansion_values()
    return {
        enabled = 0,
        max_expansion_distance = 0,
        friendly_base_influence_radius = 0,
        enemy_building_influence_radius = 0,
        building_coefficient = 0,
        other_base_coefficient = 0,
        neighbouring_chunk_coefficient = 0,
        neighbouring_base_chunk_coefficient = 0,
        max_colliding_tiles_coefficient = 0,
        settler_group_min_size = 0,
        settler_group_max_size = 0,
        min_expansion_cooldown = 0,
        max_expansion_cooldown = 0,
    }
end

function get_map_settings_unit_group_values()
    return {
        min_group_gathering_time = 0,
        max_group_gathering_time = 0,
        max_wait_time_for_late_members = 0,
        min_group_radius = 0,
        max_group_radius = 0,
        max_member_speedup_when_behind = 0,
        max_member_slowdown_when_ahead = 0,
        max_group_slowdown_factor = 0,
        max_group_member_fallback_factor = 0,
        member_disown_distance = 0,
        tick_tolerance_when_member_arrives = 0,
        max_gathering_unit_groups = 0,
        max_unit_group_size = 0,
    }
end

function get_map_settings_steering_values()
    return {
        default = 0,
        moving = 0,
    }
end

function get_steering_values()
    return {
        radius = 0,
        separation_factor = 0,
        separation_force = 0,
        force_unit_fuzzy_goto_behavior = 0,
    }
end

function get_map_settings_path_finder_values()
    return {
        fwd2bwd_ratio = 0,
        goal_pressure_ratio = 0,
        max_steps_worked_per_tick = 0,
        max_work_done_per_tick = 0,
        use_path_cache = 0,
        short_cache_size = 0,
        long_cache_size = 0,
        short_cache_min_cacheable_distance = 0,
        short_cache_min_algo_steps_to_cache = 0,
        long_cache_min_cacheable_distance = 0,
        cache_max_connect_to_cache_steps_multiplier = 0,
        cache_accept_path_start_distance_ratio = 0,
        cache_accept_path_end_distance_ratio = 0,
        negative_cache_accept_path_start_distance_ratio = 0,
        negative_cache_accept_path_end_distance_ratio = 0,
        cache_path_start_distance_rating_multiplier = 0,
        cache_path_end_distance_rating_multiplier = 0,
        stale_enemy_with_same_destination_collision_penalty = 0,
        ignore_moving_enemy_collision_distance = 0,
        enemy_with_different_destination_collision_penalty = 0,
        general_entity_collision_penalty = 0,
        general_entity_subsequent_collision_penalty = 0,
        extended_collision_penalty = 0,
        max_clients_to_accept_any_new_request = 0,
        max_clients_to_accept_short_new_request = 0,
        direct_distance_to_consider_short_request = 0,
        short_request_max_steps = 0,
        short_request_ratio = 0,
        min_steps_to_check_path_find_termination = 0,
        start_to_goal_cost_multiplier_to_terminate_path_find = 0,
        overload_levels = 0,
        overload_multipliers = 0,
        negative_path_cache_delay_interval = 0,
    }
end

function get_game_view_settings_values()
    return {
        show_controller_gui = 0,
        show_minimap = 0,
        show_research_info = 0,
        show_entity_info = 0,
        show_alert_gui = 0,
        update_entity_selection = 0,
        show_rail_block_visualisation = 0,
        show_side_menu = 0,
        show_map_view_options = 0,
        show_quickbar = 0,
        show_shortcut_bar = 0,
    }
end
