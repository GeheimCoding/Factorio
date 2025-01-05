pub mod world_ambient_sound_definition;
pub use world_ambient_sound_definition::WorldAmbientSoundDefinition;
pub mod working_visualisations;
pub use working_visualisations::WorkingVisualisations;
pub mod working_visualisation;
pub use working_visualisation::WorkingVisualisation;
pub mod working_sound;
pub use working_sound::WorkingSound;
pub mod worker_robot_storage_modifier;
pub use worker_robot_storage_modifier::WorkerRobotStorageModifier;
pub mod worker_robot_speed_modifier;
pub use worker_robot_speed_modifier::WorkerRobotSpeedModifier;
pub mod worker_robot_battery_modifier;
pub use worker_robot_battery_modifier::WorkerRobotBatteryModifier;
pub mod wire_position;
pub use wire_position::WirePosition;
pub mod wire_connection_point;
pub use wire_connection_point::WireConnectionPoint;
pub mod weight;
pub use weight::Weight;
pub mod water_tile_effect_parameters;
pub use water_tile_effect_parameters::WaterTileEffectParameters;
pub mod water_reflection_definition;
pub use water_reflection_definition::WaterReflectionDefinition;
pub mod void_energy_source;
pub use void_energy_source::VoidEnergySource;
pub mod visual_state;
pub use visual_state::VisualState;
pub mod virtual_signal_id;
pub use virtual_signal_id::VirtualSignalID;
pub mod vertical_scroll_bar_style_specification;
pub use vertical_scroll_bar_style_specification::VerticalScrollBarStyleSpecification;
pub mod vertical_flow_style_specification;
pub use vertical_flow_style_specification::VerticalFlowStyleSpecification;
pub mod vertical_align;
pub use vertical_align::VerticalAlign;
pub mod vehicle_logistics_modifier;
pub use vehicle_logistics_modifier::VehicleLogisticsModifier;
pub mod vector_4f;
pub use vector_4f::Vector4f;
pub mod vector_3d;
pub use vector_3d::Vector3D;
pub mod vector;
pub use vector::Vector;
pub mod variable_ambient_sound_variable_sound;
pub use variable_ambient_sound_variable_sound::VariableAmbientSoundVariableSound;
pub mod variable_ambient_sound_state_type;
pub use variable_ambient_sound_state_type::VariableAmbientSoundStateType;
pub mod variable_ambient_sound_state;
pub use variable_ambient_sound_state::VariableAmbientSoundState;
pub mod variable_ambient_sound_next_state_trigger;
pub use variable_ambient_sound_next_state_trigger::VariableAmbientSoundNextStateTrigger;
pub mod variable_ambient_sound_next_state_item;
pub use variable_ambient_sound_next_state_item::VariableAmbientSoundNextStateItem;
pub mod variable_ambient_sound_next_state_conditions;
pub use variable_ambient_sound_next_state_conditions::VariableAmbientSoundNextStateConditions;
pub mod variable_ambient_sound_layer_state_properties;
pub use variable_ambient_sound_layer_state_properties::VariableAmbientSoundLayerStateProperties;
pub mod variable_ambient_sound_layer_sample;
pub use variable_ambient_sound_layer_sample::VariableAmbientSoundLayerSample;
pub mod variable_ambient_sound_layer;
pub use variable_ambient_sound_layer::VariableAmbientSoundLayer;
pub mod variable_ambient_sound_composition_mode;
pub use variable_ambient_sound_composition_mode::VariableAmbientSoundCompositionMode;
pub mod use_rail_planner_tip_trigger;
pub use use_rail_planner_tip_trigger::UseRailPlannerTipTrigger;
pub mod use_pipette_tip_trigger;
pub use use_pipette_tip_trigger::UsePipetteTipTrigger;
pub mod use_on_self_capsule_action;
pub use use_on_self_capsule_action::UseOnSelfCapsuleAction;
pub mod use_confirm_tip_trigger;
pub use use_confirm_tip_trigger::UseConfirmTipTrigger;
pub mod unlock_space_location_modifier;
pub use unlock_space_location_modifier::UnlockSpaceLocationModifier;
pub mod unlock_recipe_tip_trigger;
pub use unlock_recipe_tip_trigger::UnlockRecipeTipTrigger;
pub mod unlock_recipe_modifier;
pub use unlock_recipe_modifier::UnlockRecipeModifier;
pub mod unlock_quality_modifier;
pub use unlock_quality_modifier::UnlockQualityModifier;
pub mod unit_spawn_definition;
pub use unit_spawn_definition::UnitSpawnDefinition;
pub mod unit_group_settings;
pub use unit_group_settings::UnitGroupSettings;
pub mod unit_alternative_frame_sequence;
pub use unit_alternative_frame_sequence::UnitAlternativeFrameSequence;
pub mod unit_aisettings;
pub use unit_aisettings::UnitAISettings;
pub mod turret_state;
pub use turret_state::TurretState;
pub mod turret_special_effect_center;
pub use turret_special_effect_center::TurretSpecialEffectCenter;
pub mod turret_special_effect;
pub use turret_special_effect::TurretSpecialEffect;
pub mod turret_graphics_set;
pub use turret_graphics_set::TurretGraphicsSet;
pub mod turret_base_visualisation;
pub use turret_base_visualisation::TurretBaseVisualisation;
pub mod turret_attack_modifier;
pub use turret_attack_modifier::TurretAttackModifier;
pub mod trivial_smoke_id;
pub use trivial_smoke_id::TrivialSmokeID;
pub mod trigger_target_mask;
pub use trigger_target_mask::TriggerTargetMask;
pub mod trigger_item;
pub use trigger_item::TriggerItem;
pub mod trigger_effect_with_cooldown;
pub use trigger_effect_with_cooldown::TriggerEffectWithCooldown;
pub mod trigger_effect_item;
pub use trigger_effect_item::TriggerEffectItem;
pub mod trigger_effect;
pub use trigger_effect::TriggerEffect;
pub mod trigger_delivery_item;
pub use trigger_delivery_item::TriggerDeliveryItem;
pub mod trigger_delivery;
pub use trigger_delivery::TriggerDelivery;
pub mod trigger;
pub use trigger::Trigger;
pub mod tree_variation;
pub use tree_variation::TreeVariation;
pub mod transport_belt_connector_frame;
pub use transport_belt_connector_frame::TransportBeltConnectorFrame;
pub mod transport_belt_animation_set_with_corners;
pub use transport_belt_animation_set_with_corners::TransportBeltAnimationSetWithCorners;
pub mod transport_belt_animation_set;
pub use transport_belt_animation_set::TransportBeltAnimationSet;
pub mod transition_application;
pub use transition_application::TransitionApplication;
pub mod train_visualization_constants;
pub use train_visualization_constants::TrainVisualizationConstants;
pub mod train_stop_light;
pub use train_stop_light::TrainStopLight;
pub mod train_braking_force_bonus_modifier;
pub use train_braking_force_bonus_modifier::TrainBrakingForceBonusModifier;
pub mod toggle_show_entity_info_tip_trigger;
pub use toggle_show_entity_info_tip_trigger::ToggleShowEntityInfoTipTrigger;
pub mod toggle_rail_layer_tip_trigger;
pub use toggle_rail_layer_tip_trigger::ToggleRailLayerTipTrigger;
pub mod tip_trigger;
pub use tip_trigger::TipTrigger;
pub mod tip_status;
pub use tip_status::TipStatus;
pub mod tint_procession_layer;
pub use tint_procession_layer::TintProcessionLayer;
pub mod time_since_last_tip_activation_tip_trigger;
pub use time_since_last_tip_activation_tip_trigger::TimeSinceLastTipActivationTipTrigger;
pub mod time_elapsed_tip_trigger;
pub use time_elapsed_tip_trigger::TimeElapsedTipTrigger;
pub mod tile_transitions_variants;
pub use tile_transitions_variants::TileTransitionsVariants;
pub mod tile_transitions_to_tiles;
pub use tile_transitions_to_tiles::TileTransitionsToTiles;
pub mod tile_transitions_between_transitions;
pub use tile_transitions_between_transitions::TileTransitionsBetweenTransitions;
pub mod tile_transitions;
pub use tile_transitions::TileTransitions;
pub mod tile_transition_variant_layout;
pub use tile_transition_variant_layout::TileTransitionVariantLayout;
pub mod tile_transition_spritesheet_layout;
pub use tile_transition_spritesheet_layout::TileTransitionSpritesheetLayout;
pub mod tile_sprite_layout_variant;
pub use tile_sprite_layout_variant::TileSpriteLayoutVariant;
pub mod tile_sprite_layout;
pub use tile_sprite_layout::TileSpriteLayout;
pub mod tile_render_layer;
pub use tile_render_layer::TileRenderLayer;
pub mod tile_position;
pub use tile_position::TilePosition;
pub mod tile_main_pictures;
pub use tile_main_pictures::TileMainPictures;
pub mod tile_light_pictures;
pub use tile_light_pictures::TileLightPictures;
pub mod tile_id;
pub use tile_id::TileID;
pub mod tile_effect_definition_id;
pub use tile_effect_definition_id::TileEffectDefinitionID;
pub mod tile_buildability_rule;
pub use tile_buildability_rule::TileBuildabilityRule;
pub mod tile_based_particle_tints;
pub use tile_based_particle_tints::TileBasedParticleTints;
pub mod thruster_performance_point;
pub use thruster_performance_point::ThrusterPerformancePoint;
pub mod thruster_graphics_set;
pub use thruster_graphics_set::ThrusterGraphicsSet;
pub mod throw_capsule_action;
pub use throw_capsule_action::ThrowCapsuleAction;
pub mod text_box_style_specification;
pub use text_box_style_specification::TextBoxStyleSpecification;
pub mod territory_settings;
pub use territory_settings::TerritorySettings;
pub mod technology_unit;
pub use technology_unit::TechnologyUnit;
pub mod technology_trigger;
pub use technology_trigger::TechnologyTrigger;
pub mod technology_slot_style_specification;
pub use technology_slot_style_specification::TechnologySlotStyleSpecification;
pub mod technology_id;
pub use technology_id::TechnologyID;
pub mod table_style_specification;
pub use table_style_specification::TableStyleSpecification;
pub mod tabbed_pane_style_specification;
pub use tabbed_pane_style_specification::TabbedPaneStyleSpecification;
pub mod tab_style_specification;
pub use tab_style_specification::TabStyleSpecification;
pub mod switch_style_specification;
pub use switch_style_specification::SwitchStyleSpecification;
pub mod surface_render_parameters;
pub use surface_render_parameters::SurfaceRenderParameters;
pub mod surface_property_id;
pub use surface_property_id::SurfacePropertyID;
pub mod surface_id;
pub use surface_id::SurfaceID;
pub mod surface_condition;
pub use surface_condition::SurfaceCondition;
pub mod style_with_clickable_graphical_set_specification;
pub use style_with_clickable_graphical_set_specification::StyleWithClickableGraphicalSetSpecification;
pub mod style_specification;
pub use style_specification::StyleSpecification;
pub mod stripe;
pub use stripe::Stripe;
pub mod stretch_rule;
pub use stretch_rule::StretchRule;
pub mod stream_trigger_delivery;
pub use stream_trigger_delivery::StreamTriggerDelivery;
pub mod stream_attack_parameters;
pub use stream_attack_parameters::StreamAttackParameters;
pub mod status_colors;
pub use status_colors::StatusColors;
pub mod stateless_visualisations;
pub use stateless_visualisations::StatelessVisualisations;
pub mod stateless_visualisation;
pub use stateless_visualisation::StatelessVisualisation;
pub mod state_steering_settings;
pub use state_steering_settings::StateSteeringSettings;
pub mod stack_transfer_tip_trigger;
pub use stack_transfer_tip_trigger::StackTransferTipTrigger;
pub mod sprite_variations;
pub use sprite_variations::SpriteVariations;
pub mod sprite_usage_surface_hint;
pub use sprite_usage_surface_hint::SpriteUsageSurfaceHint;
pub mod sprite_usage_hint;
pub use sprite_usage_hint::SpriteUsageHint;
pub mod sprite_source;
pub use sprite_source::SpriteSource;
pub mod sprite_size_type;
pub use sprite_size_type::SpriteSizeType;
pub mod sprite_sheet;
pub use sprite_sheet::SpriteSheet;
pub mod sprite_priority;
pub use sprite_priority::SpritePriority;
pub mod sprite_parameters;
pub use sprite_parameters::SpriteParameters;
pub mod sprite_nway_sheet;
pub use sprite_nway_sheet::SpriteNWaySheet;
pub mod sprite_flags;
pub use sprite_flags::SpriteFlags;
pub mod sprite_4way;
pub use sprite_4way::Sprite4Way;
pub mod sprite_16way;
pub use sprite_16way::Sprite16Way;
pub mod sprite;
pub use sprite::Sprite;
pub mod spoil_to_trigger_result;
pub use spoil_to_trigger_result::SpoilToTriggerResult;
pub mod spider_vehicle_graphics_set;
pub use spider_vehicle_graphics_set::SpiderVehicleGraphicsSet;
pub mod spider_torso_graphics_set;
pub use spider_torso_graphics_set::SpiderTorsoGraphicsSet;
pub mod spider_leg_trigger_effect;
pub use spider_leg_trigger_effect::SpiderLegTriggerEffect;
pub mod spider_leg_specification;
pub use spider_leg_specification::SpiderLegSpecification;
pub mod spider_leg_part;
pub use spider_leg_part::SpiderLegPart;
pub mod spider_engine_specification;
pub use spider_engine_specification::SpiderEngineSpecification;
pub mod speech_bubble_style_specification;
pub use speech_bubble_style_specification::SpeechBubbleStyleSpecification;
pub mod spawn_point;
pub use spawn_point::SpawnPoint;
pub mod space_tile_effect_parameters;
pub use space_tile_effect_parameters::SpaceTileEffectParameters;
pub mod space_platforms_modifier;
pub use space_platforms_modifier::SpacePlatformsModifier;
pub mod space_platform_tile_definition;
pub use space_platform_tile_definition::SpacePlatformTileDefinition;
pub mod space_location_id;
pub use space_location_id::SpaceLocationID;
pub mod space_location_asteroid_spawn_definition;
pub use space_location_asteroid_spawn_definition::SpaceLocationAsteroidSpawnDefinition;
pub mod space_dust_effect_properties;
pub use space_dust_effect_properties::SpaceDustEffectProperties;
pub mod space_connection_id;
pub use space_connection_id::SpaceConnectionID;
pub mod space_connection_asteroid_spawn_point;
pub use space_connection_asteroid_spawn_point::SpaceConnectionAsteroidSpawnPoint;
pub mod space_connection_asteroid_spawn_definition;
pub use space_connection_asteroid_spawn_definition::SpaceConnectionAsteroidSpawnDefinition;
pub mod sound_type;
pub use sound_type::SoundType;
pub mod sound_modifier_type;
pub use sound_modifier_type::SoundModifierType;
pub mod sound_modifier;
pub use sound_modifier::SoundModifier;
pub mod sound_definition;
pub use sound_definition::SoundDefinition;
pub mod sound_accent;
pub use sound_accent::SoundAccent;
pub mod sound;
pub use sound::Sound;
pub mod smoke_source;
pub use smoke_source::SmokeSource;
pub mod slider_style_specification;
pub use slider_style_specification::SliderStyleSpecification;
pub mod single_graphic_procession_layer;
pub use single_graphic_procession_layer::SingleGraphicProcessionLayer;
pub mod simulation_definition;
pub use simulation_definition::SimulationDefinition;
pub mod simple_modifier;
pub use simple_modifier::SimpleModifier;
pub mod simple_bounding_box;
pub use simple_bounding_box::SimpleBoundingBox;
pub mod signal_idconnector;
pub use signal_idconnector::SignalIDConnector;
pub mod signal_color_mapping;
pub use signal_color_mapping::SignalColorMapping;
pub mod show_explosion_on_chart_trigger_effect_item;
pub use show_explosion_on_chart_trigger_effect_item::ShowExplosionOnChartTriggerEffectItem;
pub mod shoot_tip_trigger;
pub use shoot_tip_trigger::ShootTipTrigger;
pub mod shift_animation_waypoints;
pub use shift_animation_waypoints::ShiftAnimationWaypoints;
pub mod settings;
pub use settings::Settings;
pub mod set_tile_trigger_effect_item;
pub use set_tile_trigger_effect_item::SetTileTriggerEffectItem;
pub mod set_recipe_tip_trigger;
pub use set_recipe_tip_trigger::SetRecipeTipTrigger;
pub mod set_logistic_request_tip_trigger;
pub use set_logistic_request_tip_trigger::SetLogisticRequestTipTrigger;
pub mod set_filter_tip_trigger;
pub use set_filter_tip_trigger::SetFilterTipTrigger;
pub mod sequence_tip_trigger;
pub use sequence_tip_trigger::SequenceTipTrigger;
pub mod send_to_orbit_mode;
pub use send_to_orbit_mode::SendToOrbitMode;
pub mod send_spidertron_tip_trigger;
pub use send_spidertron_tip_trigger::SendSpidertronTipTrigger;
pub mod send_item_to_orbit_technology_trigger;
pub use send_item_to_orbit_technology_trigger::SendItemToOrbitTechnologyTrigger;
pub mod semi_persistent_world_ambient_sound_definition;
pub use semi_persistent_world_ambient_sound_definition::SemiPersistentWorldAmbientSoundDefinition;
pub mod selection_mode_flags;
pub use selection_mode_flags::SelectionModeFlags;
pub mod selection_mode_data;
pub use selection_mode_data::SelectionModeData;
pub mod segment_specification;
pub use segment_specification::SegmentSpecification;
pub mod segment_engine_specification;
pub use segment_engine_specification::SegmentEngineSpecification;
pub mod scroll_pane_style_specification;
pub use scroll_pane_style_specification::ScrollPaneStyleSpecification;
pub mod scroll_bar_style_specification;
pub use scroll_bar_style_specification::ScrollBarStyleSpecification;
pub mod script_trigger_effect_item;
pub use script_trigger_effect_item::ScriptTriggerEffectItem;
pub mod rotated_sprite_frame;
pub use rotated_sprite_frame::RotatedSpriteFrame;
pub mod rotated_sprite;
pub use rotated_sprite::RotatedSprite;
pub mod rotated_animation_variations;
pub use rotated_animation_variations::RotatedAnimationVariations;
pub mod rotated_animation_8way;
pub use rotated_animation_8way::RotatedAnimation8Way;
pub mod rotated_animation;
pub use rotated_animation::RotatedAnimation;
pub mod rotate_entity_tip_trigger;
pub use rotate_entity_tip_trigger::RotateEntityTipTrigger;
pub mod rolling_stock_rotated_sloped_graphics;
pub use rolling_stock_rotated_sloped_graphics::RollingStockRotatedSlopedGraphics;
pub mod rich_text_setting;
pub use rich_text_setting::RichTextSetting;
pub mod resource_category_id;
pub use resource_category_id::ResourceCategoryID;
pub mod resistance;
pub use resistance::Resistance;
pub mod research_with_science_pack_tip_trigger;
pub use research_with_science_pack_tip_trigger::ResearchWithSciencePackTipTrigger;
pub mod research_technology_tip_trigger;
pub use research_technology_tip_trigger::ResearchTechnologyTipTrigger;
pub mod research_progress_product_prototype;
pub use research_progress_product_prototype::ResearchProgressProductPrototype;
pub mod research_ingredient;
pub use research_ingredient::ResearchIngredient;
pub mod render_layer;
pub use render_layer::RenderLayer;
pub mod recipe_tints;
pub use recipe_tints::RecipeTints;
pub mod recipe_id;
pub use recipe_id::RecipeID;
pub mod recipe_category_id;
pub use recipe_category_id::RecipeCategoryID;
pub mod real_orientation;
pub use real_orientation::RealOrientation;
pub mod ranged_value;
pub use ranged_value::RangedValue;
pub mod range_mode;
pub use range_mode::RangeMode;
pub mod random_range;
pub use random_range::RandomRange;
pub mod rail_support_on_deep_oil_ocean_modifier;
pub use rail_support_on_deep_oil_ocean_modifier::RailSupportOnDeepOilOceanModifier;
pub mod rail_signal_static_sprite_layer;
pub use rail_signal_static_sprite_layer::RailSignalStaticSpriteLayer;
pub mod rail_signal_picture_set;
pub use rail_signal_picture_set::RailSignalPictureSet;
pub mod rail_signal_lights;
pub use rail_signal_lights::RailSignalLights;
pub mod rail_signal_light_definition;
pub use rail_signal_light_definition::RailSignalLightDefinition;
pub mod rail_signal_color_to_frame_index;
pub use rail_signal_color_to_frame_index::RailSignalColorToFrameIndex;
pub mod rail_render_layers;
pub use rail_render_layers::RailRenderLayers;
pub mod rail_planner_allow_elevated_rails_modifier;
pub use rail_planner_allow_elevated_rails_modifier::RailPlannerAllowElevatedRailsModifier;
pub mod rail_piece_layers;
pub use rail_piece_layers::RailPieceLayers;
pub mod rail_picture_set;
pub use rail_picture_set::RailPictureSet;
pub mod rail_fence_picture_set;
pub use rail_fence_picture_set::RailFencePictureSet;
pub mod rail_fence_graphics_set;
pub use rail_fence_graphics_set::RailFenceGraphicsSet;
pub mod rail_fence_direction_set;
pub use rail_fence_direction_set::RailFenceDirectionSet;
pub mod radius_visualisation_specification;
pub use radius_visualisation_specification::RadiusVisualisationSpecification;
pub mod radio_button_style_specification;
pub use radio_button_style_specification::RadioButtonStyleSpecification;
pub mod quality_id;
pub use quality_id::QualityID;
pub mod push_back_trigger_effect_item;
pub use push_back_trigger_effect_item::PushBackTriggerEffectItem;
pub mod pump_connector_graphics_animation;
pub use pump_connector_graphics_animation::PumpConnectorGraphicsAnimation;
pub mod pump_connector_graphics;
pub use pump_connector_graphics::PumpConnectorGraphics;
pub mod puddle_tile_effect_parameters;
pub use puddle_tile_effect_parameters::PuddleTileEffectParameters;
pub mod prototype_strafe_settings;
pub use prototype_strafe_settings::PrototypeStrafeSettings;
pub mod projectile_trigger_delivery;
pub use projectile_trigger_delivery::ProjectileTriggerDelivery;
pub mod projectile_attack_parameters;
pub use projectile_attack_parameters::ProjectileAttackParameters;
pub mod progress_bar_style_specification;
pub use progress_bar_style_specification::ProgressBarStyleSpecification;
pub mod programmable_speaker_note;
pub use programmable_speaker_note::ProgrammableSpeakerNote;
pub mod production_health_effect;
pub use production_health_effect::ProductionHealthEffect;
pub mod product_prototype;
pub use product_prototype::ProductPrototype;
pub mod procession_timeline;
pub use procession_timeline::ProcessionTimeline;
pub mod procession_set;
pub use procession_set::ProcessionSet;
pub mod procession_layer_with_time;
pub use procession_layer_with_time::ProcessionLayerWithTime;
pub mod procession_layer_inheritance_group_id;
pub use procession_layer_inheritance_group_id::ProcessionLayerInheritanceGroupID;
pub mod procession_layer;
pub use procession_layer::ProcessionLayer;
pub mod procession_id;
pub use procession_id::ProcessionID;
pub mod procession_graphic_type;
pub use procession_graphic_type::ProcessionGraphicType;
pub mod procession_graphic_catalogue_item;
pub use procession_graphic_catalogue_item::ProcessionGraphicCatalogueItem;
pub mod procession_graphic_catalogue;
pub use procession_graphic_catalogue::ProcessionGraphicCatalogue;
pub mod procession_graphic;
pub use procession_graphic::ProcessionGraphic;
pub mod procession_audio_usage;
pub use procession_audio_usage::ProcessionAudioUsage;
pub mod procession_audio_type;
pub use procession_audio_type::ProcessionAudioType;
pub mod procession_audio_event_type;
pub use procession_audio_event_type::ProcessionAudioEventType;
pub mod procession_audio_event;
pub use procession_audio_event::ProcessionAudioEvent;
pub mod procession_audio_catalogue_item;
pub use procession_audio_catalogue_item::ProcessionAudioCatalogueItem;
pub mod procession_audio_catalogue;
pub use procession_audio_catalogue::ProcessionAudioCatalogue;
pub mod procession_audio;
pub use procession_audio::ProcessionAudio;
pub mod probability_table_item;
pub use probability_table_item::ProbabilityTableItem;
pub mod probability_table;
pub use probability_table::ProbabilityTable;
pub mod pollution_settings;
pub use pollution_settings::PollutionSettings;
pub mod pod_opacity_procession_layer;
pub use pod_opacity_procession_layer::PodOpacityProcessionLayer;
pub mod pod_movement_procession_layer;
pub use pod_movement_procession_layer::PodMovementProcessionLayer;
pub mod pod_distance_traveled_procession_layer;
pub use pod_distance_traveled_procession_layer::PodDistanceTraveledProcessionLayer;
pub mod pod_animation_procession_layer;
pub use pod_animation_procession_layer::PodAnimationProcessionLayer;
pub mod plumes_specification;
pub use plumes_specification::PlumesSpecification;
pub mod plume_effect;
pub use plume_effect::PlumeEffect;
pub mod player_input_method_filter;
pub use player_input_method_filter::PlayerInputMethodFilter;
pub mod play_sound_trigger_effect_item;
pub use play_sound_trigger_effect_item::PlaySoundTriggerEffectItem;
pub mod planet_prototype_map_gen_settings;
pub use planet_prototype_map_gen_settings::PlanetPrototypeMapGenSettings;
pub mod plan_train_path_tip_trigger;
pub use plan_train_path_tip_trigger::PlanTrainPathTipTrigger;
pub mod place_equipment_tip_trigger;
pub use place_equipment_tip_trigger::PlaceEquipmentTipTrigger;
pub mod pipe_pictures;
pub use pipe_pictures::PipePictures;
pub mod pipe_connection_definition;
pub use pipe_connection_definition::PipeConnectionDefinition;
pub mod persistent_world_ambient_sounds_definition_crossfade;
pub use persistent_world_ambient_sounds_definition_crossfade::PersistentWorldAmbientSoundsDefinitionCrossfade;
pub mod persistent_world_ambient_sounds_definition;
pub use persistent_world_ambient_sounds_definition::PersistentWorldAmbientSoundsDefinition;
pub mod persistent_world_ambient_sound_definition;
pub use persistent_world_ambient_sound_definition::PersistentWorldAmbientSoundDefinition;
pub mod perceived_performance;
pub use perceived_performance::PerceivedPerformance;
pub mod path_finder_settings;
pub use path_finder_settings::PathFinderSettings;
pub mod paste_entity_settings_tip_trigger;
pub use paste_entity_settings_tip_trigger::PasteEntitySettingsTipTrigger;
pub mod particle_id;
pub use particle_id::ParticleID;
pub mod oriented_cliff_prototype;
pub use oriented_cliff_prototype::OrientedCliffPrototype;
pub mod order;
pub use order::Order;
pub mod or_tip_trigger;
pub use or_tip_trigger::OrTipTrigger;
pub mod offshore_pump_graphics_set;
pub use offshore_pump_graphics_set::OffshorePumpGraphicsSet;
pub mod nothing_modifier;
pub use nothing_modifier::NothingModifier;
pub mod noise_function;
pub use noise_function::NoiseFunction;
pub mod noise_expression;
pub use noise_expression::NoiseExpression;
pub mod nested_trigger_effect_item;
pub use nested_trigger_effect_item::NestedTriggerEffectItem;
pub mod neighbour_connectable_connection_definition;
pub use neighbour_connectable_connection_definition::NeighbourConnectableConnectionDefinition;
pub mod neighbour_connectable_connection_category;
pub use neighbour_connectable_connection_category::NeighbourConnectableConnectionCategory;
pub mod neighbour_connectable;
pub use neighbour_connectable::NeighbourConnectable;
pub mod mouse_cursor_id;
pub use mouse_cursor_id::MouseCursorID;
pub mod module_transfer_tip_trigger;
pub use module_transfer_tip_trigger::ModuleTransferTipTrigger;
pub mod module_tint;
pub use module_tint::ModuleTint;
pub mod module_category_id;
pub use module_category_id::ModuleCategoryID;
pub mod mods;
pub use mods::Mods;
pub mod modifier;
pub use modifier::Modifier;
pub mod mod_setting;
pub use mod_setting::ModSetting;
pub mod mirroring;
pub use mirroring::Mirroring;
pub mod mining_with_fluid_modifier;
pub use mining_with_fluid_modifier::MiningWithFluidModifier;
pub mod mining_drill_productivity_bonus_modifier;
pub use mining_drill_productivity_bonus_modifier::MiningDrillProductivityBonusModifier;
pub mod mining_drill_graphics_set;
pub use mining_drill_graphics_set::MiningDrillGraphicsSet;
pub mod minimap_style_specification;
pub use minimap_style_specification::MinimapStyleSpecification;
pub mod mine_item_by_robot_tip_trigger;
pub use mine_item_by_robot_tip_trigger::MineItemByRobotTipTrigger;
pub mod mine_entity_technology_trigger;
pub use mine_entity_technology_trigger::MineEntityTechnologyTrigger;
pub mod minable_properties;
pub use minable_properties::MinableProperties;
pub mod maximum_following_robots_count_modifier;
pub use maximum_following_robots_count_modifier::MaximumFollowingRobotsCountModifier;
pub mod max_successful_attempts_per_tick_per_construction_queue_modifier;
pub use max_successful_attempts_per_tick_per_construction_queue_modifier::MaxSuccessfulAttemptsPerTickPerConstructionQueueModifier;
pub mod max_failed_attempts_per_tick_per_construction_queue_modifier;
pub use max_failed_attempts_per_tick_per_construction_queue_modifier::MaxFailedAttemptsPerTickPerConstructionQueueModifier;
pub mod math_expression;
pub use math_expression::MathExpression;
pub mod material_texture_parameters;
pub use material_texture_parameters::MaterialTextureParameters;
pub mod material_amount_type;
pub use material_amount_type::MaterialAmountType;
pub mod map_tick;
pub use map_tick::MapTick;
pub mod map_position;
pub use map_position::MapPosition;
pub mod map_gen_size;
pub use map_gen_size::MapGenSize;
pub mod map_gen_settings;
pub use map_gen_settings::MapGenSettings;
pub mod map_gen_preset_pollution_settings;
pub use map_gen_preset_pollution_settings::MapGenPresetPollutionSettings;
pub mod map_gen_preset_enemy_expansion_settings;
pub use map_gen_preset_enemy_expansion_settings::MapGenPresetEnemyExpansionSettings;
pub mod map_gen_preset_enemy_evolution_settings;
pub use map_gen_preset_enemy_evolution_settings::MapGenPresetEnemyEvolutionSettings;
pub mod map_gen_preset_difficulty_settings;
pub use map_gen_preset_difficulty_settings::MapGenPresetDifficultySettings;
pub mod map_gen_preset_asteroid_settings;
pub use map_gen_preset_asteroid_settings::MapGenPresetAsteroidSettings;
pub mod map_gen_preset;
pub use map_gen_preset::MapGenPreset;
pub mod manual_wire_drag_tip_trigger;
pub use manual_wire_drag_tip_trigger::ManualWireDragTipTrigger;
pub mod manual_transfer_tip_trigger;
pub use manual_transfer_tip_trigger::ManualTransferTipTrigger;
pub mod main_sound;
pub use main_sound::MainSound;
pub mod low_power_tip_trigger;
pub use low_power_tip_trigger::LowPowerTipTrigger;
pub mod loot_item;
pub use loot_item::LootItem;
pub mod logistic_filter_index;
pub use logistic_filter_index::LogisticFilterIndex;
pub mod localised_string;
pub use localised_string::LocalisedString;
pub mod loader_structure;
pub use loader_structure::LoaderStructure;
pub mod list_box_style_specification;
pub use list_box_style_specification::ListBoxStyleSpecification;
pub mod linked_game_control;
pub use linked_game_control::LinkedGameControl;
pub mod line_trigger_item;
pub use line_trigger_item::LineTriggerItem;
pub mod line_style_specification;
pub use line_style_specification::LineStyleSpecification;
pub mod limit_chest_tip_trigger;
pub use limit_chest_tip_trigger::LimitChestTipTrigger;
pub mod lightning_rule_base;
pub use lightning_rule_base::LightningRuleBase;
pub mod lightning_properties;
pub use lightning_properties::LightningProperties;
pub mod lightning_priority_rule;
pub use lightning_priority_rule::LightningPriorityRule;
pub mod lightning_graphics_set;
pub use lightning_graphics_set::LightningGraphicsSet;
pub mod light_properties;
pub use light_properties::LightProperties;
pub mod light_flickering_definition;
pub use light_flickering_definition::LightFlickeringDefinition;
pub mod light_definition;
pub use light_definition::LightDefinition;
pub mod layered_sprite_variations;
pub use layered_sprite_variations::LayeredSpriteVariations;
pub mod layered_sprite;
pub use layered_sprite::LayeredSprite;
pub mod layered_sound;
pub use layered_sound::LayeredSound;
pub mod laboratory_speed_modifier;
pub use laboratory_speed_modifier::LaboratorySpeedModifier;
pub mod laboratory_productivity_modifier;
pub use laboratory_productivity_modifier::LaboratoryProductivityModifier;
pub mod label_style_specification;
pub use label_style_specification::LabelStyleSpecification;
pub mod kill_tip_trigger;
pub use kill_tip_trigger::KillTipTrigger;
pub mod item_to_place;
pub use item_to_place::ItemToPlace;
pub mod item_sub_group_id;
pub use item_sub_group_id::ItemSubGroupID;
pub mod item_stack_index;
pub use item_stack_index::ItemStackIndex;
pub mod item_prototype_flags;
pub use item_prototype_flags::ItemPrototypeFlags;
pub mod item_product_prototype;
pub use item_product_prototype::ItemProductPrototype;
pub mod item_ingredient_prototype;
pub use item_ingredient_prototype::ItemIngredientPrototype;
pub mod item_idfilter;
pub use item_idfilter::ItemIDFilter;
pub mod item_id;
pub use item_id::ItemID;
pub mod item_group_id;
pub use item_group_id::ItemGroupID;
pub mod item_count_type;
pub use item_count_type::ItemCountType;
pub mod invoke_tile_effect_trigger_effect_item;
pub use invoke_tile_effect_trigger_effect_item::InvokeTileEffectTriggerEffectItem;
pub mod interruptible_sound;
pub use interruptible_sound::InterruptibleSound;
pub mod instant_trigger_delivery;
pub use instant_trigger_delivery::InstantTriggerDelivery;
pub mod inserter_stack_size_bonus_modifier;
pub use inserter_stack_size_bonus_modifier::InserterStackSizeBonusModifier;
pub mod insert_item_trigger_effect_item;
pub use insert_item_trigger_effect_item::InsertItemTriggerEffectItem;
pub mod ingredient_prototype;
pub use ingredient_prototype::IngredientPrototype;
pub mod image_style_specification;
pub use image_style_specification::ImageStyleSpecification;
pub mod icon_sequence_positioning;
pub use icon_sequence_positioning::IconSequencePositioning;
pub mod icon_draw_specification;
pub use icon_draw_specification::IconDrawSpecification;
pub mod icon_data;
pub use icon_data::IconData;
pub mod horizontal_scroll_bar_style_specification;
pub use horizontal_scroll_bar_style_specification::HorizontalScrollBarStyleSpecification;
pub mod horizontal_flow_style_specification;
pub use horizontal_flow_style_specification::HorizontalFlowStyleSpecification;
pub mod horizontal_align;
pub use horizontal_align::HorizontalAlign;
pub mod heat_energy_source;
pub use heat_energy_source::HeatEnergySource;
pub mod heat_connection;
pub use heat_connection::HeatConnection;
pub mod heat_buffer;
pub use heat_buffer::HeatBuffer;
pub mod gun_speed_modifier;
pub use gun_speed_modifier::GunSpeedModifier;
pub mod group_attack_tip_trigger;
pub use group_attack_tip_trigger::GroupAttackTipTrigger;
pub mod graph_style_specification;
pub use graph_style_specification::GraphStyleSpecification;
pub mod glow_style_specification;
pub use glow_style_specification::GlowStyleSpecification;
pub mod global_tint_effect_properties;
pub use global_tint_effect_properties::GlobalTintEffectProperties;
pub mod global_recipe_tints;
pub use global_recipe_tints::GlobalRecipeTints;
pub mod give_item_modifier;
pub use give_item_modifier::GiveItemModifier;
pub mod giga_cargo_hatch_definition;
pub use giga_cargo_hatch_definition::GigaCargoHatchDefinition;
pub mod ghost_shimmer_overlay_data;
pub use ghost_shimmer_overlay_data::GhostShimmerOverlayData;
pub mod ghost_shimmer_distortion_data;
pub use ghost_shimmer_distortion_data::GhostShimmerDistortionData;
pub mod ghost_shimmer_config;
pub use ghost_shimmer_config::GhostShimmerConfig;
pub mod generating_power_tip_trigger;
pub use generating_power_tip_trigger::GeneratingPowerTipTrigger;
pub mod gate_over_rail_build_tip_trigger;
pub use gate_over_rail_build_tip_trigger::GateOverRailBuildTipTrigger;
pub mod game_view_settings;
pub use game_view_settings::GameViewSettings;
pub mod game_controller_vibration_data;
pub use game_controller_vibration_data::GameControllerVibrationData;
pub mod fusion_reactor_graphics_set;
pub use fusion_reactor_graphics_set::FusionReactorGraphicsSet;
pub mod fusion_reactor_connection_graphics;
pub use fusion_reactor_connection_graphics::FusionReactorConnectionGraphics;
pub mod fusion_generator_graphics_set;
pub use fusion_generator_graphics_set::FusionGeneratorGraphicsSet;
pub mod fusion_generator_fluid_input_graphics;
pub use fusion_generator_fluid_input_graphics::FusionGeneratorFluidInputGraphics;
pub mod fusion_generator_direction_graphics_set;
pub use fusion_generator_direction_graphics_set::FusionGeneratorDirectionGraphicsSet;
pub mod fuel_category_id;
pub use fuel_category_id::FuelCategoryID;
pub mod frequency_size_richness;
pub use frequency_size_richness::FrequencySizeRichness;
pub mod frame_style_specification;
pub use frame_style_specification::FrameStyleSpecification;
pub mod force_condition;
pub use force_condition::ForceCondition;
pub mod footstep_trigger_effect_list;
pub use footstep_trigger_effect_list::FootstepTriggerEffectList;
pub mod footstep_trigger_effect_item;
pub use footstep_trigger_effect_item::FootstepTriggerEffectItem;
pub mod footprint_particle;
pub use footprint_particle::FootprintParticle;
pub mod follower_robot_lifetime_modifier;
pub use follower_robot_lifetime_modifier::FollowerRobotLifetimeModifier;
pub mod fog_mask_shape_definition;
pub use fog_mask_shape_definition::FogMaskShapeDefinition;
pub mod fog_effect_properties;
pub use fog_effect_properties::FogEffectProperties;
pub mod fluid_product_prototype;
pub use fluid_product_prototype::FluidProductPrototype;
pub mod fluid_ingredient_prototype;
pub use fluid_ingredient_prototype::FluidIngredientPrototype;
pub mod fluid_id;
pub use fluid_id::FluidID;
pub mod fluid_energy_source;
pub use fluid_energy_source::FluidEnergySource;
pub mod fluid_box_linked_connection_id;
pub use fluid_box_linked_connection_id::FluidBoxLinkedConnectionID;
pub mod fluid_box;
pub use fluid_box::FluidBox;
pub mod fluid_amount;
pub use fluid_amount::FluidAmount;
pub mod flow_style_specification;
pub use flow_style_specification::FlowStyleSpecification;
pub mod flip_entity_tip_trigger;
pub use flip_entity_tip_trigger::FlipEntityTipTrigger;
pub mod file_name;
pub use file_name::FileName;
pub mod feature_flags;
pub use feature_flags::FeatureFlags;
pub mod fast_replace_tip_trigger;
pub use fast_replace_tip_trigger::FastReplaceTipTrigger;
pub mod fast_belt_bend_tip_trigger;
pub use fast_belt_bend_tip_trigger::FastBeltBendTipTrigger;
pub mod fades;
pub use fades::Fades;
pub mod fade;
pub use fade::Fade;
pub mod explosion_definition;
pub use explosion_definition::ExplosionDefinition;
pub mod equipment_shape;
pub use equipment_shape::EquipmentShape;
pub mod equipment_id;
pub use equipment_id::EquipmentID;
pub mod equipment_grid_id;
pub use equipment_grid_id::EquipmentGridID;
pub mod equipment_category_id;
pub use equipment_category_id::EquipmentCategoryID;
pub mod entity_transfer_tip_trigger;
pub use entity_transfer_tip_trigger::EntityTransferTipTrigger;
pub mod entity_status;
pub use entity_status::EntityStatus;
pub mod entity_prototype_flags;
pub use entity_prototype_flags::EntityPrototypeFlags;
pub mod entity_idfilter;
pub use entity_idfilter::EntityIDFilter;
pub mod entity_id;
pub use entity_id::EntityID;
pub mod entity_build_animation_piece;
pub use entity_build_animation_piece::EntityBuildAnimationPiece;
pub mod enter_vehicle_tip_trigger;
pub use enter_vehicle_tip_trigger::EnterVehicleTipTrigger;
pub mod energy_source;
pub use energy_source::EnergySource;
pub mod energy;
pub use energy::Energy;
pub mod enemy_spawner_graphics_set;
pub use enemy_spawner_graphics_set::EnemySpawnerGraphicsSet;
pub mod enemy_spawner_absorption;
pub use enemy_spawner_absorption::EnemySpawnerAbsorption;
pub mod enemy_expansion_settings;
pub use enemy_expansion_settings::EnemyExpansionSettings;
pub mod enemy_evolution_settings;
pub use enemy_evolution_settings::EnemyEvolutionSettings;
pub mod empty_widget_style_specification;
pub use empty_widget_style_specification::EmptyWidgetStyleSpecification;
pub mod element_image_set_layer;
pub use element_image_set_layer::ElementImageSetLayer;
pub mod element_image_set;
pub use element_image_set::ElementImageSet;
pub mod electric_usage_priority;
pub use electric_usage_priority::ElectricUsagePriority;
pub mod electric_energy_source;
pub use electric_energy_source::ElectricEnergySource;
pub mod effect_variation;
pub use effect_variation::EffectVariation;
pub mod effect_value;
pub use effect_value::EffectValue;
pub mod effect_type_limitation;
pub use effect_type_limitation::EffectTypeLimitation;
pub mod effect_texture;
pub use effect_texture::EffectTexture;
pub mod effect_relative_to;
pub use effect_relative_to::EffectRelativeTo;
pub mod effect_receiver;
pub use effect_receiver::EffectReceiver;
pub mod effect;
pub use effect::Effect;
pub mod drop_item_tip_trigger;
pub use drop_item_tip_trigger::DropItemTipTrigger;
pub mod drop_down_style_specification;
pub use drop_down_style_specification::DropDownStyleSpecification;
pub mod double_slider_style_specification;
pub use double_slider_style_specification::DoubleSliderStyleSpecification;
pub mod direction_string;
pub use direction_string::DirectionString;
pub mod direction;
pub use direction::Direction;
pub mod direct_trigger_item;
pub use direct_trigger_item::DirectTriggerItem;
pub mod difficulty_settings;
pub use difficulty_settings::DifficultySettings;
pub mod destroy_decoratives_trigger_effect_item;
pub use destroy_decoratives_trigger_effect_item::DestroyDecorativesTriggerEffectItem;
pub mod destroy_cliffs_trigger_effect_item;
pub use destroy_cliffs_trigger_effect_item::DestroyCliffsTriggerEffectItem;
pub mod destroy_cliffs_capsule_action;
pub use destroy_cliffs_capsule_action::DestroyCliffsCapsuleAction;
pub mod dependencies_met_tip_trigger;
pub use dependencies_met_tip_trigger::DependenciesMetTipTrigger;
pub mod delayed_trigger_delivery;
pub use delayed_trigger_delivery::DelayedTriggerDelivery;
pub mod decorative_id;
pub use decorative_id::DecorativeID;
pub mod deconstruction_time_to_live_modifier;
pub use deconstruction_time_to_live_modifier::DeconstructionTimeToLiveModifier;
pub mod daytime_color_lookup_table;
pub use daytime_color_lookup_table::DaytimeColorLookupTable;
pub mod data_extend_method;
pub use data_extend_method::DataExtendMethod;
pub mod data;
pub use data::Data;
pub mod damage_type_id;
pub use damage_type_id::DamageTypeID;
pub mod damage_type_filters;
pub use damage_type_filters::DamageTypeFilters;
pub mod damage_trigger_effect_item;
pub use damage_trigger_effect_item::DamageTriggerEffectItem;
pub mod damage_parameters;
pub use damage_parameters::DamageParameters;
pub mod cyclic_sound;
pub use cyclic_sound::CyclicSound;
pub mod cursor_box_type;
pub use cursor_box_type::CursorBoxType;
pub mod create_trivial_smoke_effect_item;
pub use create_trivial_smoke_effect_item::CreateTrivialSmokeEffectItem;
pub mod create_sticker_trigger_effect_item;
pub use create_sticker_trigger_effect_item::CreateStickerTriggerEffectItem;
pub mod create_space_platform_technology_trigger;
pub use create_space_platform_technology_trigger::CreateSpacePlatformTechnologyTrigger;
pub mod create_smoke_trigger_effect_item;
pub use create_smoke_trigger_effect_item::CreateSmokeTriggerEffectItem;
pub mod create_particle_trigger_effect_item;
pub use create_particle_trigger_effect_item::CreateParticleTriggerEffectItem;
pub mod create_ghost_on_entity_death_modifier;
pub use create_ghost_on_entity_death_modifier::CreateGhostOnEntityDeathModifier;
pub mod create_fire_trigger_effect_item;
pub use create_fire_trigger_effect_item::CreateFireTriggerEffectItem;
pub mod create_explosion_trigger_effect_item;
pub use create_explosion_trigger_effect_item::CreateExplosionTriggerEffectItem;
pub mod create_entity_trigger_effect_item;
pub use create_entity_trigger_effect_item::CreateEntityTriggerEffectItem;
pub mod create_decoratives_trigger_effect_item;
pub use create_decoratives_trigger_effect_item::CreateDecorativesTriggerEffectItem;
pub mod create_asteroid_chunk_effect_item;
pub use create_asteroid_chunk_effect_item::CreateAsteroidChunkEffectItem;
pub mod crater_placement_definition;
pub use crater_placement_definition::CraterPlacementDefinition;
pub mod crane_part_dying_effect;
pub use crane_part_dying_effect::CranePartDyingEffect;
pub mod crane_part;
pub use crane_part::CranePart;
pub mod crafting_machine_graphics_set;
pub use crafting_machine_graphics_set::CraftingMachineGraphicsSet;
pub mod craft_item_tip_trigger;
pub use craft_item_tip_trigger::CraftItemTipTrigger;
pub mod craft_item_technology_trigger;
pub use craft_item_technology_trigger::CraftItemTechnologyTrigger;
pub mod craft_fluid_technology_trigger;
pub use craft_fluid_technology_trigger::CraftFluidTechnologyTrigger;
pub mod cover_graphic_procession_layer;
pub use cover_graphic_procession_layer::CoverGraphicProcessionLayer;
pub mod cover_graphic_effect_data;
pub use cover_graphic_effect_data::CoverGraphicEffectData;
pub mod count_based_tip_trigger;
pub use count_based_tip_trigger::CountBasedTipTrigger;
pub mod control_point;
pub use control_point::ControlPoint;
pub mod consuming_type;
pub use consuming_type::ConsumingType;
pub mod connectable_entity_graphics;
pub use connectable_entity_graphics::ConnectableEntityGraphics;
pub mod comparator_string;
pub use comparator_string::ComparatorString;
pub mod column_width_item;
pub use column_width_item::ColumnWidthItem;
pub mod column_width;
pub use column_width::ColumnWidth;
pub mod column_alignment;
pub use column_alignment::ColumnAlignment;
pub mod color_lookup_table;
pub use color_lookup_table::ColorLookupTable;
pub mod color_hint_specification;
pub use color_hint_specification::ColorHintSpecification;
pub mod color;
pub use color::Color;
pub mod collision_mask_connector;
pub use collision_mask_connector::CollisionMaskConnector;
pub mod collision_layer_id;
pub use collision_layer_id::CollisionLayerID;
pub mod cluster_trigger_item;
pub use cluster_trigger_item::ClusterTriggerItem;
pub mod clouds_texture_coordinate_transformation;
pub use clouds_texture_coordinate_transformation::CloudsTextureCoordinateTransformation;
pub mod clouds_effect_properties;
pub use clouds_effect_properties::CloudsEffectProperties;
pub mod cloud_effect_style;
pub use cloud_effect_style::CloudEffectStyle;
pub mod cliff_placement_settings;
pub use cliff_placement_settings::CliffPlacementSettings;
pub mod cliff_deconstruction_enabled_modifier;
pub use cliff_deconstruction_enabled_modifier::CliffDeconstructionEnabledModifier;
pub mod clear_cursor_tip_trigger;
pub use clear_cursor_tip_trigger::ClearCursorTipTrigger;
pub mod circular_projectile_creation_specification;
pub use circular_projectile_creation_specification::CircularProjectileCreationSpecification;
pub mod circular_particle_creation_specification;
pub use circular_particle_creation_specification::CircularParticleCreationSpecification;
pub mod circuit_network_modifier;
pub use circuit_network_modifier::CircuitNetworkModifier;
pub mod circuit_connector_sprites;
pub use circuit_connector_sprites::CircuitConnectorSprites;
pub mod circuit_connector_secondary_draw_order;
pub use circuit_connector_secondary_draw_order::CircuitConnectorSecondaryDrawOrder;
pub mod circuit_connector_layer;
pub use circuit_connector_layer::CircuitConnectorLayer;
pub mod circuit_connector_definition;
pub use circuit_connector_definition::CircuitConnectorDefinition;
pub mod check_box_style_specification;
pub use check_box_style_specification::CheckBoxStyleSpecification;
pub mod chart_utility_constants;
pub use chart_utility_constants::ChartUtilityConstants;
pub mod chargable_graphics;
pub use chargable_graphics::ChargableGraphics;
pub mod character_running_speed_modifier;
pub use character_running_speed_modifier::CharacterRunningSpeedModifier;
pub mod character_resource_reach_distance_modifier;
pub use character_resource_reach_distance_modifier::CharacterResourceReachDistanceModifier;
pub mod character_reach_distance_modifier;
pub use character_reach_distance_modifier::CharacterReachDistanceModifier;
pub mod character_mining_speed_modifier;
pub use character_mining_speed_modifier::CharacterMiningSpeedModifier;
pub mod character_loot_pickup_distance_modifier;
pub use character_loot_pickup_distance_modifier::CharacterLootPickupDistanceModifier;
pub mod character_logistic_trash_slots_modifier;
pub use character_logistic_trash_slots_modifier::CharacterLogisticTrashSlotsModifier;
pub mod character_logistic_requests_modifier;
pub use character_logistic_requests_modifier::CharacterLogisticRequestsModifier;
pub mod character_item_pickup_distance_modifier;
pub use character_item_pickup_distance_modifier::CharacterItemPickupDistanceModifier;
pub mod character_item_drop_distance_modifier;
pub use character_item_drop_distance_modifier::CharacterItemDropDistanceModifier;
pub mod character_inventory_slots_bonus_modifier;
pub use character_inventory_slots_bonus_modifier::CharacterInventorySlotsBonusModifier;
pub mod character_health_bonus_modifier;
pub use character_health_bonus_modifier::CharacterHealthBonusModifier;
pub mod character_crafting_speed_modifier;
pub use character_crafting_speed_modifier::CharacterCraftingSpeedModifier;
pub mod character_build_distance_modifier;
pub use character_build_distance_modifier::CharacterBuildDistanceModifier;
pub mod character_armor_animation;
pub use character_armor_animation::CharacterArmorAnimation;
pub mod change_surface_tip_trigger;
pub use change_surface_tip_trigger::ChangeSurfaceTipTrigger;
pub mod change_recipe_productivity_modifier;
pub use change_recipe_productivity_modifier::ChangeRecipeProductivityModifier;
pub mod chain_trigger_delivery;
pub use chain_trigger_delivery::ChainTriggerDelivery;
pub mod cargo_station_parameters;
pub use cargo_station_parameters::CargoStationParameters;
pub mod cargo_landing_pad_limit_modifier;
pub use cargo_landing_pad_limit_modifier::CargoLandingPadLimitModifier;
pub mod cargo_hatch_definition;
pub use cargo_hatch_definition::CargoHatchDefinition;
pub mod cargo_bay_connections;
pub use cargo_bay_connections::CargoBayConnections;
pub mod cargo_bay_connectable_graphics_set;
pub use cargo_bay_connectable_graphics_set::CargoBayConnectableGraphicsSet;
pub mod capture_spawner_technology_trigger;
pub use capture_spawner_technology_trigger::CaptureSpawnerTechnologyTrigger;
pub mod capsule_action;
pub use capsule_action::CapsuleAction;
pub mod camera_style_specification;
pub use camera_style_specification::CameraStyleSpecification;
pub mod camera_effect_trigger_effect_item;
pub use camera_effect_trigger_effect_item::CameraEffectTriggerEffectItem;
pub mod button_style_specification;
pub use button_style_specification::ButtonStyleSpecification;
pub mod burner_usage_id;
pub use burner_usage_id::BurnerUsageID;
pub mod burner_energy_source;
pub use burner_energy_source::BurnerEnergySource;
pub mod bulk_inserter_capacity_bonus_modifier;
pub use bulk_inserter_capacity_bonus_modifier::BulkInserterCapacityBonusModifier;
pub mod build_mode;
pub use build_mode::BuildMode;
pub mod build_entity_tip_trigger;
pub use build_entity_tip_trigger::BuildEntityTipTrigger;
pub mod build_entity_technology_trigger;
pub use build_entity_technology_trigger::BuildEntityTechnologyTrigger;
pub mod build_entity_by_robot_tip_trigger;
pub use build_entity_by_robot_tip_trigger::BuildEntityByRobotTipTrigger;
pub mod box_specification;
pub use box_specification::BoxSpecification;
pub mod bounding_box;
pub use bounding_box::BoundingBox;
pub mod border_image_set;
pub use border_image_set::BorderImageSet;
pub mod bool_modifier;
pub use bool_modifier::BoolModifier;
pub mod boiler_pictures;
pub use boiler_pictures::BoilerPictures;
pub mod blend_mode;
pub use blend_mode::BlendMode;
pub mod belt_traverse_tip_trigger;
pub use belt_traverse_tip_trigger::BeltTraverseTipTrigger;
pub mod belt_stack_size_bonus_modifier;
pub use belt_stack_size_bonus_modifier::BeltStackSizeBonusModifier;
pub mod belt_reader_layer;
pub use belt_reader_layer::BeltReaderLayer;
pub mod beam_trigger_delivery;
pub use beam_trigger_delivery::BeamTriggerDelivery;
pub mod beam_graphics_set;
pub use beam_graphics_set::BeamGraphicsSet;
pub mod beam_attack_parameters;
pub use beam_attack_parameters::BeamAttackParameters;
pub mod beam_animation_set;
pub use beam_animation_set::BeamAnimationSet;
pub mod beacon_module_visualizations;
pub use beacon_module_visualizations::BeaconModuleVisualizations;
pub mod beacon_module_visualization;
pub use beacon_module_visualization::BeaconModuleVisualization;
pub mod beacon_graphics_set;
pub use beacon_graphics_set::BeaconGraphicsSet;
pub mod beacon_distribution_modifier;
pub use beacon_distribution_modifier::BeaconDistributionModifier;
pub mod base_style_specification;
pub use base_style_specification::BaseStyleSpecification;
pub mod base_modifier;
pub use base_modifier::BaseModifier;
pub mod base_energy_source;
pub use base_energy_source::BaseEnergySource;
pub mod base_attack_parameters;
pub use base_attack_parameters::BaseAttackParameters;
pub mod autoplace_specification;
pub use autoplace_specification::AutoplaceSpecification;
pub mod autoplace_settings;
pub use autoplace_settings::AutoplaceSettings;
pub mod autoplace_control_id;
pub use autoplace_control_id::AutoplaceControlID;
pub mod attenuation_type;
pub use attenuation_type::AttenuationType;
pub mod attenuation;
pub use attenuation::Attenuation;
pub mod attack_reaction_item;
pub use attack_reaction_item::AttackReactionItem;
pub mod attack_parameters;
pub use attack_parameters::AttackParameters;
pub mod asteroid_variation;
pub use asteroid_variation::AsteroidVariation;
pub mod asteroid_spawn_point;
pub use asteroid_spawn_point::AsteroidSpawnPoint;
pub mod asteroid_settings;
pub use asteroid_settings::AsteroidSettings;
pub mod asteroid_graphics_set;
pub use asteroid_graphics_set::AsteroidGraphicsSet;
pub mod asteroid_chunk_id;
pub use asteroid_chunk_id::AsteroidChunkID;
pub mod artillery_trigger_delivery;
pub use artillery_trigger_delivery::ArtilleryTriggerDelivery;
pub mod artillery_remote_capsule_action;
pub use artillery_remote_capsule_action::ArtilleryRemoteCapsuleAction;
pub mod artillery_range_modifier;
pub use artillery_range_modifier::ArtilleryRangeModifier;
pub mod area_trigger_item;
pub use area_trigger_item::AreaTriggerItem;
pub mod apply_starter_pack_tip_trigger;
pub use apply_starter_pack_tip_trigger::ApplyStarterPackTipTrigger;
pub mod animation_variations;
pub use animation_variations::AnimationVariations;
pub mod animation_sheet;
pub use animation_sheet::AnimationSheet;
pub mod animation_run_mode;
pub use animation_run_mode::AnimationRunMode;
pub mod animation_parameters;
pub use animation_parameters::AnimationParameters;
pub mod animation_frame_sequence;
pub use animation_frame_sequence::AnimationFrameSequence;
pub mod animation_element;
pub use animation_element::AnimationElement;
pub mod animation_4way;
pub use animation_4way::Animation4Way;
pub mod animation;
pub use animation::Animation;
pub mod animated_vector;
pub use animated_vector::AnimatedVector;
pub mod and_tip_trigger;
pub use and_tip_trigger::AndTipTrigger;
pub mod ammo_type;
pub use ammo_type::AmmoType;
pub mod ammo_source_type;
pub use ammo_source_type::AmmoSourceType;
pub mod ammo_damage_modifier;
pub use ammo_damage_modifier::AmmoDamageModifier;
pub mod ammo_category_id;
pub use ammo_category_id::AmmoCategoryID;
pub mod ambient_sound_type;
pub use ambient_sound_type::AmbientSoundType;
pub mod alternative_build_tip_trigger;
pub use alternative_build_tip_trigger::AlternativeBuildTipTrigger;
pub mod airborne_pollutant_id;
pub use airborne_pollutant_id::AirbornePollutantID;
pub mod agricultural_crane_speed_grappler;
pub use agricultural_crane_speed_grappler::AgriculturalCraneSpeedGrappler;
pub mod agricultural_crane_speed_arm;
pub use agricultural_crane_speed_arm::AgriculturalCraneSpeedArm;
pub mod agricultural_crane_speed;
pub use agricultural_crane_speed::AgriculturalCraneSpeed;
pub mod agricultural_crane_properties;
pub use agricultural_crane_properties::AgriculturalCraneProperties;
pub mod aggregation_specification;
pub use aggregation_specification::AggregationSpecification;
pub mod advanced_volume_control;
pub use advanced_volume_control::AdvancedVolumeControl;
pub mod activity_matching_modifiers;
pub use activity_matching_modifiers::ActivityMatchingModifiers;
pub mod activity_bar_style_specification;
pub use activity_bar_style_specification::ActivityBarStyleSpecification;
pub mod active_trigger_id;
pub use active_trigger_id::ActiveTriggerID;
pub mod activate_paste_tip_trigger;
pub use activate_paste_tip_trigger::ActivatePasteTipTrigger;
pub mod activate_impact_trigger_effect_item;
pub use activate_impact_trigger_effect_item::ActivateImpactTriggerEffectItem;
pub mod activate_equipment_capsule_action;
pub use activate_equipment_capsule_action::ActivateEquipmentCapsuleAction;
pub enum Types {
    ActivateEquipmentCapsuleAction(Box<ActivateEquipmentCapsuleAction>),
    ActivateImpactTriggerEffectItem(Box<ActivateImpactTriggerEffectItem>),
    ActivatePasteTipTrigger(Box<ActivatePasteTipTrigger>),
    ActiveTriggerID(Box<ActiveTriggerID>),
    ActivityBarStyleSpecification(Box<ActivityBarStyleSpecification>),
    ActivityMatchingModifiers(Box<ActivityMatchingModifiers>),
    AdvancedVolumeControl(Box<AdvancedVolumeControl>),
    AggregationSpecification(Box<AggregationSpecification>),
    AgriculturalCraneProperties(Box<AgriculturalCraneProperties>),
    AgriculturalCraneSpeed(Box<AgriculturalCraneSpeed>),
    AgriculturalCraneSpeedArm(Box<AgriculturalCraneSpeedArm>),
    AgriculturalCraneSpeedGrappler(Box<AgriculturalCraneSpeedGrappler>),
    AirbornePollutantID(Box<AirbornePollutantID>),
    AlternativeBuildTipTrigger(Box<AlternativeBuildTipTrigger>),
    AmbientSoundType(Box<AmbientSoundType>),
    AmmoCategoryID(Box<AmmoCategoryID>),
    AmmoDamageModifier(Box<AmmoDamageModifier>),
    AmmoSourceType(Box<AmmoSourceType>),
    AmmoType(Box<AmmoType>),
    AndTipTrigger(Box<AndTipTrigger>),
    AnimatedVector(Box<AnimatedVector>),
    Animation(Box<Animation>),
    Animation4Way(Box<Animation4Way>),
    AnimationElement(Box<AnimationElement>),
    AnimationFrameSequence(Box<AnimationFrameSequence>),
    AnimationParameters(Box<AnimationParameters>),
    AnimationRunMode(Box<AnimationRunMode>),
    AnimationSheet(Box<AnimationSheet>),
    AnimationVariations(Box<AnimationVariations>),
    ApplyStarterPackTipTrigger(Box<ApplyStarterPackTipTrigger>),
    AreaTriggerItem(Box<AreaTriggerItem>),
    ArtilleryRangeModifier(Box<ArtilleryRangeModifier>),
    ArtilleryRemoteCapsuleAction(Box<ArtilleryRemoteCapsuleAction>),
    ArtilleryTriggerDelivery(Box<ArtilleryTriggerDelivery>),
    AsteroidChunkID(Box<AsteroidChunkID>),
    AsteroidGraphicsSet(Box<AsteroidGraphicsSet>),
    AsteroidSettings(Box<AsteroidSettings>),
    AsteroidSpawnPoint(Box<AsteroidSpawnPoint>),
    AsteroidVariation(Box<AsteroidVariation>),
    AttackParameters(Box<AttackParameters>),
    AttackReactionItem(Box<AttackReactionItem>),
    Attenuation(Box<Attenuation>),
    AttenuationType(Box<AttenuationType>),
    AutoplaceControlID(Box<AutoplaceControlID>),
    AutoplaceSettings(Box<AutoplaceSettings>),
    AutoplaceSpecification(Box<AutoplaceSpecification>),
    BaseAttackParameters(Box<BaseAttackParameters>),
    BaseEnergySource(Box<BaseEnergySource>),
    BaseModifier(Box<BaseModifier>),
    BaseStyleSpecification(Box<BaseStyleSpecification>),
    BeaconDistributionModifier(Box<BeaconDistributionModifier>),
    BeaconGraphicsSet(Box<BeaconGraphicsSet>),
    BeaconModuleVisualization(Box<BeaconModuleVisualization>),
    BeaconModuleVisualizations(Box<BeaconModuleVisualizations>),
    BeamAnimationSet(Box<BeamAnimationSet>),
    BeamAttackParameters(Box<BeamAttackParameters>),
    BeamGraphicsSet(Box<BeamGraphicsSet>),
    BeamTriggerDelivery(Box<BeamTriggerDelivery>),
    BeltReaderLayer(Box<BeltReaderLayer>),
    BeltStackSizeBonusModifier(Box<BeltStackSizeBonusModifier>),
    BeltTraverseTipTrigger(Box<BeltTraverseTipTrigger>),
    BlendMode(Box<BlendMode>),
    BoilerPictures(Box<BoilerPictures>),
    BoolModifier(Box<BoolModifier>),
    BorderImageSet(Box<BorderImageSet>),
    BoundingBox(Box<BoundingBox>),
    BoxSpecification(Box<BoxSpecification>),
    BuildEntityByRobotTipTrigger(Box<BuildEntityByRobotTipTrigger>),
    BuildEntityTechnologyTrigger(Box<BuildEntityTechnologyTrigger>),
    BuildEntityTipTrigger(Box<BuildEntityTipTrigger>),
    BuildMode(Box<BuildMode>),
    BulkInserterCapacityBonusModifier(Box<BulkInserterCapacityBonusModifier>),
    BurnerEnergySource(Box<BurnerEnergySource>),
    BurnerUsageID(Box<BurnerUsageID>),
    ButtonStyleSpecification(Box<ButtonStyleSpecification>),
    CameraEffectTriggerEffectItem(Box<CameraEffectTriggerEffectItem>),
    CameraStyleSpecification(Box<CameraStyleSpecification>),
    CapsuleAction(Box<CapsuleAction>),
    CaptureSpawnerTechnologyTrigger(Box<CaptureSpawnerTechnologyTrigger>),
    CargoBayConnectableGraphicsSet(Box<CargoBayConnectableGraphicsSet>),
    CargoBayConnections(Box<CargoBayConnections>),
    CargoHatchDefinition(Box<CargoHatchDefinition>),
    CargoLandingPadLimitModifier(Box<CargoLandingPadLimitModifier>),
    CargoStationParameters(Box<CargoStationParameters>),
    ChainTriggerDelivery(Box<ChainTriggerDelivery>),
    ChangeRecipeProductivityModifier(Box<ChangeRecipeProductivityModifier>),
    ChangeSurfaceTipTrigger(Box<ChangeSurfaceTipTrigger>),
    CharacterArmorAnimation(Box<CharacterArmorAnimation>),
    CharacterBuildDistanceModifier(Box<CharacterBuildDistanceModifier>),
    CharacterCraftingSpeedModifier(Box<CharacterCraftingSpeedModifier>),
    CharacterHealthBonusModifier(Box<CharacterHealthBonusModifier>),
    CharacterInventorySlotsBonusModifier(Box<CharacterInventorySlotsBonusModifier>),
    CharacterItemDropDistanceModifier(Box<CharacterItemDropDistanceModifier>),
    CharacterItemPickupDistanceModifier(Box<CharacterItemPickupDistanceModifier>),
    CharacterLogisticRequestsModifier(Box<CharacterLogisticRequestsModifier>),
    CharacterLogisticTrashSlotsModifier(Box<CharacterLogisticTrashSlotsModifier>),
    CharacterLootPickupDistanceModifier(Box<CharacterLootPickupDistanceModifier>),
    CharacterMiningSpeedModifier(Box<CharacterMiningSpeedModifier>),
    CharacterReachDistanceModifier(Box<CharacterReachDistanceModifier>),
    CharacterResourceReachDistanceModifier(Box<CharacterResourceReachDistanceModifier>),
    CharacterRunningSpeedModifier(Box<CharacterRunningSpeedModifier>),
    ChargableGraphics(Box<ChargableGraphics>),
    ChartUtilityConstants(Box<ChartUtilityConstants>),
    CheckBoxStyleSpecification(Box<CheckBoxStyleSpecification>),
    CircuitConnectorDefinition(Box<CircuitConnectorDefinition>),
    CircuitConnectorLayer(Box<CircuitConnectorLayer>),
    CircuitConnectorSecondaryDrawOrder(Box<CircuitConnectorSecondaryDrawOrder>),
    CircuitConnectorSprites(Box<CircuitConnectorSprites>),
    CircuitNetworkModifier(Box<CircuitNetworkModifier>),
    CircularParticleCreationSpecification(Box<CircularParticleCreationSpecification>),
    CircularProjectileCreationSpecification(Box<CircularProjectileCreationSpecification>),
    ClearCursorTipTrigger(Box<ClearCursorTipTrigger>),
    CliffDeconstructionEnabledModifier(Box<CliffDeconstructionEnabledModifier>),
    CliffPlacementSettings(Box<CliffPlacementSettings>),
    CloudEffectStyle(Box<CloudEffectStyle>),
    CloudsEffectProperties(Box<CloudsEffectProperties>),
    CloudsTextureCoordinateTransformation(Box<CloudsTextureCoordinateTransformation>),
    ClusterTriggerItem(Box<ClusterTriggerItem>),
    CollisionLayerID(Box<CollisionLayerID>),
    CollisionMaskConnector(Box<CollisionMaskConnector>),
    Color(Box<Color>),
    ColorHintSpecification(Box<ColorHintSpecification>),
    ColorLookupTable(Box<ColorLookupTable>),
    ColumnAlignment(Box<ColumnAlignment>),
    ColumnWidth(Box<ColumnWidth>),
    ColumnWidthItem(Box<ColumnWidthItem>),
    ComparatorString(Box<ComparatorString>),
    ConnectableEntityGraphics(Box<ConnectableEntityGraphics>),
    ConsumingType(Box<ConsumingType>),
    ControlPoint(Box<ControlPoint>),
    CountBasedTipTrigger(Box<CountBasedTipTrigger>),
    CoverGraphicEffectData(Box<CoverGraphicEffectData>),
    CoverGraphicProcessionLayer(Box<CoverGraphicProcessionLayer>),
    CraftFluidTechnologyTrigger(Box<CraftFluidTechnologyTrigger>),
    CraftItemTechnologyTrigger(Box<CraftItemTechnologyTrigger>),
    CraftItemTipTrigger(Box<CraftItemTipTrigger>),
    CraftingMachineGraphicsSet(Box<CraftingMachineGraphicsSet>),
    CranePart(Box<CranePart>),
    CranePartDyingEffect(Box<CranePartDyingEffect>),
    CraterPlacementDefinition(Box<CraterPlacementDefinition>),
    CreateAsteroidChunkEffectItem(Box<CreateAsteroidChunkEffectItem>),
    CreateDecorativesTriggerEffectItem(Box<CreateDecorativesTriggerEffectItem>),
    CreateEntityTriggerEffectItem(Box<CreateEntityTriggerEffectItem>),
    CreateExplosionTriggerEffectItem(Box<CreateExplosionTriggerEffectItem>),
    CreateFireTriggerEffectItem(Box<CreateFireTriggerEffectItem>),
    CreateGhostOnEntityDeathModifier(Box<CreateGhostOnEntityDeathModifier>),
    CreateParticleTriggerEffectItem(Box<CreateParticleTriggerEffectItem>),
    CreateSmokeTriggerEffectItem(Box<CreateSmokeTriggerEffectItem>),
    CreateSpacePlatformTechnologyTrigger(Box<CreateSpacePlatformTechnologyTrigger>),
    CreateStickerTriggerEffectItem(Box<CreateStickerTriggerEffectItem>),
    CreateTrivialSmokeEffectItem(Box<CreateTrivialSmokeEffectItem>),
    CursorBoxType(Box<CursorBoxType>),
    CyclicSound(Box<CyclicSound>),
    DamageParameters(Box<DamageParameters>),
    DamageTriggerEffectItem(Box<DamageTriggerEffectItem>),
    DamageTypeFilters(Box<DamageTypeFilters>),
    DamageTypeID(Box<DamageTypeID>),
    Data(Box<Data>),
    DataExtendMethod(Box<DataExtendMethod>),
    DaytimeColorLookupTable(Box<DaytimeColorLookupTable>),
    DeconstructionTimeToLiveModifier(Box<DeconstructionTimeToLiveModifier>),
    DecorativeID(Box<DecorativeID>),
    DelayedTriggerDelivery(Box<DelayedTriggerDelivery>),
    DependenciesMetTipTrigger(Box<DependenciesMetTipTrigger>),
    DestroyCliffsCapsuleAction(Box<DestroyCliffsCapsuleAction>),
    DestroyCliffsTriggerEffectItem(Box<DestroyCliffsTriggerEffectItem>),
    DestroyDecorativesTriggerEffectItem(Box<DestroyDecorativesTriggerEffectItem>),
    DifficultySettings(Box<DifficultySettings>),
    DirectTriggerItem(Box<DirectTriggerItem>),
    Direction(Box<Direction>),
    DirectionString(Box<DirectionString>),
    DoubleSliderStyleSpecification(Box<DoubleSliderStyleSpecification>),
    DropDownStyleSpecification(Box<DropDownStyleSpecification>),
    DropItemTipTrigger(Box<DropItemTipTrigger>),
    Effect(Box<Effect>),
    EffectReceiver(Box<EffectReceiver>),
    EffectRelativeTo(Box<EffectRelativeTo>),
    EffectTexture(Box<EffectTexture>),
    EffectTypeLimitation(Box<EffectTypeLimitation>),
    EffectValue(Box<EffectValue>),
    EffectVariation(Box<EffectVariation>),
    ElectricEnergySource(Box<ElectricEnergySource>),
    ElectricUsagePriority(Box<ElectricUsagePriority>),
    ElementImageSet(Box<ElementImageSet>),
    ElementImageSetLayer(Box<ElementImageSetLayer>),
    EmptyWidgetStyleSpecification(Box<EmptyWidgetStyleSpecification>),
    EnemyEvolutionSettings(Box<EnemyEvolutionSettings>),
    EnemyExpansionSettings(Box<EnemyExpansionSettings>),
    EnemySpawnerAbsorption(Box<EnemySpawnerAbsorption>),
    EnemySpawnerGraphicsSet(Box<EnemySpawnerGraphicsSet>),
    Energy(Box<Energy>),
    EnergySource(Box<EnergySource>),
    EnterVehicleTipTrigger(Box<EnterVehicleTipTrigger>),
    EntityBuildAnimationPiece(Box<EntityBuildAnimationPiece>),
    EntityID(Box<EntityID>),
    EntityIDFilter(Box<EntityIDFilter>),
    EntityPrototypeFlags(Box<EntityPrototypeFlags>),
    EntityStatus(Box<EntityStatus>),
    EntityTransferTipTrigger(Box<EntityTransferTipTrigger>),
    EquipmentCategoryID(Box<EquipmentCategoryID>),
    EquipmentGridID(Box<EquipmentGridID>),
    EquipmentID(Box<EquipmentID>),
    EquipmentShape(Box<EquipmentShape>),
    ExplosionDefinition(Box<ExplosionDefinition>),
    Fade(Box<Fade>),
    Fades(Box<Fades>),
    FastBeltBendTipTrigger(Box<FastBeltBendTipTrigger>),
    FastReplaceTipTrigger(Box<FastReplaceTipTrigger>),
    FeatureFlags(Box<FeatureFlags>),
    FileName(Box<FileName>),
    FlipEntityTipTrigger(Box<FlipEntityTipTrigger>),
    FlowStyleSpecification(Box<FlowStyleSpecification>),
    FluidAmount(Box<FluidAmount>),
    FluidBox(Box<FluidBox>),
    FluidBoxLinkedConnectionID(Box<FluidBoxLinkedConnectionID>),
    FluidEnergySource(Box<FluidEnergySource>),
    FluidID(Box<FluidID>),
    FluidIngredientPrototype(Box<FluidIngredientPrototype>),
    FluidProductPrototype(Box<FluidProductPrototype>),
    FogEffectProperties(Box<FogEffectProperties>),
    FogMaskShapeDefinition(Box<FogMaskShapeDefinition>),
    FollowerRobotLifetimeModifier(Box<FollowerRobotLifetimeModifier>),
    FootprintParticle(Box<FootprintParticle>),
    FootstepTriggerEffectItem(Box<FootstepTriggerEffectItem>),
    FootstepTriggerEffectList(Box<FootstepTriggerEffectList>),
    ForceCondition(Box<ForceCondition>),
    FrameStyleSpecification(Box<FrameStyleSpecification>),
    FrequencySizeRichness(Box<FrequencySizeRichness>),
    FuelCategoryID(Box<FuelCategoryID>),
    FusionGeneratorDirectionGraphicsSet(Box<FusionGeneratorDirectionGraphicsSet>),
    FusionGeneratorFluidInputGraphics(Box<FusionGeneratorFluidInputGraphics>),
    FusionGeneratorGraphicsSet(Box<FusionGeneratorGraphicsSet>),
    FusionReactorConnectionGraphics(Box<FusionReactorConnectionGraphics>),
    FusionReactorGraphicsSet(Box<FusionReactorGraphicsSet>),
    GameControllerVibrationData(Box<GameControllerVibrationData>),
    GameViewSettings(Box<GameViewSettings>),
    GateOverRailBuildTipTrigger(Box<GateOverRailBuildTipTrigger>),
    GeneratingPowerTipTrigger(Box<GeneratingPowerTipTrigger>),
    GhostShimmerConfig(Box<GhostShimmerConfig>),
    GhostShimmerDistortionData(Box<GhostShimmerDistortionData>),
    GhostShimmerOverlayData(Box<GhostShimmerOverlayData>),
    GigaCargoHatchDefinition(Box<GigaCargoHatchDefinition>),
    GiveItemModifier(Box<GiveItemModifier>),
    GlobalRecipeTints(Box<GlobalRecipeTints>),
    GlobalTintEffectProperties(Box<GlobalTintEffectProperties>),
    GlowStyleSpecification(Box<GlowStyleSpecification>),
    GraphStyleSpecification(Box<GraphStyleSpecification>),
    GroupAttackTipTrigger(Box<GroupAttackTipTrigger>),
    GunSpeedModifier(Box<GunSpeedModifier>),
    HeatBuffer(Box<HeatBuffer>),
    HeatConnection(Box<HeatConnection>),
    HeatEnergySource(Box<HeatEnergySource>),
    HorizontalAlign(Box<HorizontalAlign>),
    HorizontalFlowStyleSpecification(Box<HorizontalFlowStyleSpecification>),
    HorizontalScrollBarStyleSpecification(Box<HorizontalScrollBarStyleSpecification>),
    IconData(Box<IconData>),
    IconDrawSpecification(Box<IconDrawSpecification>),
    IconSequencePositioning(Box<IconSequencePositioning>),
    ImageStyleSpecification(Box<ImageStyleSpecification>),
    IngredientPrototype(Box<IngredientPrototype>),
    InsertItemTriggerEffectItem(Box<InsertItemTriggerEffectItem>),
    InserterStackSizeBonusModifier(Box<InserterStackSizeBonusModifier>),
    InstantTriggerDelivery(Box<InstantTriggerDelivery>),
    InterruptibleSound(Box<InterruptibleSound>),
    InvokeTileEffectTriggerEffectItem(Box<InvokeTileEffectTriggerEffectItem>),
    ItemCountType(Box<ItemCountType>),
    ItemGroupID(Box<ItemGroupID>),
    ItemID(Box<ItemID>),
    ItemIDFilter(Box<ItemIDFilter>),
    ItemIngredientPrototype(Box<ItemIngredientPrototype>),
    ItemProductPrototype(Box<ItemProductPrototype>),
    ItemPrototypeFlags(Box<ItemPrototypeFlags>),
    ItemStackIndex(Box<ItemStackIndex>),
    ItemSubGroupID(Box<ItemSubGroupID>),
    ItemToPlace(Box<ItemToPlace>),
    KillTipTrigger(Box<KillTipTrigger>),
    LabelStyleSpecification(Box<LabelStyleSpecification>),
    LaboratoryProductivityModifier(Box<LaboratoryProductivityModifier>),
    LaboratorySpeedModifier(Box<LaboratorySpeedModifier>),
    LayeredSound(Box<LayeredSound>),
    LayeredSprite(Box<LayeredSprite>),
    LayeredSpriteVariations(Box<LayeredSpriteVariations>),
    LightDefinition(Box<LightDefinition>),
    LightFlickeringDefinition(Box<LightFlickeringDefinition>),
    LightProperties(Box<LightProperties>),
    LightningGraphicsSet(Box<LightningGraphicsSet>),
    LightningPriorityRule(Box<LightningPriorityRule>),
    LightningProperties(Box<LightningProperties>),
    LightningRuleBase(Box<LightningRuleBase>),
    LimitChestTipTrigger(Box<LimitChestTipTrigger>),
    LineStyleSpecification(Box<LineStyleSpecification>),
    LineTriggerItem(Box<LineTriggerItem>),
    LinkedGameControl(Box<LinkedGameControl>),
    ListBoxStyleSpecification(Box<ListBoxStyleSpecification>),
    LoaderStructure(Box<LoaderStructure>),
    LocalisedString(Box<LocalisedString>),
    LogisticFilterIndex(Box<LogisticFilterIndex>),
    LootItem(Box<LootItem>),
    LowPowerTipTrigger(Box<LowPowerTipTrigger>),
    MainSound(Box<MainSound>),
    ManualTransferTipTrigger(Box<ManualTransferTipTrigger>),
    ManualWireDragTipTrigger(Box<ManualWireDragTipTrigger>),
    MapGenPreset(Box<MapGenPreset>),
    MapGenPresetAsteroidSettings(Box<MapGenPresetAsteroidSettings>),
    MapGenPresetDifficultySettings(Box<MapGenPresetDifficultySettings>),
    MapGenPresetEnemyEvolutionSettings(Box<MapGenPresetEnemyEvolutionSettings>),
    MapGenPresetEnemyExpansionSettings(Box<MapGenPresetEnemyExpansionSettings>),
    MapGenPresetPollutionSettings(Box<MapGenPresetPollutionSettings>),
    MapGenSettings(Box<MapGenSettings>),
    MapGenSize(Box<MapGenSize>),
    MapPosition(Box<MapPosition>),
    MapTick(Box<MapTick>),
    MaterialAmountType(Box<MaterialAmountType>),
    MaterialTextureParameters(Box<MaterialTextureParameters>),
    MathExpression(Box<MathExpression>),
    MaxFailedAttemptsPerTickPerConstructionQueueModifier(
        Box<MaxFailedAttemptsPerTickPerConstructionQueueModifier>,
    ),
    MaxSuccessfulAttemptsPerTickPerConstructionQueueModifier(
        Box<MaxSuccessfulAttemptsPerTickPerConstructionQueueModifier>,
    ),
    MaximumFollowingRobotsCountModifier(Box<MaximumFollowingRobotsCountModifier>),
    MinableProperties(Box<MinableProperties>),
    MineEntityTechnologyTrigger(Box<MineEntityTechnologyTrigger>),
    MineItemByRobotTipTrigger(Box<MineItemByRobotTipTrigger>),
    MinimapStyleSpecification(Box<MinimapStyleSpecification>),
    MiningDrillGraphicsSet(Box<MiningDrillGraphicsSet>),
    MiningDrillProductivityBonusModifier(Box<MiningDrillProductivityBonusModifier>),
    MiningWithFluidModifier(Box<MiningWithFluidModifier>),
    Mirroring(Box<Mirroring>),
    ModSetting(Box<ModSetting>),
    Modifier(Box<Modifier>),
    Mods(Box<Mods>),
    ModuleCategoryID(Box<ModuleCategoryID>),
    ModuleTint(Box<ModuleTint>),
    ModuleTransferTipTrigger(Box<ModuleTransferTipTrigger>),
    MouseCursorID(Box<MouseCursorID>),
    NeighbourConnectable(Box<NeighbourConnectable>),
    NeighbourConnectableConnectionCategory(Box<NeighbourConnectableConnectionCategory>),
    NeighbourConnectableConnectionDefinition(Box<NeighbourConnectableConnectionDefinition>),
    NestedTriggerEffectItem(Box<NestedTriggerEffectItem>),
    NoiseExpression(Box<NoiseExpression>),
    NoiseFunction(Box<NoiseFunction>),
    NothingModifier(Box<NothingModifier>),
    OffshorePumpGraphicsSet(Box<OffshorePumpGraphicsSet>),
    OrTipTrigger(Box<OrTipTrigger>),
    Order(Box<Order>),
    OrientedCliffPrototype(Box<OrientedCliffPrototype>),
    ParticleID(Box<ParticleID>),
    PasteEntitySettingsTipTrigger(Box<PasteEntitySettingsTipTrigger>),
    PathFinderSettings(Box<PathFinderSettings>),
    PerceivedPerformance(Box<PerceivedPerformance>),
    PersistentWorldAmbientSoundDefinition(Box<PersistentWorldAmbientSoundDefinition>),
    PersistentWorldAmbientSoundsDefinition(Box<PersistentWorldAmbientSoundsDefinition>),
    PersistentWorldAmbientSoundsDefinitionCrossfade(
        Box<PersistentWorldAmbientSoundsDefinitionCrossfade>,
    ),
    PipeConnectionDefinition(Box<PipeConnectionDefinition>),
    PipePictures(Box<PipePictures>),
    PlaceEquipmentTipTrigger(Box<PlaceEquipmentTipTrigger>),
    PlanTrainPathTipTrigger(Box<PlanTrainPathTipTrigger>),
    PlanetPrototypeMapGenSettings(Box<PlanetPrototypeMapGenSettings>),
    PlaySoundTriggerEffectItem(Box<PlaySoundTriggerEffectItem>),
    PlayerInputMethodFilter(Box<PlayerInputMethodFilter>),
    PlumeEffect(Box<PlumeEffect>),
    PlumesSpecification(Box<PlumesSpecification>),
    PodAnimationProcessionLayer(Box<PodAnimationProcessionLayer>),
    PodDistanceTraveledProcessionLayer(Box<PodDistanceTraveledProcessionLayer>),
    PodMovementProcessionLayer(Box<PodMovementProcessionLayer>),
    PodOpacityProcessionLayer(Box<PodOpacityProcessionLayer>),
    PollutionSettings(Box<PollutionSettings>),
    ProbabilityTable(Box<ProbabilityTable>),
    ProbabilityTableItem(Box<ProbabilityTableItem>),
    ProcessionAudio(Box<ProcessionAudio>),
    ProcessionAudioCatalogue(Box<ProcessionAudioCatalogue>),
    ProcessionAudioCatalogueItem(Box<ProcessionAudioCatalogueItem>),
    ProcessionAudioEvent(Box<ProcessionAudioEvent>),
    ProcessionAudioEventType(Box<ProcessionAudioEventType>),
    ProcessionAudioType(Box<ProcessionAudioType>),
    ProcessionAudioUsage(Box<ProcessionAudioUsage>),
    ProcessionGraphic(Box<ProcessionGraphic>),
    ProcessionGraphicCatalogue(Box<ProcessionGraphicCatalogue>),
    ProcessionGraphicCatalogueItem(Box<ProcessionGraphicCatalogueItem>),
    ProcessionGraphicType(Box<ProcessionGraphicType>),
    ProcessionID(Box<ProcessionID>),
    ProcessionLayer(Box<ProcessionLayer>),
    ProcessionLayerInheritanceGroupID(Box<ProcessionLayerInheritanceGroupID>),
    ProcessionLayerWithTime(Box<ProcessionLayerWithTime>),
    ProcessionSet(Box<ProcessionSet>),
    ProcessionTimeline(Box<ProcessionTimeline>),
    ProductPrototype(Box<ProductPrototype>),
    ProductionHealthEffect(Box<ProductionHealthEffect>),
    ProgrammableSpeakerNote(Box<ProgrammableSpeakerNote>),
    ProgressBarStyleSpecification(Box<ProgressBarStyleSpecification>),
    ProjectileAttackParameters(Box<ProjectileAttackParameters>),
    ProjectileTriggerDelivery(Box<ProjectileTriggerDelivery>),
    PrototypeStrafeSettings(Box<PrototypeStrafeSettings>),
    PuddleTileEffectParameters(Box<PuddleTileEffectParameters>),
    PumpConnectorGraphics(Box<PumpConnectorGraphics>),
    PumpConnectorGraphicsAnimation(Box<PumpConnectorGraphicsAnimation>),
    PushBackTriggerEffectItem(Box<PushBackTriggerEffectItem>),
    QualityID(Box<QualityID>),
    RadioButtonStyleSpecification(Box<RadioButtonStyleSpecification>),
    RadiusVisualisationSpecification(Box<RadiusVisualisationSpecification>),
    RailFenceDirectionSet(Box<RailFenceDirectionSet>),
    RailFenceGraphicsSet(Box<RailFenceGraphicsSet>),
    RailFencePictureSet(Box<RailFencePictureSet>),
    RailPictureSet(Box<RailPictureSet>),
    RailPieceLayers(Box<RailPieceLayers>),
    RailPlannerAllowElevatedRailsModifier(Box<RailPlannerAllowElevatedRailsModifier>),
    RailRenderLayers(Box<RailRenderLayers>),
    RailSignalColorToFrameIndex(Box<RailSignalColorToFrameIndex>),
    RailSignalLightDefinition(Box<RailSignalLightDefinition>),
    RailSignalLights(Box<RailSignalLights>),
    RailSignalPictureSet(Box<RailSignalPictureSet>),
    RailSignalStaticSpriteLayer(Box<RailSignalStaticSpriteLayer>),
    RailSupportOnDeepOilOceanModifier(Box<RailSupportOnDeepOilOceanModifier>),
    RandomRange(Box<RandomRange>),
    RangeMode(Box<RangeMode>),
    RangedValue(Box<RangedValue>),
    RealOrientation(Box<RealOrientation>),
    RecipeCategoryID(Box<RecipeCategoryID>),
    RecipeID(Box<RecipeID>),
    RecipeTints(Box<RecipeTints>),
    RenderLayer(Box<RenderLayer>),
    ResearchIngredient(Box<ResearchIngredient>),
    ResearchProgressProductPrototype(Box<ResearchProgressProductPrototype>),
    ResearchTechnologyTipTrigger(Box<ResearchTechnologyTipTrigger>),
    ResearchWithSciencePackTipTrigger(Box<ResearchWithSciencePackTipTrigger>),
    Resistance(Box<Resistance>),
    ResourceCategoryID(Box<ResourceCategoryID>),
    RichTextSetting(Box<RichTextSetting>),
    RollingStockRotatedSlopedGraphics(Box<RollingStockRotatedSlopedGraphics>),
    RotateEntityTipTrigger(Box<RotateEntityTipTrigger>),
    RotatedAnimation(Box<RotatedAnimation>),
    RotatedAnimation8Way(Box<RotatedAnimation8Way>),
    RotatedAnimationVariations(Box<RotatedAnimationVariations>),
    RotatedSprite(Box<RotatedSprite>),
    RotatedSpriteFrame(Box<RotatedSpriteFrame>),
    ScriptTriggerEffectItem(Box<ScriptTriggerEffectItem>),
    ScrollBarStyleSpecification(Box<ScrollBarStyleSpecification>),
    ScrollPaneStyleSpecification(Box<ScrollPaneStyleSpecification>),
    SegmentEngineSpecification(Box<SegmentEngineSpecification>),
    SegmentSpecification(Box<SegmentSpecification>),
    SelectionModeData(Box<SelectionModeData>),
    SelectionModeFlags(Box<SelectionModeFlags>),
    SemiPersistentWorldAmbientSoundDefinition(Box<SemiPersistentWorldAmbientSoundDefinition>),
    SendItemToOrbitTechnologyTrigger(Box<SendItemToOrbitTechnologyTrigger>),
    SendSpidertronTipTrigger(Box<SendSpidertronTipTrigger>),
    SendToOrbitMode(Box<SendToOrbitMode>),
    SequenceTipTrigger(Box<SequenceTipTrigger>),
    SetFilterTipTrigger(Box<SetFilterTipTrigger>),
    SetLogisticRequestTipTrigger(Box<SetLogisticRequestTipTrigger>),
    SetRecipeTipTrigger(Box<SetRecipeTipTrigger>),
    SetTileTriggerEffectItem(Box<SetTileTriggerEffectItem>),
    Settings(Box<Settings>),
    ShiftAnimationWaypoints(Box<ShiftAnimationWaypoints>),
    ShootTipTrigger(Box<ShootTipTrigger>),
    ShowExplosionOnChartTriggerEffectItem(Box<ShowExplosionOnChartTriggerEffectItem>),
    SignalColorMapping(Box<SignalColorMapping>),
    SignalIDConnector(Box<SignalIDConnector>),
    SimpleBoundingBox(Box<SimpleBoundingBox>),
    SimpleModifier(Box<SimpleModifier>),
    SimulationDefinition(Box<SimulationDefinition>),
    SingleGraphicProcessionLayer(Box<SingleGraphicProcessionLayer>),
    SliderStyleSpecification(Box<SliderStyleSpecification>),
    SmokeSource(Box<SmokeSource>),
    Sound(Box<Sound>),
    SoundAccent(Box<SoundAccent>),
    SoundDefinition(Box<SoundDefinition>),
    SoundModifier(Box<SoundModifier>),
    SoundModifierType(Box<SoundModifierType>),
    SoundType(Box<SoundType>),
    SpaceConnectionAsteroidSpawnDefinition(Box<SpaceConnectionAsteroidSpawnDefinition>),
    SpaceConnectionAsteroidSpawnPoint(Box<SpaceConnectionAsteroidSpawnPoint>),
    SpaceConnectionID(Box<SpaceConnectionID>),
    SpaceDustEffectProperties(Box<SpaceDustEffectProperties>),
    SpaceLocationAsteroidSpawnDefinition(Box<SpaceLocationAsteroidSpawnDefinition>),
    SpaceLocationID(Box<SpaceLocationID>),
    SpacePlatformTileDefinition(Box<SpacePlatformTileDefinition>),
    SpacePlatformsModifier(Box<SpacePlatformsModifier>),
    SpaceTileEffectParameters(Box<SpaceTileEffectParameters>),
    SpawnPoint(Box<SpawnPoint>),
    SpeechBubbleStyleSpecification(Box<SpeechBubbleStyleSpecification>),
    SpiderEngineSpecification(Box<SpiderEngineSpecification>),
    SpiderLegPart(Box<SpiderLegPart>),
    SpiderLegSpecification(Box<SpiderLegSpecification>),
    SpiderLegTriggerEffect(Box<SpiderLegTriggerEffect>),
    SpiderTorsoGraphicsSet(Box<SpiderTorsoGraphicsSet>),
    SpiderVehicleGraphicsSet(Box<SpiderVehicleGraphicsSet>),
    SpoilToTriggerResult(Box<SpoilToTriggerResult>),
    Sprite(Box<Sprite>),
    Sprite16Way(Box<Sprite16Way>),
    Sprite4Way(Box<Sprite4Way>),
    SpriteFlags(Box<SpriteFlags>),
    SpriteNWaySheet(Box<SpriteNWaySheet>),
    SpriteParameters(Box<SpriteParameters>),
    SpritePriority(Box<SpritePriority>),
    SpriteSheet(Box<SpriteSheet>),
    SpriteSizeType(Box<SpriteSizeType>),
    SpriteSource(Box<SpriteSource>),
    SpriteUsageHint(Box<SpriteUsageHint>),
    SpriteUsageSurfaceHint(Box<SpriteUsageSurfaceHint>),
    SpriteVariations(Box<SpriteVariations>),
    StackTransferTipTrigger(Box<StackTransferTipTrigger>),
    StateSteeringSettings(Box<StateSteeringSettings>),
    StatelessVisualisation(Box<StatelessVisualisation>),
    StatelessVisualisations(Box<StatelessVisualisations>),
    StatusColors(Box<StatusColors>),
    StreamAttackParameters(Box<StreamAttackParameters>),
    StreamTriggerDelivery(Box<StreamTriggerDelivery>),
    StretchRule(Box<StretchRule>),
    Stripe(Box<Stripe>),
    StyleSpecification(Box<StyleSpecification>),
    StyleWithClickableGraphicalSetSpecification(Box<StyleWithClickableGraphicalSetSpecification>),
    SurfaceCondition(Box<SurfaceCondition>),
    SurfaceID(Box<SurfaceID>),
    SurfacePropertyID(Box<SurfacePropertyID>),
    SurfaceRenderParameters(Box<SurfaceRenderParameters>),
    SwitchStyleSpecification(Box<SwitchStyleSpecification>),
    TabStyleSpecification(Box<TabStyleSpecification>),
    TabbedPaneStyleSpecification(Box<TabbedPaneStyleSpecification>),
    TableStyleSpecification(Box<TableStyleSpecification>),
    TechnologyID(Box<TechnologyID>),
    TechnologySlotStyleSpecification(Box<TechnologySlotStyleSpecification>),
    TechnologyTrigger(Box<TechnologyTrigger>),
    TechnologyUnit(Box<TechnologyUnit>),
    TerritorySettings(Box<TerritorySettings>),
    TextBoxStyleSpecification(Box<TextBoxStyleSpecification>),
    ThrowCapsuleAction(Box<ThrowCapsuleAction>),
    ThrusterGraphicsSet(Box<ThrusterGraphicsSet>),
    ThrusterPerformancePoint(Box<ThrusterPerformancePoint>),
    TileBasedParticleTints(Box<TileBasedParticleTints>),
    TileBuildabilityRule(Box<TileBuildabilityRule>),
    TileEffectDefinitionID(Box<TileEffectDefinitionID>),
    TileID(Box<TileID>),
    TileLightPictures(Box<TileLightPictures>),
    TileMainPictures(Box<TileMainPictures>),
    TilePosition(Box<TilePosition>),
    TileRenderLayer(Box<TileRenderLayer>),
    TileSpriteLayout(Box<TileSpriteLayout>),
    TileSpriteLayoutVariant(Box<TileSpriteLayoutVariant>),
    TileTransitionSpritesheetLayout(Box<TileTransitionSpritesheetLayout>),
    TileTransitionVariantLayout(Box<TileTransitionVariantLayout>),
    TileTransitions(Box<TileTransitions>),
    TileTransitionsBetweenTransitions(Box<TileTransitionsBetweenTransitions>),
    TileTransitionsToTiles(Box<TileTransitionsToTiles>),
    TileTransitionsVariants(Box<TileTransitionsVariants>),
    TimeElapsedTipTrigger(Box<TimeElapsedTipTrigger>),
    TimeSinceLastTipActivationTipTrigger(Box<TimeSinceLastTipActivationTipTrigger>),
    TintProcessionLayer(Box<TintProcessionLayer>),
    TipStatus(Box<TipStatus>),
    TipTrigger(Box<TipTrigger>),
    ToggleRailLayerTipTrigger(Box<ToggleRailLayerTipTrigger>),
    ToggleShowEntityInfoTipTrigger(Box<ToggleShowEntityInfoTipTrigger>),
    TrainBrakingForceBonusModifier(Box<TrainBrakingForceBonusModifier>),
    TrainStopLight(Box<TrainStopLight>),
    TrainVisualizationConstants(Box<TrainVisualizationConstants>),
    TransitionApplication(Box<TransitionApplication>),
    TransportBeltAnimationSet(Box<TransportBeltAnimationSet>),
    TransportBeltAnimationSetWithCorners(Box<TransportBeltAnimationSetWithCorners>),
    TransportBeltConnectorFrame(Box<TransportBeltConnectorFrame>),
    TreeVariation(Box<TreeVariation>),
    Trigger(Box<Trigger>),
    TriggerDelivery(Box<TriggerDelivery>),
    TriggerDeliveryItem(Box<TriggerDeliveryItem>),
    TriggerEffect(Box<TriggerEffect>),
    TriggerEffectItem(Box<TriggerEffectItem>),
    TriggerEffectWithCooldown(Box<TriggerEffectWithCooldown>),
    TriggerItem(Box<TriggerItem>),
    TriggerTargetMask(Box<TriggerTargetMask>),
    TrivialSmokeID(Box<TrivialSmokeID>),
    TurretAttackModifier(Box<TurretAttackModifier>),
    TurretBaseVisualisation(Box<TurretBaseVisualisation>),
    TurretGraphicsSet(Box<TurretGraphicsSet>),
    TurretSpecialEffect(Box<TurretSpecialEffect>),
    TurretSpecialEffectCenter(Box<TurretSpecialEffectCenter>),
    TurretState(Box<TurretState>),
    UnitAISettings(Box<UnitAISettings>),
    UnitAlternativeFrameSequence(Box<UnitAlternativeFrameSequence>),
    UnitGroupSettings(Box<UnitGroupSettings>),
    UnitSpawnDefinition(Box<UnitSpawnDefinition>),
    UnlockQualityModifier(Box<UnlockQualityModifier>),
    UnlockRecipeModifier(Box<UnlockRecipeModifier>),
    UnlockRecipeTipTrigger(Box<UnlockRecipeTipTrigger>),
    UnlockSpaceLocationModifier(Box<UnlockSpaceLocationModifier>),
    UseConfirmTipTrigger(Box<UseConfirmTipTrigger>),
    UseOnSelfCapsuleAction(Box<UseOnSelfCapsuleAction>),
    UsePipetteTipTrigger(Box<UsePipetteTipTrigger>),
    UseRailPlannerTipTrigger(Box<UseRailPlannerTipTrigger>),
    VariableAmbientSoundCompositionMode(Box<VariableAmbientSoundCompositionMode>),
    VariableAmbientSoundLayer(Box<VariableAmbientSoundLayer>),
    VariableAmbientSoundLayerSample(Box<VariableAmbientSoundLayerSample>),
    VariableAmbientSoundLayerStateProperties(Box<VariableAmbientSoundLayerStateProperties>),
    VariableAmbientSoundNextStateConditions(Box<VariableAmbientSoundNextStateConditions>),
    VariableAmbientSoundNextStateItem(Box<VariableAmbientSoundNextStateItem>),
    VariableAmbientSoundNextStateTrigger(Box<VariableAmbientSoundNextStateTrigger>),
    VariableAmbientSoundState(Box<VariableAmbientSoundState>),
    VariableAmbientSoundStateType(Box<VariableAmbientSoundStateType>),
    VariableAmbientSoundVariableSound(Box<VariableAmbientSoundVariableSound>),
    Vector(Box<Vector>),
    Vector3D(Box<Vector3D>),
    Vector4f(Box<Vector4f>),
    VehicleLogisticsModifier(Box<VehicleLogisticsModifier>),
    VerticalAlign(Box<VerticalAlign>),
    VerticalFlowStyleSpecification(Box<VerticalFlowStyleSpecification>),
    VerticalScrollBarStyleSpecification(Box<VerticalScrollBarStyleSpecification>),
    VirtualSignalID(Box<VirtualSignalID>),
    VisualState(Box<VisualState>),
    VoidEnergySource(Box<VoidEnergySource>),
    WaterReflectionDefinition(Box<WaterReflectionDefinition>),
    WaterTileEffectParameters(Box<WaterTileEffectParameters>),
    Weight(Box<Weight>),
    WireConnectionPoint(Box<WireConnectionPoint>),
    WirePosition(Box<WirePosition>),
    WorkerRobotBatteryModifier(Box<WorkerRobotBatteryModifier>),
    WorkerRobotSpeedModifier(Box<WorkerRobotSpeedModifier>),
    WorkerRobotStorageModifier(Box<WorkerRobotStorageModifier>),
    WorkingSound(Box<WorkingSound>),
    WorkingVisualisation(Box<WorkingVisualisation>),
    WorkingVisualisations(Box<WorkingVisualisations>),
    WorldAmbientSoundDefinition(Box<WorldAmbientSoundDefinition>),
}
