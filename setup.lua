function needs_special_care(obj, attribute)
    if obj.object_name == 'LuaGuiElement' then
        return attribute == 'actual_size' or attribute == 'anchor'
    elseif obj.object_name == 'LuaPlayer' then
        return attribute == 'infinity_inventory_filters'
            or attribute == 'remove_unfiltered_items'
            or attribute == 'vehicle_logistic_requests_enabled'
    end
    return false
end

function get_values(obj)
    if not obj.help then
        return obj
    end

    if obj.object_name == 'LuaDifficultySettings' then
        return get_difficulty_settings_values()
    elseif obj.object_name == 'LuaMapSettings' then
        return get_map_settings_values()
    elseif obj.object_name == 'LuaMapSettings.pollution' then
        return get_map_settings_pollution_values()
    elseif obj.object_name == 'LuaMapSettings.enemy_evolution' then
        return get_map_settings_enemy_evolution_values()
    elseif obj.object_name == 'LuaMapSettings.enemy_expansion' then
        return get_map_settings_enemy_expansion_values()
    elseif obj.object_name == 'LuaMapSettings.unit_group' then
        return get_map_settings_unit_group_values()
    elseif obj.object_name == 'LuaMapSettings.steering' then
        return get_map_settings_steering_values()
    elseif obj.object_name == 'LuaMapSettings.steering.default'
        or obj.object_name == 'LuaMapSettings.steering.moving' then
        return get_steering_values()
    elseif obj.object_name == 'LuaMapSettings.path_finder' then
        return get_map_settings_path_finder_values()
    elseif obj.object_name == 'LuaGameViewSettings' then
        return get_game_view_settings_values()
    end

    local t = {}
    for k, v in string.gmatch(obj.help(), '([a-z_]+)%s(%[R%u?%])') do
        if not needs_special_care(obj, k) then
            t[k] = 0
        end
    end
    return t
end

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

function is_cycle(obj, map)
    for k,v in pairs(global.lookup.cycles) do
        if v.obj == obj then
            map[v.obj.object_name .. ':' .. tostring(k)] = 0
            return true, k
        end
    end
    return false, 0
end

function is_allowed_to_access_attribute(obj, values, attribute)
    if obj.object_name == 'LuaItemStack' and not obj.valid_for_read then
        return false
    elseif obj.object_name == 'LuaGroup' then
        if obj.type == 'item-group' then
            return attribute ~= 'group'
        else
            return attribute ~= 'subgroups' and attribute ~= 'order_in_recipe'
        end
    end
    local key = values.type
    if obj.object_name == 'LuaStyle' then
        key = obj.name
    end
    if not key or not global.lookup.subclasses[obj.object_name] or not global.lookup.subclasses[obj.object_name][attribute] then
        return true
    end
    local subclasses = global.lookup.subclasses[obj.object_name][attribute]
    if type(subclasses) == 'string' then
        return subclasses == key
    else
        for k,v in pairs(subclasses) do
            if v == key then
                return true
            end
        end
        return false
    end
end

function is_class(obj)
    -- game is not serializable (and there are no cycles with it anyway)
    if not obj.help or obj.object_name == 'LuaGameScript' then
        return false
    end
    return type(obj.help) ~= 'string'
end

function to_json(obj)
    return to_json_internal(obj, 1, {}, false)
end

function to_json_cycles_only(obj)
    return to_json_internal(obj, 1, {}, true)
end

function to_json_internal(obj, depth, map, cycles_only)
    if type(obj) ~= 'table' then
        if type(obj) == 'string' then
            return '"' .. obj:gsub('"', '\\"') .. '"'
        else
            local string = tostring(obj)
            if string == 'nan' or string == '-inf' or string == 'inf' then
                return '"' .. string .. '"'
            else
                return string
            end
        end
    end
    if obj.object_name == 'LuaCustomTable' then
        local json = {'['}
        for k,v in pairs(obj) do
            table.insert(json, to_json_internal(v, depth + 1, map))
            table.insert(json, ',\n')
        end
        local size = #json
        if json[size] == ',\n' then
            json[size] = ''
        end
        table.insert(json, ']')
        return table.concat(json, '')
    end

    local json = {'{'}
    local is_array = false
    local class = is_class(obj)
    local cycle, id = is_cycle(obj, map)
    if cycle and (not cycles_only or depth > 1) then
        if depth == 1 then
            return global.lookup.cycles[id].json
        end
        table.insert(json, '"cycle_id":' .. id)
    else
        if class then
            if cycles_only then
                table.insert(json, '"class_id":' .. id .. ',\n')
            else
                table.insert(global.lookup.cycles, {obj = obj, json = ''})
                table.insert(json, '"class_id":' .. #global.lookup.cycles .. ',\n')
            end
        end
        if class or obj.object_name == 'LuaGameScript' then
            table.insert(json, '"serde_tag":"' .. obj.object_name .. '",\n')
        end
        local values = get_values(obj)
        is_array = values[1] ~= nil or table_size(values) == 0

        if depth == 1 and not is_array then
            local typ = 'concept'
             if class or obj.object_name == 'LuaGameScript' then
                typ = 'class'
             elseif type(obj.name) == 'number' then
                typ = 'event'
             end
             table.insert(json, '"serde_type":"' .. typ .. '",\n')
        end
        for k,v in pairs(values) do
            if is_allowed_to_access_attribute(obj, values, k) then
                local internal = to_json_internal(obj[k], depth + 1, map)
                if internal ~= 'nil' then
                    if not is_array then
                        table.insert(json, '"' .. k .. '":')
                    end
                    table.insert(json, internal)
                    table.insert(json, ',\n')
                end
            end
        end
        local size = #json
        if json[size] == ',\n' then
            json[size] = ''
        end
    end
    if is_array then
        json[1] = '['
        table.insert(json, ']')
    else
        table.insert(json, '}')
    end
    local obj_json = table.concat(json, '')
    if class and not cycle then
        local _, index = is_cycle(obj, {})
        global.lookup.cycles[index].json = obj_json
    end
    return obj_json
end

-- Prepare lookup table

--global.lookup = nil
if not global.lookup then
    global.lookup = {}
end
if not global.lookup.cycles then
    global.lookup.cycles = {}
end
global.lookup.subclasses = {
    LuaEntity = {
        armed = "LandMine",
        chain_signal_state = "RailChainSignal",
        ghost_name = "Ghost",
        character_corpse_death_cause = "CharacterCorpse",
        player = "Character",
        tick_of_last_attack = "Character",
        tick_of_last_damage = "Character",
        driver_is_gunner = {"Car", "SpiderVehicle"},
        associated_player = "Character",
        autopilot_destinations = "SpiderVehicle",
        power_usage = "ElectricEnergyInterface",
        character_corpse_tick_of_death = "CharacterCorpse",
        bonus_progress = "CraftingMachine",
        pickup_position = "Inserter",
        distraction_command = "Unit",
        products_finished = "CraftingMachine",
        parameters = "ProgrammableSpeaker",
        timeout = "LandMine",
        effectivity_modifier = "Car",
        remove_unfiltered_items = "InfinityContainer",
        recipe_locked = "AssemblingMachine",
        linked_belt_type = "LinkedBelt",
        crafting_speed = "CraftingMachine",
        trains_in_block = "Rail",
        last_user = "EntityWithOwner",
        held_stack_position = "Inserter",
        autopilot_destination = "SpiderVehicle",
        character_corpse_player_index = "CharacterCorpse",
        connected_rail = "TrainStop",
        ai_settings = "Unit",
        ghost_localised_description = "Ghost",
        splitter_output_priority = "Splitter",
        vehicle_automatic_targeting_parameters = "SpiderVehicle",
        highlight_box_type = "HighlightBox",
        linked_belt_neighbour = "LinkedBelt",
        allow_dispatching_robots = "Character",
        kills = "Turret",
        moving = "Unit",
        splitter_filter = "Splitter",
        belt_neighbours = "TransportBeltConnectable",
        splitter_input_priority = "Splitter",
        consumption_modifier = "Car",
        ghost_type = "Ghost",
        crafting_progress = "CraftingMachine",
        radar_scan_progress = "Radar",
        stack = "ItemEntity",
        neighbour_bonus = "Reactor",
        time_to_next_effect = "SmokeWithTrigger",
        held_stack = "Inserter",
        ghost_localised_name = "Ghost",
        power_production = "ElectricEnergyInterface",
        connected_rail_direction = "TrainStop",
        pump_rail_target = "Pump",
        unit_group = "Unit",
        highlight_box_blink_interval = "HighlightBox",
        to_be_looted = "ItemEntity",
        infinity_container_filters = "InfinityContainer",
        energy_generated_last_tick = "Generator",
        belt_to_ground_type = "TransportBeltToGround",
        ghost_prototype = "Ghost",
        auto_launch = "RocketSilo",
        mining_target = "MiningDrill",
        enable_logistics_while_moving = "Vehicle",
        pickup_target = "Inserter",
        text = "FlyingText",
        trains_limit = "TrainStop",
        electric_network_statistics = "ElectricPole",
        alert_parameters = "ProgrammableSpeaker",
        command = "Unit",
        damage_dealt = "Turret",
        effective_speed = "Unit",
        follow_target = "SpiderVehicle",
        previous_recipe = "Furnace",
        selected_gun_index = {"Character", "Car", "SpiderVehicle"},
        signal_state = {"RailSignal", "RailChainSignal"},
        amount = "ResourceEntity",
        follow_offset = "SpiderVehicle",
        initial_amount = "ResourceEntity",
        friction_modifier = "Car",
        rocket_parts = "RocketSilo",
        loader_container = "Loader",
        torso_orientation = "SpiderVehicle",
        trains_count = "TrainStop",
        ghost_unit_number = "EntityGhost",
        loader_type = "Loader",
        inserter_filter_mode = "Inserter",
        -- Added manually:
        build_distance = "Character",
        character_additional_mining_categories = "Character",
        character_build_distance_bonus = "Character",
        character_crafting_speed_modifier = "Character",
        character_health_bonus = "Character",
        character_inventory_slots_bonus = "Character",
        character_item_drop_distance_bonus = "Character",
        character_item_pickup_distance_bonus = "Character",
        character_loot_pickup_distance_bonus = "Character",
        character_maximum_following_robot_count_bonus = "Character",
        character_mining_progress = "Character",
        character_mining_speed_modifier = "Character",
        character_personal_logistic_requests_enabled = "Character",
        character_reach_distance_bonus = "Character",
        character_resource_reach_distance_bonus = "Character",
        character_running_speed = "Character",
        character_running_speed_modifier = "Character",
        character_trash_slot_count_bonus = "Character",
        cheat_mode = "Character",
        cliff_orientation = "Cliff",
        combat_robot_owner = "CombatRobot",
        corpse_expires = "Corpse",
        corpse_immune_to_entity_placement = "Corpse",
        crafting_queue = "Character",
        crafting_queue_progress = "Character",
        crafting_queue_size = "Character",
        cursor_ghost = "Character",
        cursor_stack = "Character",
        driving = "Character",
        drop_item_distance = "Character",
        following_robots = "Character",
        in_combat = "Character",
        inserter_stack_size_override = "Inserter",
        inserter_target_pickup_count = "Inserter",
        item_pickup_distance = "Character",
        item_requests = {"Ghost", "ItemRequestProxy"},
        link_id = "LinkedContainer",
        loot_pickup_distance = "Character",
        mining_state = "Character",
        neighbours = "TODO!!!",
        opened = "Character",
        opened_gui_type = "Character",
        picking_state = "Character",
        power_switch_state = "PowerSwitch",
        proxy_target = "ItemRequestProxy",
        reach_distance = "Character",
        repair_state = "Character",
        request_from_buffers = "TODO!!!",
        resource_reach_distance = "Character",
        -- TODO?
        riding_state = {"Character", "Player", "Car"},
        rocket_silo_status = "RocketSilo",
        selected = "Character",
        shooting_state = "Character",
        shooting_target = "Turret",
        spawner = "Unit",
        sticked_to = "Sticker",
        storage_filter = "LogisticStorageContainer",
        time_to_live = "TODO!!!",
        tree_color_index = "Tree",
        tree_color_index_max = "Tree",
        tree_gray_stage_index = "Tree",
        tree_gray_stage_index_max = "Tree",
        tree_stage_index = "Tree",
        tree_stage_index_max = "Tree",
        units = "UnitSpawner",
        vehicle = "Character",
        vehicle_logistic_requests_enabled = "SpiderVehicle",
        walking_state = "Character",
    },
    LuaEntityPrototype = {
        adjacent_tile_collision_test = "OffshorePump",
        rocket_entity_prototype = "RocketSilo",
        resistances = "EntityWithHealth",
        mining_speed = {"MiningDrill", "Character"},
        ticks_to_stay_in_combat = "Character",
        belt_speed = "TransportBeltConnectable",
        allow_access_to_all_forces = "Market",
        construction_radius = "Roboport",
        max_polyphony = "ProgrammableSpeaker",
        use_exact_mode = "LogisticContainer",
        max_pursue_distance = "Unit",
        energy_per_move = "FlyingRobot",
        repair_speed_modifier = "EntityWithHealth",
        build_distance = "Character",
        timeout = "LandMine",
        distraction_cooldown = "Unit",
        maximum_temperature = "Generator",
        max_power_output = {"BurnerGenerator", "Generator"},
        min_to_charge = "FlyingRobot",
        adjacent_tile_collision_mask = "OffshorePump",
        resource_categories = {"MiningDrill", "Character"},
        loot_pickup_distance = "Character",
        neighbour_bonus = "Reactor",
        light_blinking_speed = "RocketSilo",
        effectivity = {"Car", "Generator"},
        scale_fluid_usage = "Generator",
        corpses = "EntityWithHealth",
        belt_length = "Loader",
        lab_inputs = "Lab",
        terrain_friction_modifier = "Vehicle",
        affected_by_tiles = "Unit",
        automated_ammo_count = {"ArtilleryTurret", "AmmoTurret"},
        mining_drill_radius = "MiningDrill",
        speed = {"FlyingRobot", "RollingStock", "Unit"},
        count_as_rock_for_filtered_deconstruction = "SimpleEntity",
        drop_item_distance = "Character",
        fixed_recipe = "AssemblingMachine",
        max_darkness_to_spawn = "Spawner",
        ticks_to_keep_aiming_direction = "Character",
        crafting_speed = "CraftingMachine",
        alert_when_damaged = "EntityWithHealth",
        fluid = "OffshorePump",
        min_pursue_time = "Unit",
        engine_starting_speed = "RocketSiloRocket",
        allow_burner_leech = "Inserter",
        idle_energy_usage = "RocketSilo",
        inserter_extension_speed = "Inserter",
        target_temperature = "Boiler",
        animation_speed_coefficient = "BeltConnectable",
        max_to_charge = "FlyingRobot",
        rotation_speed = "Car",
        stack = "Inserter",
        door_opening_speed = "RocketSilo",
        explosion_rotate = "Explosion",
        minimum_resource_amount = "ResourceEntity",
        launch_wait_time = "RocketSilo",
        call_for_help_radius = "Spawner",
        allow_passengers = "Vehicle",
        filter_count = {"Inserter", "Loader", "LogisticContainer"},
        rising_speed = "RocketSiloRocket",
        max_payload_size = "RobotWithLogisticsInterface",
        max_friends_around_to_spawn = "Spawner",
        inserter_pickup_position = "Inserter",
        move_while_shooting = "Unit",
        fluid_usage_per_tick = "Generator",
        max_underground_distance = {"UndergroundBelt", "PipeToGround"},
        allow_run_time_change_of_is_military_target = "EntityWithOwner",
        can_open_gates = "Unit",
        consumption = "Car",
        darkness_for_all_lamps_on = "Lamp",
        destroy_non_fuel_fluid = "Generator",
        energy_per_hit_point = "Vehicle",
        ingredient_count = "CraftingMachine",
        braking_force = "Vehicle",
        inserter_chases_belt_items = "Inserter",
        ticks_to_keep_gun = "Character",
        radar_range = "Unit",
        draw_cargo = "RobotWithLogisticsInterface",
        normal_resource_amount = "ResourceEntity",
        weight = "Vehicle",
        cliff_explosive_prototype = "Cliff",
        item_slot_count = "ConstantCombinator",
        pollution_to_join_attack = "Unit",
        min_darkness_to_spawn = "Spawner",
        height = "SpiderVehicle",
        rocket_rising_delay = "RocketSilo",
        flying_acceleration = "RocketSiloRocket",
        logistic_parameters = "Roboport",
        base_productivity = {"CraftingMachine", "Lab", "MiningDrill"},
        active_energy_usage = {"RocketSilo", "Combinator"},
        inserter_rotation_speed = "Inserter",
        is_military_target = "EntityWithOwner",
        belt_distance = "Loader",
        chunk_exploration_radius = "SpiderVehicle",
        inserter_drop_position = "Inserter",
        maximum_corner_sliding_distance = "Character",
        always_on = "Lamp",
        torso_bob_speed = "SpiderVehicle",
        instruments = "ProgrammableSpeaker",
        spawning_radius = "Spawner",
        pumping_speed = {"OffshorePump", "Pump"},
        has_belt_immunity = {"Unit", "Car", "Character"},
        automatic_weapon_cycling = "SpiderVehicle",
        infinite_resource = "ResourceEntity",
        inserter_stack_size_bonus = "Inserter",
        crafting_categories = "CraftingMachine Character",
        tank_driving = "Car",
        trigger_collision_mask = "LandMine",
        max_count_of_owned_units = "Spawner",
        darkness_for_all_lamps_off = "Lamp",
        air_resistance = "RollingStock",
        max_energy = "FlyingRobot",
        explosion_beam = "Explosion",
        rocket_parts_required = "RocketSilo",
        burns_fluid = "Generator",
        indexed_guns = {"Car", "SpiderVehicle", "ArtilleryTurret", "ArtilleryWagon"},
        reach_resource_distance = "Character",
        vision_distance = "Unit",
        alert_when_attacking = "Turret",
        supply_area_distance = {"ElectricPole", "Beacon"},
        tree_color_count = "Tree",
        result_units = "Spawner",
        related_underground_belt = "TransportBelt",
        center_collision_mask = "OffshorePump",
        speed_multiplier_when_out_of_energy = "FlyingRobot",
        dying_speed = "Corpse",
        reach_distance = "Character",
        logistic_radius = "Roboport",
        running_speed = "Character",
        enter_vehicle_distance = "Character",
        distribution_effectivity = "Beacon",
        torso_rotation_speed = "SpiderVehicle",
        logistic_mode = "LogisticContainer",
        item_pickup_distance = "Character",
        max_distance_of_nearby_sector_revealed = "Radar",
        max_speed = {"Projectile", "FlyingRobot"},
        loot = "EntityWithHealth",
        ammo_category = "LandMine",
        character_corpse = "Character",
        allow_custom_vectors = "Inserter",
        researching_speed = "Lab",
        spawning_time_modifier = "Unit",
        chain_shooting_cooldown_modifier = "SpiderVehicle",
        max_distance_of_sector_revealed = "Radar",
        damage_hit_tint = "Character",
        adjacent_tile_collision_box = "OffshorePump",
        flying_speed = "RocketSiloRocket",
        lamp_energy_usage = "RocketSilo",
        friction_force = "Vehicle",
        energy_per_tick = "FlyingRobot",
        respawn_time = "Character",
        spawn_cooldown = "Spawner",
        turret_rotation_speed = "Car",
        container_distance = "Loader",
        final_attack_result = "Projectile",
        resource_category = "ResourceEntity",
        infinite_depletion_resource_amount = "ResourceEntity",
        spawning_spacing = "Spawner",
        turret_range = "Turret",
    },
    LuaEquipmentPrototype = {
        logistic_parameters = "RoboportEquipment",
        movement_bonus = "MovementBonusEquipment",
        attack_parameters = "ActiveDefenseEquipment",
    },
    LuaGuiElement = {
        position = {"camera", "minimap"},
        vertical_centering = "table",
        clear_and_focus_on_right_click = {"textfield", "text-box"},
        horizontal_scroll_policy = "scroll-pane",
        allow_none_state = "switch",
        text = {"textfield", "text-box"},
        allow_negative = "textfield",
        draw_vertical_lines = "table",
        selected_index = {"drop-down", "list-box"},
        left_label_caption = "switch",
        sprite = {"sprite-button", "sprite"},
        is_password = "textfield",
        numeric = "textfield",
        number = "sprite-button",
        tabs = "tabbed-pane",
        state = {"checkbox", "radiobutton"},
        minimap_player_index = "minimap",
        allow_decimal = "textfield",
        entity = {"entity-preview", "camera", "minimap"},
        selectable = "text-box",
        value = "progressbar",
        locked = "choose-elem-button",
        elem_value = "choose-elem-button",
        read_only = "text-box",
        resize_to_sprite = "sprite",
        draw_horizontal_lines = "table",
        mouse_button_filter = {"button", "sprite-button"},
        lose_focus_on_confirm = "textfield",
        surface_index = {"camera", "minimap"},
        auto_center = "frame",
        zoom = {"camera", "minimap"},
        elem_type = "choose-elem-button",
        items = {"drop-down", "list-box"},
        hovered_sprite = "sprite-button",
        badge_text = "tab",
        column_count = "table",
        show_percent_for_small_numbers = "sprite-button",
        elem_filters = "choose-elem-button",
        force = "minimap",
        slider_value = "slider",
        switch_state = "switch",
        vertical_scroll_policy = "scroll-pane",
        clicked_sprite = "sprite-button",
        word_wrap = "text-box",
        right_label_caption = "switch",
        draw_horizontal_line_after_headers = "table",
        right_label_tooltip = "switch",
        drag_target = {"flow", "frame", "label", "table", "empty-widget"},
        direction = {"frame", "flow", "line"},
        selected_tab_index = "tabbed-pane",
        left_label_tooltip = "switch",
    },
    LuaItemPrototype = {
        category = "ModuleItem",
        entity_filters = "SelectionTool",
        durability_description_key = "ToolItem",
        alt_selection_mode_flags = "SelectionTool",
        alt_tile_filters = "SelectionTool",
        curved_rail = "RailPlanner",
        entity_filter_mode = "SelectionTool",
        reverse_selection_cursor_box_type = "SelectionTool",
        filter_mode = "ItemWithInventory",
        reverse_entity_filters = "SelectionTool",
        infinite = "ToolItem",
        extend_inventory_by_default = "ItemWithInventory",
        module_effects = "ModuleItem",
        reverse_tile_filters = "SelectionTool",
        selection_border_color = "SelectionTool",
        alt_entity_filter_mode = "SelectionTool",
        attack_parameters = "Gun",
        alt_reverse_tile_filter_mode = "SelectionTool",
        alt_selection_cursor_box_type = "SelectionTool",
        reverse_alt_entity_filter_mode = "SelectionTool",
        item_subgroup_filters = "ItemWithInventory",
        selection_cursor_box_type = "SelectionTool",
        alt_entity_filters = "SelectionTool",
        alt_reverse_entity_filters = "SelectionTool",
        straight_rail = "RailPlanner",
        durability = "ToolItem",
        localised_filter_message = "ItemWithInventory",
        tile_filter_mode = "SelectionTool",
        inventory_size = "ItemWithInventoryPrototype",
        alt_reverse_alt_entity_filter_mode = "SelectionTool",
        tile_filter_slots = "DeconstructionItem",
        alt_entity_type_filters = "SelectionTool",
        always_include_tiles = "SelectionTool",
        mapper_count = "UpgradeItem",
        draw_label_for_cursor_render = "ItemWithLabel",
        limitations = "ModuleItem",
        default_label_color = "ItemWithLabel",
        item_group_filters = "ItemWithInventory",
        magazine_size = "AmmoItem",
        alt_reverse_selection_border_color = "SelectionTool",
        alt_reverse_selection_cursor_box_type = "SelectionTool",
        selection_mode_flags = "SelectionTool",
        reverse_selection_mode_flags = "SelectionTool",
        alt_tile_filter_mode = "SelectionTool",
        reload_time = "AmmoItem",
        capsule_action = "Capsule",
        inventory_size_bonus = "ArmorPrototype",
        alt_reverse_tile_filters = "SelectionTool",
        alt_reverse_selection_mode_flags = "SelectionTool",
        entity_filter_slots = "DeconstructionItem",
        speed = "RepairTool",
        reverse_entity_type_filters = "SelectionTool",
        entity_type_filters = "SelectionTool",
        alt_reverse_entity_type_filters = "SelectionTool",
        alt_selection_border_color = "SelectionTool",
        resistances = "Armor",
        item_filters = "ItemWithInventory",
        equipment_grid = "Armor",
        insertion_priority_mode = "ItemWithInventory",
        limitation_message_key = "ModuleItem",
        reverse_selection_border_color = "SelectionTool",
        repair_result = "RepairTool",
        reverse_tile_filter_mode = "SelectionTool",
        tier = "ModuleItem",
        tile_filters = "SelectionTool",
    },
    LuaItemStack = {
        allow_manual_label_change = "ItemWithLabel",
        tags = "ItemWithTags",
        prioritize_insertion_mode = "ItemWithInventory",
        blueprint_position_relative_to_grid = "BlueprintItem",
        cost_to_build = "BlueprintItem",
        ammo = "AmmoItem",
        entity_label = "ItemWithEntityData",
        tile_filter_mode = "DeconstructionItem",
        trees_and_rocks_only = "DeconstructionItem",
        default_icons = "BlueprintItem",
        extends_inventory = "ItemWithInventory",
        label_color = "ItemWithLabel",
        durability = "Tool",
        entity_color = "ItemWithEntityData",
        blueprint_absolute_snapping = "BlueprintItem",
        connected_entity = "SpidertronRemote",
        entity_filter_mode = "DeconstructionItem",
        active_index = "BlueprintBookItem",
        blueprint_snap_to_grid = "BlueprintItem",
        label = "ItemWithLabel",
        entity_filter_count = "DeconstructionItem",
        tile_selection_mode = "DeconstructionItem",
        tile_filter_count = "DeconstructionItem",
        blueprint_icons = "BlueprintItem",
    },
    LuaStyle = {
        disabled_badge_font_color = "TabStyle",
        horizontal_spacing = {"LuaTableStyle", "LuaFlowStyle", "LuaHorizontalFlowStyle"},
        clicked_font_color = "LuaButtonStyle",
        disabled_font_color = {"LuaButtonStyle", "LuaTabStyle"},
        selected_badge_font_color = "TabStyle",
        extra_bottom_padding_when_activated = "ScrollPaneStyle",
        top_cell_padding = "LuaTableStyle",
        vertical_spacing = {"LuaTableStyle", "LuaFlowStyle", "LuaVerticalFlowStyle", "LuaTabbedPaneStyle"},
        color = "LuaProgressBarStyle",
        badge_font = "TabStyle",
        left_cell_padding = "LuaTableStyle",
        bottom_cell_padding = "LuaTableStyle",
        pie_progress_color = "LuaButtonStyle",
        right_cell_padding = "LuaTableStyle",
        extra_top_margin_when_activated = "ScrollPaneStyle",
        selected_hovered_font_color = "LuaButtonStyle",
        extra_left_padding_when_activated = "ScrollPaneStyle",
        hovered_font_color = "LuaButtonStyle",
        strikethrough_color = "LuaButtonStyle",
        clicked_vertical_offset = "LuaButtonStyle",
        selected_font_color = "LuaButtonStyle",
        extra_top_padding_when_activated = "ScrollPaneStyle",
        cell_padding = "LuaTableStyle",
        default_badge_font_color = "TabStyle",
        extra_left_margin_when_activated = "ScrollPaneStyle",
        extra_right_padding_when_activated = "ScrollPaneStyle",
        rich_text_setting = {"LuaLabelStyle", "LuaTextBoxStyle", "LuaTextFieldStyle"},
        badge_horizontal_spacing = "TabStyle",
        single_line = "LabelStyle",
        stretch_image_to_widget_size = "ImageStyle",
        use_header_filler = "LuaFrameStyle",
        extra_right_margin_when_activated = "ScrollPaneStyle",
        selected_clicked_font_color = "LuaButtonStyle",
        extra_bottom_margin_when_activated = "ScrollPaneStyle",
        bar_width = "LuaProgressBarStyle",
        -- Added manually:
        column_alignments = "LuaTableStyle",
        font_color = "TODO!!!",
    }
}

-- Note: all classes except LuaControl, LuaControlBehavior and LuaCombinatorControlBehavior have member object_name
-- Note: game class is not serializable