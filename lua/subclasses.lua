global.lua_objects.subclasses = {
LuaEntity = {
    absorbed_pollution = {["Spawner"] = 0},
    ai_settings = {["Unit"] = 0},
    alert_parameters = {["ProgrammableSpeaker"] = 0},
    allow_dispatching_robots = {["Character"] = 0},
    amount = {["ResourceEntity"] = 0},
    armed = {["LandMine"] = 0},
    associated_player = {["Character"] = 0},
    auto_launch = {["RocketSilo"] = 0},
    autopilot_destination = {["SpiderVehicle"] = 0},
    autopilot_destinations = {["SpiderVehicle"] = 0},
    belt_neighbours = {["TransportBeltConnectable"] = 0},
    belt_shape = {["TransportBelt"] = 0},
    belt_to_ground_type = {["TransportBeltToGround"] = 0},
    bonus_progress = {["CraftingMachine"] = 0},
    chain_signal_state = {["RailChainSignal"] = 0},
    character_corpse_death_cause = {["CharacterCorpse"] = 0},
    character_corpse_player_index = {["CharacterCorpse"] = 0},
    character_corpse_tick_of_death = {["CharacterCorpse"] = 0},
    command = {["Unit"] = 0},
    connected_rail = {["TrainStop"] = 0},
    connected_rail_direction = {["TrainStop"] = 0},
    consumption_modifier = {["Car"] = 0},
    crafting_progress = {["CraftingMachine"] = 0},
    crafting_speed = {["CraftingMachine"] = 0},
    damage_dealt = {["Turret"] = 0},
    distraction_command = {["Unit"] = 0},
    draw_data = {["RollingStock"] = 0},
    driver_is_gunner = {["Car"] = 0, ["SpiderVehicle"] = 0},
    effective_speed = {["Unit"] = 0},
    effectivity_modifier = {["Car"] = 0},
    electric_network_statistics = {["ElectricPole"] = 0},
    enable_logistics_while_moving = {["Vehicle"] = 0},
    energy_generated_last_tick = {["Generator"] = 0},
    follow_offset = {["SpiderVehicle"] = 0},
    follow_target = {["SpiderVehicle"] = 0},
    friction_modifier = {["Car"] = 0},
    ghost_localised_description = {["Ghost"] = 0},
    ghost_localised_name = {["Ghost"] = 0},
    ghost_name = {["Ghost"] = 0},
    ghost_prototype = {["Ghost"] = 0},
    ghost_type = {["Ghost"] = 0},
    ghost_unit_number = {["EntityGhost"] = 0},
    held_stack = {["Inserter"] = 0},
    held_stack_position = {["Inserter"] = 0},
    highlight_box_blink_interval = {["HighlightBox"] = 0},
    highlight_box_type = {["HighlightBox"] = 0},
    infinity_container_filters = {["InfinityContainer"] = 0},
    initial_amount = {["ResourceEntity"] = 0},
    inserter_filter_mode = {["Inserter"] = 0},
    is_headed_to_trains_front = {["RollingStock"] = 0},
    kills = {["Turret"] = 0},
    last_user = {["EntityWithOwner"] = 0},
    linked_belt_neighbour = {["LinkedBelt"] = 0},
    linked_belt_type = {["LinkedBelt"] = 0},
    loader_container = {["Loader"] = 0},
    loader_type = {["Loader"] = 0},
    mining_target = {["MiningDrill"] = 0},
    moving = {["Unit"] = 0},
    neighbour_bonus = {["Reactor"] = 0},
    parameters = {["ProgrammableSpeaker"] = 0},
    pickup_position = {["Inserter"] = 0},
    pickup_target = {["Inserter"] = 0},
    player = {["Character"] = 0},
    power_production = {["ElectricEnergyInterface"] = 0},
    power_usage = {["ElectricEnergyInterface"] = 0},
    previous_recipe = {["Furnace"] = 0},
    products_finished = {["CraftingMachine"] = 0},
    pump_rail_target = {["Pump"] = 0},
    radar_scan_progress = {["Radar"] = 0},
    recipe_locked = {["AssemblingMachine"] = 0},
    remove_unfiltered_items = {["InfinityContainer"] = 0},
    rocket_parts = {["RocketSilo"] = 0},
    selected_gun_index = {["Character"] = 0, ["Car"] = 0, ["SpiderVehicle"] = 0},
    signal_state = {["RailSignal"] = 0, ["RailChainSignal"] = 0},
    spawn_shift = {["Spawner"] = 0},
    spawning_cooldown = {["Spawner"] = 0},
    splitter_filter = {["Splitter"] = 0},
    splitter_input_priority = {["Splitter"] = 0},
    splitter_output_priority = {["Splitter"] = 0},
    stack = {["ItemEntity"] = 0},
    text = {["FlyingText"] = 0},
    tick_of_last_attack = {["Character"] = 0},
    tick_of_last_damage = {["Character"] = 0},
    time_to_next_effect = {["SmokeWithTrigger"] = 0},
    timeout = {["LandMine"] = 0},
    to_be_looted = {["ItemEntity"] = 0},
    torso_orientation = {["SpiderVehicle"] = 0},
    trains_count = {["TrainStop"] = 0},
    trains_in_block = {["Rail"] = 0},
    trains_limit = {["TrainStop"] = 0},
    unit_group = {["Unit"] = 0},
    units = {["Spawner"] = 0},
    vehicle_automatic_targeting_parameters = {["SpiderVehicle"] = 0},
    build_distance = {["Character"] = 0},
    character_additional_mining_categories = {["Character"] = 0},
    character_build_distance_bonus = {["Character"] = 0},
    character_crafting_speed_modifier = {["Character"] = 0},
    character_health_bonus = {["Character"] = 0},
    character_inventory_slots_bonus = {["Character"] = 0},
    character_item_drop_distance_bonus = {["Character"] = 0},
    character_item_pickup_distance_bonus = {["Character"] = 0},
    character_loot_pickup_distance_bonus = {["Character"] = 0},
    character_maximum_following_robot_count_bonus = {["Character"] = 0},
    character_mining_progress = {["Character"] = 0},
    character_mining_speed_modifier = {["Character"] = 0},
    character_personal_logistic_requests_enabled = {["Character"] = 0},
    character_reach_distance_bonus = {["Character"] = 0},
    character_resource_reach_distance_bonus = {["Character"] = 0},
    character_running_speed = {["Character"] = 0},
    character_running_speed_modifier = {["Character"] = 0},
    character_trash_slot_count_bonus = {["Character"] = 0},
    cheat_mode = {["Character"] = 0},
    cliff_orientation = {["Cliff"] = 0},
    combat_robot_owner = {["CombatRobot"] = 0},
    corpse_expires = {["Corpse"] = 0},
    corpse_immune_to_entity_placement = {["Corpse"] = 0},
    crafting_queue = {["Character"] = 0},
    crafting_queue_progress = {["Character"] = 0},
    crafting_queue_size = {["Character"] = 0},
    cursor_ghost = {["Character"] = 0},
    cursor_stack = {["Character"] = 0},
    driving = {["Character"] = 0},
    drop_item_distance = {["Character"] = 0},
    following_robots = {["Character"] = 0},
    in_combat = {["Character"] = 0},
    inserter_stack_size_override = {["Inserter"] = 0},
    inserter_target_pickup_count = {["Inserter"] = 0},
    item_pickup_distance = {["Character"] = 0},
    item_requests = {["Ghost"] = 0, ["ItemRequestProxy"] = 0},
    link_id = {["LinkedContainer"] = 0},
    loot_pickup_distance = {["Character"] = 0},
    mining_state = {["Character"] = 0},
    neighbours = {["ElectricPole"] = 0, ["PowerSwitch"] = 0, ["UndergroundBelt"] = 0, ["Wall"] = 0, ["Gate"] = 0, ["Reactor"] = 0, ["Cliff"] = 0, ["PipeConnectable"] = 0},
    opened = {["Character"] = 0},
    opened_gui_type = {["Character"] = 0},
    picking_state = {["Character"] = 0},
    power_switch_state = {["PowerSwitch"] = 0},
    proxy_target = {["ItemRequestProxy"] = 0},
    reach_distance = {["Character"] = 0},
    repair_state = {["Character"] = 0},
    request_from_buffers = {["RequestSlot"] = 0},
    resource_reach_distance = {["Character"] = 0},
    riding_state = {["Character"] = 0, ["Player"] = 0, ["Car"] = 0},
    rocket_silo_status = {["RocketSilo"] = 0},
    selected = {["Character"] = 0},
    shooting_state = {["Character"] = 0},
    shooting_target = {["Turret"] = 0},
    spawner = {["Unit"] = 0},
    sticked_to = {["Sticker"] = 0},
    storage_filter = {["LogisticStorageContainer"] = 0},
    time_to_live = {["Ghost"] = 0, ["CombatRobot"] = 0, ["HighlightBox"] = 0, ["SmokeWithTrigger"] = 0},
    tree_color_index = {["Tree"] = 0},
    tree_color_index_max = {["Tree"] = 0},
    tree_gray_stage_index = {["Tree"] = 0},
    tree_gray_stage_index_max = {["Tree"] = 0},
    tree_stage_index = {["Tree"] = 0},
    tree_stage_index_max = {["Tree"] = 0},
    units = {["UnitSpawner"] = 0},
    vehicle = {["Character"] = 0},
    vehicle_logistic_requests_enabled = {["SpiderVehicle"] = 0},
    walking_state = {["Character"] = 0},
},
LuaEntityPrototype = {
    active_energy_usage = {["RocketSilo"] = 0, ["Combinator"] = 0},
    adjacent_tile_collision_box = {["OffshorePump"] = 0},
    adjacent_tile_collision_mask = {["OffshorePump"] = 0},
    adjacent_tile_collision_test = {["OffshorePump"] = 0},
    affected_by_tiles = {["Unit"] = 0},
    air_resistance = {["RollingStock"] = 0},
    alert_when_attacking = {["Turret"] = 0},
    alert_when_damaged = {["EntityWithHealth"] = 0},
    allow_access_to_all_forces = {["Market"] = 0},
    allow_burner_leech = {["Inserter"] = 0},
    allow_custom_vectors = {["Inserter"] = 0},
    allow_passengers = {["Vehicle"] = 0},
    allow_run_time_change_of_is_military_target = {["EntityWithOwner"] = 0},
    always_on = {["Lamp"] = 0},
    ammo_category = {["LandMine"] = 0},
    animation_speed_coefficient = {["BeltConnectable"] = 0},
    automated_ammo_count = {["ArtilleryTurret"] = 0, ["AmmoTurret"] = 0},
    automatic_weapon_cycling = {["SpiderVehicle"] = 0},
    base_productivity = {["CraftingMachine"] = 0, ["Lab"] = 0, ["MiningDrill"] = 0},
    belt_distance = {["Loader"] = 0},
    belt_length = {["Loader"] = 0},
    belt_speed = {["TransportBeltConnectable"] = 0},
    boiler_mode = {["Boiler"] = 0},
    braking_force = {["Vehicle"] = 0},
    build_distance = {["Character"] = 0},
    burns_fluid = {["Generator"] = 0},
    call_for_help_radius = {["Spawner"] = 0},
    can_open_gates = {["Unit"] = 0},
    center_collision_mask = {["OffshorePump"] = 0},
    chain_shooting_cooldown_modifier = {["SpiderVehicle"] = 0},
    character_corpse = {["Character"] = 0},
    chunk_exploration_radius = {["SpiderVehicle"] = 0},
    cliff_explosive_prototype = {["Cliff"] = 0},
    connection_distance = {["RollingStock"] = 0},
    construction_radius = {["Roboport"] = 0},
    consumption = {["Car"] = 0},
    container_distance = {["Loader"] = 0},
    corpses = {["EntityWithHealth"] = 0},
    count_as_rock_for_filtered_deconstruction = {["SimpleEntity"] = 0},
    crafting_categories = {["CraftingMachine Character"] = 0},
    crafting_speed = {["CraftingMachine"] = 0},
    damage_hit_tint = {["Character"] = 0},
    darkness_for_all_lamps_off = {["Lamp"] = 0},
    darkness_for_all_lamps_on = {["Lamp"] = 0},
    destroy_non_fuel_fluid = {["Generator"] = 0},
    distraction_cooldown = {["Unit"] = 0},
    distribution_effectivity = {["Beacon"] = 0},
    door_opening_speed = {["RocketSilo"] = 0},
    draw_cargo = {["RobotWithLogisticsInterface"] = 0},
    drop_item_distance = {["Character"] = 0},
    dying_speed = {["Corpse"] = 0},
    effectivity = {["Car"] = 0, ["Generator"] = 0},
    energy_per_hit_point = {["Vehicle"] = 0},
    energy_per_move = {["FlyingRobot"] = 0},
    energy_per_tick = {["FlyingRobot"] = 0},
    engine_starting_speed = {["RocketSiloRocket"] = 0},
    enter_vehicle_distance = {["Character"] = 0},
    explosion_beam = {["Explosion"] = 0},
    explosion_rotate = {["Explosion"] = 0},
    filter_count = {["Inserter"] = 0, ["Loader"] = 0, ["LogisticContainer"] = 0},
    final_attack_result = {["Projectile"] = 0},
    fixed_recipe = {["AssemblingMachine"] = 0},
    fluid = {["OffshorePump"] = 0},
    fluid_usage_per_tick = {["Generator"] = 0},
    flying_acceleration = {["RocketSiloRocket"] = 0},
    flying_speed = {["RocketSiloRocket"] = 0},
    friction_force = {["Vehicle"] = 0},
    has_belt_immunity = {["Unit"] = 0, ["Car"] = 0, ["Character"] = 0},
    height = {["SpiderVehicle"] = 0},
    indexed_guns = {["Car"] = 0, ["SpiderVehicle"] = 0, ["ArtilleryTurret"] = 0, ["ArtilleryWagon"] = 0},
    infinite_depletion_resource_amount = {["ResourceEntity"] = 0},
    infinite_resource = {["ResourceEntity"] = 0},
    ingredient_count = {["CraftingMachine"] = 0},
    inserter_chases_belt_items = {["Inserter"] = 0},
    inserter_drop_position = {["Inserter"] = 0},
    inserter_extension_speed = {["Inserter"] = 0},
    inserter_pickup_position = {["Inserter"] = 0},
    inserter_rotation_speed = {["Inserter"] = 0},
    inserter_stack_size_bonus = {["Inserter"] = 0},
    instruments = {["ProgrammableSpeaker"] = 0},
    is_military_target = {["EntityWithOwner"] = 0},
    item_pickup_distance = {["Character"] = 0},
    item_slot_count = {["ConstantCombinator"] = 0},
    joint_distance = {["RollingStock"] = 0},
    lab_inputs = {["Lab"] = 0},
    lamp_energy_usage = {["RocketSilo"] = 0},
    launch_wait_time = {["RocketSilo"] = 0},
    light_blinking_speed = {["RocketSilo"] = 0},
    logistic_mode = {["LogisticContainer"] = 0},
    logistic_parameters = {["Roboport"] = 0},
    logistic_radius = {["Roboport"] = 0},
    loot = {["EntityWithHealth"] = 0},
    loot_pickup_distance = {["Character"] = 0},
    max_count_of_owned_units = {["Spawner"] = 0},
    max_darkness_to_spawn = {["Spawner"] = 0},
    max_distance_of_nearby_sector_revealed = {["Radar"] = 0},
    max_distance_of_sector_revealed = {["Radar"] = 0},
    max_energy = {["FlyingRobot"] = 0},
    max_friends_around_to_spawn = {["Spawner"] = 0},
    max_payload_size = {["RobotWithLogisticsInterface"] = 0},
    max_polyphony = {["ProgrammableSpeaker"] = 0},
    max_power_output = {["BurnerGenerator"] = 0, ["Generator"] = 0},
    max_pursue_distance = {["Unit"] = 0},
    max_speed = {["Projectile"] = 0, ["FlyingRobot"] = 0},
    max_to_charge = {["FlyingRobot"] = 0},
    max_underground_distance = {["UndergroundBelt"] = 0, ["PipeToGround"] = 0},
    maximum_corner_sliding_distance = {["Character"] = 0},
    maximum_temperature = {["Generator"] = 0},
    min_darkness_to_spawn = {["Spawner"] = 0},
    min_pursue_time = {["Unit"] = 0},
    min_to_charge = {["FlyingRobot"] = 0},
    minimum_resource_amount = {["ResourceEntity"] = 0},
    mining_drill_radius = {["MiningDrill"] = 0},
    mining_speed = {["MiningDrill"] = 0, ["Character"] = 0},
    move_while_shooting = {["Unit"] = 0},
    neighbour_bonus = {["Reactor"] = 0},
    normal_resource_amount = {["ResourceEntity"] = 0},
    pollution_to_join_attack = {["Unit"] = 0},
    pumping_speed = {["OffshorePump"] = 0, ["Pump"] = 0},
    radar_range = {["Unit"] = 0},
    reach_distance = {["Character"] = 0},
    reach_resource_distance = {["Character"] = 0},
    related_underground_belt = {["TransportBelt"] = 0},
    repair_speed_modifier = {["EntityWithHealth"] = 0},
    researching_speed = {["Lab"] = 0},
    resistances = {["EntityWithHealth"] = 0},
    resource_categories = {["MiningDrill"] = 0, ["Character"] = 0},
    resource_category = {["ResourceEntity"] = 0},
    respawn_time = {["Character"] = 0},
    result_units = {["Spawner"] = 0},
    rising_speed = {["RocketSiloRocket"] = 0},
    rocket_entity_prototype = {["RocketSilo"] = 0},
    rocket_parts_required = {["RocketSilo"] = 0},
    rocket_rising_delay = {["RocketSilo"] = 0},
    rotation_speed = {["Car"] = 0},
    running_speed = {["Character"] = 0},
    scale_fluid_usage = {["Generator"] = 0},
    spawn_cooldown = {["Spawner"] = 0},
    spawn_decoration = {["Spawner"] = 0, ["Turret"] = 0},
    spawn_decorations_on_expansion = {["Spawner"] = 0, ["Turret"] = 0},
    spawning_radius = {["Spawner"] = 0},
    spawning_spacing = {["Spawner"] = 0},
    spawning_time_modifier = {["Unit"] = 0},
    speed = {["FlyingRobot"] = 0, ["RollingStock"] = 0, ["Unit"] = 0},
    speed_multiplier_when_out_of_energy = {["FlyingRobot"] = 0},
    stack = {["Inserter"] = 0},
    supply_area_distance = {["ElectricPole"] = 0, ["Beacon"] = 0},
    tank_driving = {["Car"] = 0},
    target_temperature = {["Boiler"] = 0},
    terrain_friction_modifier = {["Vehicle"] = 0},
    ticks_to_keep_aiming_direction = {["Character"] = 0},
    ticks_to_keep_gun = {["Character"] = 0},
    ticks_to_stay_in_combat = {["Character"] = 0},
    timeout = {["LandMine"] = 0},
    torso_bob_speed = {["SpiderVehicle"] = 0},
    torso_rotation_speed = {["SpiderVehicle"] = 0},
    tree_color_count = {["Tree"] = 0},
    trigger_collision_mask = {["LandMine"] = 0},
    turret_range = {["Turret"] = 0},
    turret_rotation_speed = {["Car"] = 0},
    use_exact_mode = {["LogisticContainer"] = 0},
    vertical_selection_shift = {["RollingStock"] = 0},
    vision_distance = {["Unit"] = 0},
    weight = {["Vehicle"] = 0},
},
LuaEquipmentPrototype = {
    attack_parameters = {["ActiveDefenseEquipment"] = 0},
    logistic_parameters = {["RoboportEquipment"] = 0},
    movement_bonus = {["MovementBonusEquipment"] = 0},
},
LuaGuiElement = {
    allow_decimal = {["textfield"] = 0},
    allow_negative = {["textfield"] = 0},
    allow_none_state = {["switch"] = 0},
    auto_center = {["frame"] = 0},
    auto_toggle = {["button"] = 0, ["sprite-button"] = 0},
    badge_text = {["tab"] = 0},
    clear_and_focus_on_right_click = {["textfield"] = 0, ["text-box"] = 0},
    clicked_sprite = {["sprite-button"] = 0},
    column_count = {["table"] = 0},
    direction = {["frame"] = 0, ["flow"] = 0, ["line"] = 0},
    drag_target = {["flow"] = 0, ["frame"] = 0, ["label"] = 0, ["table"] = 0, ["empty-widget"] = 0},
    draw_horizontal_line_after_headers = {["table"] = 0},
    draw_horizontal_lines = {["table"] = 0},
    draw_vertical_lines = {["table"] = 0},
    elem_filters = {["choose-elem-button"] = 0},
    elem_type = {["choose-elem-button"] = 0},
    elem_value = {["choose-elem-button"] = 0},
    entity = {["entity-preview"] = 0, ["camera"] = 0, ["minimap"] = 0},
    force = {["minimap"] = 0},
    horizontal_scroll_policy = {["scroll-pane"] = 0},
    hovered_sprite = {["sprite-button"] = 0},
    is_password = {["textfield"] = 0},
    items = {["drop-down"] = 0, ["list-box"] = 0},
    left_label_caption = {["switch"] = 0},
    left_label_tooltip = {["switch"] = 0},
    locked = {["choose-elem-button"] = 0},
    lose_focus_on_confirm = {["textfield"] = 0},
    minimap_player_index = {["minimap"] = 0},
    mouse_button_filter = {["button"] = 0, ["sprite-button"] = 0},
    number = {["sprite-button"] = 0},
    numeric = {["textfield"] = 0},
    position = {["camera"] = 0, ["minimap"] = 0},
    read_only = {["text-box"] = 0},
    resize_to_sprite = {["sprite"] = 0},
    right_label_caption = {["switch"] = 0},
    right_label_tooltip = {["switch"] = 0},
    selectable = {["text-box"] = 0},
    selected_index = {["drop-down"] = 0, ["list-box"] = 0},
    selected_tab_index = {["tabbed-pane"] = 0},
    show_percent_for_small_numbers = {["sprite-button"] = 0},
    slider_value = {["slider"] = 0},
    sprite = {["sprite-button"] = 0, ["sprite"] = 0},
    state = {["checkbox"] = 0, ["radiobutton"] = 0},
    surface_index = {["camera"] = 0, ["minimap"] = 0},
    switch_state = {["switch"] = 0},
    tabs = {["tabbed-pane"] = 0},
    text = {["textfield"] = 0, ["text-box"] = 0},
    toggled = {["button"] = 0, ["sprite-button"] = 0},
    value = {["progressbar"] = 0},
    vertical_centering = {["table"] = 0},
    vertical_scroll_policy = {["scroll-pane"] = 0},
    word_wrap = {["text-box"] = 0},
    zoom = {["camera"] = 0, ["minimap"] = 0},
},
LuaItemPrototype = {
    alt_entity_filter_mode = {["SelectionTool"] = 0},
    alt_entity_filters = {["SelectionTool"] = 0},
    alt_entity_type_filters = {["SelectionTool"] = 0},
    alt_reverse_alt_entity_filter_mode = {["SelectionTool"] = 0},
    alt_reverse_entity_filters = {["SelectionTool"] = 0},
    alt_reverse_entity_type_filters = {["SelectionTool"] = 0},
    alt_reverse_selection_border_color = {["SelectionTool"] = 0},
    alt_reverse_selection_cursor_box_type = {["SelectionTool"] = 0},
    alt_reverse_selection_mode_flags = {["SelectionTool"] = 0},
    alt_reverse_tile_filter_mode = {["SelectionTool"] = 0},
    alt_reverse_tile_filters = {["SelectionTool"] = 0},
    alt_selection_border_color = {["SelectionTool"] = 0},
    alt_selection_cursor_box_type = {["SelectionTool"] = 0},
    alt_selection_mode_flags = {["SelectionTool"] = 0},
    alt_tile_filter_mode = {["SelectionTool"] = 0},
    alt_tile_filters = {["SelectionTool"] = 0},
    always_include_tiles = {["SelectionTool"] = 0},
    attack_parameters = {["Gun"] = 0},
    capsule_action = {["Capsule"] = 0},
    category = {["ModuleItem"] = 0},
    curved_rail = {["RailPlanner"] = 0},
    default_label_color = {["ItemWithLabel"] = 0},
    draw_label_for_cursor_render = {["ItemWithLabel"] = 0},
    durability = {["ToolItem"] = 0},
    durability_description_key = {["ToolItem"] = 0},
    entity_filter_mode = {["SelectionTool"] = 0},
    entity_filter_slots = {["DeconstructionItem"] = 0},
    entity_filters = {["SelectionTool"] = 0},
    entity_type_filters = {["SelectionTool"] = 0},
    equipment_grid = {["Armor"] = 0},
    extend_inventory_by_default = {["ItemWithInventory"] = 0},
    filter_mode = {["ItemWithInventory"] = 0},
    infinite = {["ToolItem"] = 0},
    insertion_priority_mode = {["ItemWithInventory"] = 0},
    inventory_size = {["ItemWithInventoryPrototype"] = 0},
    inventory_size_bonus = {["ArmorPrototype"] = 0},
    item_filters = {["ItemWithInventory"] = 0},
    item_group_filters = {["ItemWithInventory"] = 0},
    item_subgroup_filters = {["ItemWithInventory"] = 0},
    limitation_message_key = {["ModuleItem"] = 0},
    limitations = {["ModuleItem"] = 0},
    localised_filter_message = {["ItemWithInventory"] = 0},
    magazine_size = {["AmmoItem"] = 0},
    mapper_count = {["UpgradeItem"] = 0},
    module_effects = {["ModuleItem"] = 0},
    reload_time = {["AmmoItem"] = 0},
    repair_result = {["RepairTool"] = 0},
    resistances = {["Armor"] = 0},
    reverse_alt_entity_filter_mode = {["SelectionTool"] = 0},
    reverse_entity_filters = {["SelectionTool"] = 0},
    reverse_entity_type_filters = {["SelectionTool"] = 0},
    reverse_selection_border_color = {["SelectionTool"] = 0},
    reverse_selection_cursor_box_type = {["SelectionTool"] = 0},
    reverse_selection_mode_flags = {["SelectionTool"] = 0},
    reverse_tile_filter_mode = {["SelectionTool"] = 0},
    reverse_tile_filters = {["SelectionTool"] = 0},
    selection_border_color = {["SelectionTool"] = 0},
    selection_cursor_box_type = {["SelectionTool"] = 0},
    selection_mode_flags = {["SelectionTool"] = 0},
    speed = {["RepairTool"] = 0},
    straight_rail = {["RailPlanner"] = 0},
    tier = {["ModuleItem"] = 0},
    tile_filter_mode = {["SelectionTool"] = 0},
    tile_filter_slots = {["DeconstructionItem"] = 0},
    tile_filters = {["SelectionTool"] = 0},
},
LuaItemStack = {
    active_index = {["BlueprintBookItem"] = 0},
    allow_manual_label_change = {["ItemWithLabel"] = 0},
    ammo = {["AmmoItem"] = 0},
    blueprint_absolute_snapping = {["BlueprintItem"] = 0},
    blueprint_icons = {["BlueprintItem"] = 0},
    blueprint_position_relative_to_grid = {["BlueprintItem"] = 0},
    blueprint_snap_to_grid = {["BlueprintItem"] = 0},
    connected_entity = {["SpidertronRemote"] = 0},
    cost_to_build = {["BlueprintItem"] = 0},
    default_icons = {["BlueprintItem"] = 0},
    durability = {["Tool"] = 0},
    entity_color = {["ItemWithEntityData"] = 0},
    entity_filter_count = {["DeconstructionItem"] = 0},
    entity_filter_mode = {["DeconstructionItem"] = 0},
    entity_label = {["ItemWithEntityData"] = 0},
    extends_inventory = {["ItemWithInventory"] = 0},
    label = {["ItemWithLabel"] = 0},
    label_color = {["ItemWithLabel"] = 0},
    prioritize_insertion_mode = {["ItemWithInventory"] = 0},
    tags = {["ItemWithTags"] = 0},
    tile_filter_count = {["DeconstructionItem"] = 0},
    tile_filter_mode = {["DeconstructionItem"] = 0},
    tile_selection_mode = {["DeconstructionItem"] = 0},
    trees_and_rocks_only = {["DeconstructionItem"] = 0},
},
LuaStyle = {
    badge_font = {["TabStyle"] = 0},
    badge_horizontal_spacing = {["TabStyle"] = 0},
    bar_width = {["LuaProgressBarStyle"] = 0},
    bottom_cell_padding = {["LuaTableStyle"] = 0},
    cell_padding = {["LuaTableStyle"] = 0},
    clicked_font_color = {["LuaButtonStyle"] = 0},
    clicked_vertical_offset = {["LuaButtonStyle"] = 0},
    color = {["LuaProgressBarStyle"] = 0},
    default_badge_font_color = {["TabStyle"] = 0},
    disabled_badge_font_color = {["TabStyle"] = 0},
    disabled_font_color = {["LuaButtonStyle"] = 0, ["LuaTabStyle"] = 0},
    draw_grayscale_picture = {["LuaButtonStyle"] = 0},
    extra_bottom_margin_when_activated = {["ScrollPaneStyle"] = 0},
    extra_bottom_padding_when_activated = {["ScrollPaneStyle"] = 0},
    extra_left_margin_when_activated = {["ScrollPaneStyle"] = 0},
    extra_left_padding_when_activated = {["ScrollPaneStyle"] = 0},
    extra_right_margin_when_activated = {["ScrollPaneStyle"] = 0},
    extra_right_padding_when_activated = {["ScrollPaneStyle"] = 0},
    extra_top_margin_when_activated = {["ScrollPaneStyle"] = 0},
    extra_top_padding_when_activated = {["ScrollPaneStyle"] = 0},
    horizontal_spacing = {["LuaTableStyle"] = 0, ["LuaFlowStyle"] = 0, ["LuaHorizontalFlowStyle"] = 0},
    hovered_font_color = {["LuaButtonStyle"] = 0},
    left_cell_padding = {["LuaTableStyle"] = 0},
    pie_progress_color = {["LuaButtonStyle"] = 0},
    rich_text_setting = {["LuaLabelStyle"] = 0, ["LuaTextBoxStyle"] = 0, ["LuaTextFieldStyle"] = 0},
    right_cell_padding = {["LuaTableStyle"] = 0},
    selected_badge_font_color = {["TabStyle"] = 0},
    selected_clicked_font_color = {["LuaButtonStyle"] = 0},
    selected_font_color = {["LuaButtonStyle"] = 0},
    selected_hovered_font_color = {["LuaButtonStyle"] = 0},
    single_line = {["LabelStyle"] = 0},
    stretch_image_to_widget_size = {["ImageStyle"] = 0},
    strikethrough_color = {["LuaButtonStyle"] = 0},
    top_cell_padding = {["LuaTableStyle"] = 0},
    use_header_filler = {["LuaFrameStyle"] = 0},
    vertical_spacing = {["LuaTableStyle"] = 0, ["LuaFlowStyle"] = 0, ["LuaVerticalFlowStyle"] = 0, ["LuaTabbedPaneStyle"] = 0},
},
}