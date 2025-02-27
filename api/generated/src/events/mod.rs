#![allow(dead_code)]
pub mod script_raised_teleported;
pub use script_raised_teleported::ScriptRaisedTeleported;
pub mod script_raised_set_tiles;
pub use script_raised_set_tiles::ScriptRaisedSetTiles;
pub mod script_raised_revive;
pub use script_raised_revive::ScriptRaisedRevive;
pub mod script_raised_destroy;
pub use script_raised_destroy::ScriptRaisedDestroy;
pub mod script_raised_built;
pub use script_raised_built::ScriptRaisedBuilt;
pub mod on_worker_robot_expired;
pub use on_worker_robot_expired::OnWorkerRobotExpired;
pub mod on_unit_removed_from_group;
pub use on_unit_removed_from_group::OnUnitRemovedFromGroup;
pub mod on_unit_group_finished_gathering;
pub use on_unit_group_finished_gathering::OnUnitGroupFinishedGathering;
pub mod on_unit_group_created;
pub use on_unit_group_created::OnUnitGroupCreated;
pub mod on_unit_added_to_group;
pub use on_unit_added_to_group::OnUnitAddedToGroup;
pub mod on_undo_applied;
pub use on_undo_applied::OnUndoApplied;
pub mod on_trigger_fired_artillery;
pub use on_trigger_fired_artillery::OnTriggerFiredArtillery;
pub mod on_trigger_created_entity;
pub use on_trigger_created_entity::OnTriggerCreatedEntity;
pub mod on_train_schedule_changed;
pub use on_train_schedule_changed::OnTrainScheduleChanged;
pub mod on_train_created;
pub use on_train_created::OnTrainCreated;
pub mod on_train_changed_state;
pub use on_train_changed_state::OnTrainChangedState;
pub mod on_tick;
pub use on_tick::OnTick;
pub mod on_technology_effects_reset;
pub use on_technology_effects_reset::OnTechnologyEffectsReset;
pub mod on_surface_renamed;
pub use on_surface_renamed::OnSurfaceRenamed;
pub mod on_surface_imported;
pub use on_surface_imported::OnSurfaceImported;
pub mod on_surface_deleted;
pub use on_surface_deleted::OnSurfaceDeleted;
pub mod on_surface_created;
pub use on_surface_created::OnSurfaceCreated;
pub mod on_surface_cleared;
pub use on_surface_cleared::OnSurfaceCleared;
pub mod on_string_translated;
pub use on_string_translated::OnStringTranslated;
pub mod on_spider_command_completed;
pub use on_spider_command_completed::OnSpiderCommandCompleted;
pub mod on_space_platform_pre_mined;
pub use on_space_platform_pre_mined::OnSpacePlatformPreMined;
pub mod on_space_platform_mined_tile;
pub use on_space_platform_mined_tile::OnSpacePlatformMinedTile;
pub mod on_space_platform_mined_item;
pub use on_space_platform_mined_item::OnSpacePlatformMinedItem;
pub mod on_space_platform_mined_entity;
pub use on_space_platform_mined_entity::OnSpacePlatformMinedEntity;
pub mod on_space_platform_changed_state;
pub use on_space_platform_changed_state::OnSpacePlatformChangedState;
pub mod on_space_platform_built_tile;
pub use on_space_platform_built_tile::OnSpacePlatformBuiltTile;
pub mod on_space_platform_built_entity;
pub use on_space_platform_built_entity::OnSpacePlatformBuiltEntity;
pub mod on_selected_entity_changed;
pub use on_selected_entity_changed::OnSelectedEntityChanged;
pub mod on_segment_entity_created;
pub use on_segment_entity_created::OnSegmentEntityCreated;
pub mod on_sector_scanned;
pub use on_sector_scanned::OnSectorScanned;
pub mod on_script_trigger_effect;
pub use on_script_trigger_effect::OnScriptTriggerEffect;
pub mod on_script_path_request_finished;
pub use on_script_path_request_finished::OnScriptPathRequestFinished;
pub mod on_script_inventory_resized;
pub use on_script_inventory_resized::OnScriptInventoryResized;
pub mod on_runtime_mod_setting_changed;
pub use on_runtime_mod_setting_changed::OnRuntimeModSettingChanged;
pub mod on_rocket_launched;
pub use on_rocket_launched::OnRocketLaunched;
pub mod on_rocket_launch_ordered;
pub use on_rocket_launch_ordered::OnRocketLaunchOrdered;
pub mod on_robot_pre_mined;
pub use on_robot_pre_mined::OnRobotPreMined;
pub mod on_robot_mined_tile;
pub use on_robot_mined_tile::OnRobotMinedTile;
pub mod on_robot_mined_entity;
pub use on_robot_mined_entity::OnRobotMinedEntity;
pub mod on_robot_mined;
pub use on_robot_mined::OnRobotMined;
pub mod on_robot_exploded_cliff;
pub use on_robot_exploded_cliff::OnRobotExplodedCliff;
pub mod on_robot_built_tile;
pub use on_robot_built_tile::OnRobotBuiltTile;
pub mod on_robot_built_entity;
pub use on_robot_built_entity::OnRobotBuiltEntity;
pub mod on_resource_depleted;
pub use on_resource_depleted::OnResourceDepleted;
pub mod on_research_started;
pub use on_research_started::OnResearchStarted;
pub mod on_research_reversed;
pub use on_research_reversed::OnResearchReversed;
pub mod on_research_moved;
pub use on_research_moved::OnResearchMoved;
pub mod on_research_finished;
pub use on_research_finished::OnResearchFinished;
pub mod on_research_cancelled;
pub use on_research_cancelled::OnResearchCancelled;
pub mod on_redo_applied;
pub use on_redo_applied::OnRedoApplied;
pub mod on_pre_surface_deleted;
pub use on_pre_surface_deleted::OnPreSurfaceDeleted;
pub mod on_pre_surface_cleared;
pub use on_pre_surface_cleared::OnPreSurfaceCleared;
pub mod on_pre_script_inventory_resized;
pub use on_pre_script_inventory_resized::OnPreScriptInventoryResized;
pub mod on_pre_scenario_finished;
pub use on_pre_scenario_finished::OnPreScenarioFinished;
pub mod on_pre_robot_exploded_cliff;
pub use on_pre_robot_exploded_cliff::OnPreRobotExplodedCliff;
pub mod on_pre_player_toggled_map_editor;
pub use on_pre_player_toggled_map_editor::OnPrePlayerToggledMapEditor;
pub mod on_pre_player_removed;
pub use on_pre_player_removed::OnPrePlayerRemoved;
pub mod on_pre_player_mined_item;
pub use on_pre_player_mined_item::OnPrePlayerMinedItem;
pub mod on_pre_player_left_game;
pub use on_pre_player_left_game::OnPrePlayerLeftGame;
pub mod on_pre_player_died;
pub use on_pre_player_died::OnPrePlayerDied;
pub mod on_pre_player_crafted_item;
pub use on_pre_player_crafted_item::OnPrePlayerCraftedItem;
pub mod on_pre_permission_string_imported;
pub use on_pre_permission_string_imported::OnPrePermissionStringImported;
pub mod on_pre_permission_group_deleted;
pub use on_pre_permission_group_deleted::OnPrePermissionGroupDeleted;
pub mod on_pre_ghost_upgraded;
pub use on_pre_ghost_upgraded::OnPreGhostUpgraded;
pub mod on_pre_ghost_deconstructed;
pub use on_pre_ghost_deconstructed::OnPreGhostDeconstructed;
pub mod on_pre_entity_settings_pasted;
pub use on_pre_entity_settings_pasted::OnPreEntitySettingsPasted;
pub mod on_pre_chunk_deleted;
pub use on_pre_chunk_deleted::OnPreChunkDeleted;
pub mod on_pre_build;
pub use on_pre_build::OnPreBuild;
pub mod on_post_entity_died;
pub use on_post_entity_died::OnPostEntityDied;
pub mod on_player_used_spidertron_remote;
pub use on_player_used_spidertron_remote::OnPlayerUsedSpidertronRemote;
pub mod on_player_used_capsule;
pub use on_player_used_capsule::OnPlayerUsedCapsule;
pub mod on_player_unmuted;
pub use on_player_unmuted::OnPlayerUnmuted;
pub mod on_player_unbanned;
pub use on_player_unbanned::OnPlayerUnbanned;
pub mod on_player_trash_inventory_changed;
pub use on_player_trash_inventory_changed::OnPlayerTrashInventoryChanged;
pub mod on_player_toggled_map_editor;
pub use on_player_toggled_map_editor::OnPlayerToggledMapEditor;
pub mod on_player_toggled_alt_mode;
pub use on_player_toggled_alt_mode::OnPlayerToggledAltMode;
pub mod on_player_setup_blueprint;
pub use on_player_setup_blueprint::OnPlayerSetupBlueprint;
pub mod on_player_set_quick_bar_slot;
pub use on_player_set_quick_bar_slot::OnPlayerSetQuickBarSlot;
pub mod on_player_selected_area;
pub use on_player_selected_area::OnPlayerSelectedArea;
pub mod on_player_rotated_entity;
pub use on_player_rotated_entity::OnPlayerRotatedEntity;
pub mod on_player_reverse_selected_area;
pub use on_player_reverse_selected_area::OnPlayerReverseSelectedArea;
pub mod on_player_respawned;
pub use on_player_respawned::OnPlayerRespawned;
pub mod on_player_repaired_entity;
pub use on_player_repaired_entity::OnPlayerRepairedEntity;
pub mod on_player_removed_equipment;
pub use on_player_removed_equipment::OnPlayerRemovedEquipment;
pub mod on_player_removed;
pub use on_player_removed::OnPlayerRemoved;
pub mod on_player_promoted;
pub use on_player_promoted::OnPlayerPromoted;
pub mod on_player_placed_equipment;
pub use on_player_placed_equipment::OnPlayerPlacedEquipment;
pub mod on_player_pipette;
pub use on_player_pipette::OnPlayerPipette;
pub mod on_player_muted;
pub use on_player_muted::OnPlayerMuted;
pub mod on_player_mined_tile;
pub use on_player_mined_tile::OnPlayerMinedTile;
pub mod on_player_mined_item;
pub use on_player_mined_item::OnPlayerMinedItem;
pub mod on_player_mined_entity;
pub use on_player_mined_entity::OnPlayerMinedEntity;
pub mod on_player_main_inventory_changed;
pub use on_player_main_inventory_changed::OnPlayerMainInventoryChanged;
pub mod on_player_locale_changed;
pub use on_player_locale_changed::OnPlayerLocaleChanged;
pub mod on_player_left_game;
pub use on_player_left_game::OnPlayerLeftGame;
pub mod on_player_kicked;
pub use on_player_kicked::OnPlayerKicked;
pub mod on_player_joined_game;
pub use on_player_joined_game::OnPlayerJoinedGame;
pub mod on_player_input_method_changed;
pub use on_player_input_method_changed::OnPlayerInputMethodChanged;
pub mod on_player_gun_inventory_changed;
pub use on_player_gun_inventory_changed::OnPlayerGunInventoryChanged;
pub mod on_player_flushed_fluid;
pub use on_player_flushed_fluid::OnPlayerFlushedFluid;
pub mod on_player_flipped_entity;
pub use on_player_flipped_entity::OnPlayerFlippedEntity;
pub mod on_player_fast_transferred;
pub use on_player_fast_transferred::OnPlayerFastTransferred;
pub mod on_player_dropped_item;
pub use on_player_dropped_item::OnPlayerDroppedItem;
pub mod on_player_driving_changed_state;
pub use on_player_driving_changed_state::OnPlayerDrivingChangedState;
pub mod on_player_display_scale_changed;
pub use on_player_display_scale_changed::OnPlayerDisplayScaleChanged;
pub mod on_player_display_resolution_changed;
pub use on_player_display_resolution_changed::OnPlayerDisplayResolutionChanged;
pub mod on_player_display_density_scale_changed;
pub use on_player_display_density_scale_changed::OnPlayerDisplayDensityScaleChanged;
pub mod on_player_died;
pub use on_player_died::OnPlayerDied;
pub mod on_player_demoted;
pub use on_player_demoted::OnPlayerDemoted;
pub mod on_player_deconstructed_area;
pub use on_player_deconstructed_area::OnPlayerDeconstructedArea;
pub mod on_player_cursor_stack_changed;
pub use on_player_cursor_stack_changed::OnPlayerCursorStackChanged;
pub mod on_player_created;
pub use on_player_created::OnPlayerCreated;
pub mod on_player_crafted_item;
pub use on_player_crafted_item::OnPlayerCraftedItem;
pub mod on_player_controller_changed;
pub use on_player_controller_changed::OnPlayerControllerChanged;
pub mod on_player_configured_blueprint;
pub use on_player_configured_blueprint::OnPlayerConfiguredBlueprint;
pub mod on_player_clicked_gps_tag;
pub use on_player_clicked_gps_tag::OnPlayerClickedGpsTag;
pub mod on_player_cheat_mode_enabled;
pub use on_player_cheat_mode_enabled::OnPlayerCheatModeEnabled;
pub mod on_player_cheat_mode_disabled;
pub use on_player_cheat_mode_disabled::OnPlayerCheatModeDisabled;
pub mod on_player_changed_surface;
pub use on_player_changed_surface::OnPlayerChangedSurface;
pub mod on_player_changed_position;
pub use on_player_changed_position::OnPlayerChangedPosition;
pub mod on_player_changed_force;
pub use on_player_changed_force::OnPlayerChangedForce;
pub mod on_player_cancelled_crafting;
pub use on_player_cancelled_crafting::OnPlayerCancelledCrafting;
pub mod on_player_built_tile;
pub use on_player_built_tile::OnPlayerBuiltTile;
pub mod on_player_banned;
pub use on_player_banned::OnPlayerBanned;
pub mod on_player_armor_inventory_changed;
pub use on_player_armor_inventory_changed::OnPlayerArmorInventoryChanged;
pub mod on_player_ammo_inventory_changed;
pub use on_player_ammo_inventory_changed::OnPlayerAmmoInventoryChanged;
pub mod on_player_alt_selected_area;
pub use on_player_alt_selected_area::OnPlayerAltSelectedArea;
pub mod on_player_alt_reverse_selected_area;
pub use on_player_alt_reverse_selected_area::OnPlayerAltReverseSelectedArea;
pub mod on_picked_up_item;
pub use on_picked_up_item::OnPickedUpItem;
pub mod on_permission_string_imported;
pub use on_permission_string_imported::OnPermissionStringImported;
pub mod on_permission_group_edited;
pub use on_permission_group_edited::OnPermissionGroupEdited;
pub mod on_permission_group_deleted;
pub use on_permission_group_deleted::OnPermissionGroupDeleted;
pub mod on_permission_group_added;
pub use on_permission_group_added::OnPermissionGroupAdded;
pub mod on_object_destroyed;
pub use on_object_destroyed::OnObjectDestroyed;
pub mod on_mod_item_opened;
pub use on_mod_item_opened::OnModItemOpened;
pub mod on_market_item_purchased;
pub use on_market_item_purchased::OnMarketItemPurchased;
pub mod on_marked_for_upgrade;
pub use on_marked_for_upgrade::OnMarkedForUpgrade;
pub mod on_marked_for_deconstruction;
pub use on_marked_for_deconstruction::OnMarkedForDeconstruction;
pub mod on_lua_shortcut;
pub use on_lua_shortcut::OnLuaShortcut;
pub mod on_land_mine_armed;
pub use on_land_mine_armed::OnLandMineArmed;
pub mod on_gui_value_changed;
pub use on_gui_value_changed::OnGuiValueChanged;
pub mod on_gui_text_changed;
pub use on_gui_text_changed::OnGuiTextChanged;
pub mod on_gui_switch_state_changed;
pub use on_gui_switch_state_changed::OnGuiSwitchStateChanged;
pub mod on_gui_selection_state_changed;
pub use on_gui_selection_state_changed::OnGuiSelectionStateChanged;
pub mod on_gui_selected_tab_changed;
pub use on_gui_selected_tab_changed::OnGuiSelectedTabChanged;
pub mod on_gui_opened;
pub use on_gui_opened::OnGuiOpened;
pub mod on_gui_location_changed;
pub use on_gui_location_changed::OnGuiLocationChanged;
pub mod on_gui_leave;
pub use on_gui_leave::OnGuiLeave;
pub mod on_gui_hover;
pub use on_gui_hover::OnGuiHover;
pub mod on_gui_elem_changed;
pub use on_gui_elem_changed::OnGuiElemChanged;
pub mod on_gui_confirmed;
pub use on_gui_confirmed::OnGuiConfirmed;
pub mod on_gui_closed;
pub use on_gui_closed::OnGuiClosed;
pub mod on_gui_click;
pub use on_gui_click::OnGuiClick;
pub mod on_gui_checked_state_changed;
pub use on_gui_checked_state_changed::OnGuiCheckedStateChanged;
pub mod on_game_created_from_scenario;
pub use on_game_created_from_scenario::OnGameCreatedFromScenario;
pub mod on_forces_merging;
pub use on_forces_merging::OnForcesMerging;
pub mod on_forces_merged;
pub use on_forces_merged::OnForcesMerged;
pub mod on_force_reset;
pub use on_force_reset::OnForceReset;
pub mod on_force_friends_changed;
pub use on_force_friends_changed::OnForceFriendsChanged;
pub mod on_force_created;
pub use on_force_created::OnForceCreated;
pub mod on_force_cease_fire_changed;
pub use on_force_cease_fire_changed::OnForceCeaseFireChanged;
pub mod on_equipment_removed;
pub use on_equipment_removed::OnEquipmentRemoved;
pub mod on_equipment_inserted;
pub use on_equipment_inserted::OnEquipmentInserted;
pub mod on_entity_spawned;
pub use on_entity_spawned::OnEntitySpawned;
pub mod on_entity_settings_pasted;
pub use on_entity_settings_pasted::OnEntitySettingsPasted;
pub mod on_entity_renamed;
pub use on_entity_renamed::OnEntityRenamed;
pub mod on_entity_logistic_slot_changed;
pub use on_entity_logistic_slot_changed::OnEntityLogisticSlotChanged;
pub mod on_entity_died;
pub use on_entity_died::OnEntityDied;
pub mod on_entity_damaged;
pub use on_entity_damaged::OnEntityDamaged;
pub mod on_entity_color_changed;
pub use on_entity_color_changed::OnEntityColorChanged;
pub mod on_entity_cloned;
pub use on_entity_cloned::OnEntityCloned;
pub mod on_cutscene_waypoint_reached;
pub use on_cutscene_waypoint_reached::OnCutsceneWaypointReached;
pub mod on_cutscene_started;
pub use on_cutscene_started::OnCutsceneStarted;
pub mod on_cutscene_finished;
pub use on_cutscene_finished::OnCutsceneFinished;
pub mod on_cutscene_cancelled;
pub use on_cutscene_cancelled::OnCutsceneCancelled;
pub mod on_console_command;
pub use on_console_command::OnConsoleCommand;
pub mod on_console_chat;
pub use on_console_chat::OnConsoleChat;
pub mod on_combat_robot_expired;
pub use on_combat_robot_expired::OnCombatRobotExpired;
pub mod on_chunk_generated;
pub use on_chunk_generated::OnChunkGenerated;
pub mod on_chunk_deleted;
pub use on_chunk_deleted::OnChunkDeleted;
pub mod on_chunk_charted;
pub use on_chunk_charted::OnChunkCharted;
pub mod on_chart_tag_removed;
pub use on_chart_tag_removed::OnChartTagRemoved;
pub mod on_chart_tag_modified;
pub use on_chart_tag_modified::OnChartTagModified;
pub mod on_chart_tag_added;
pub use on_chart_tag_added::OnChartTagAdded;
pub mod on_character_corpse_expired;
pub use on_character_corpse_expired::OnCharacterCorpseExpired;
pub mod on_cargo_pod_finished_ascending;
pub use on_cargo_pod_finished_ascending::OnCargoPodFinishedAscending;
pub mod on_cancelled_upgrade;
pub use on_cancelled_upgrade::OnCancelledUpgrade;
pub mod on_cancelled_deconstruction;
pub use on_cancelled_deconstruction::OnCancelledDeconstruction;
pub mod on_built_entity;
pub use on_built_entity::OnBuiltEntity;
pub mod on_build_base_arrived;
pub use on_build_base_arrived::OnBuildBaseArrived;
pub mod on_brush_cloned;
pub use on_brush_cloned::OnBrushCloned;
pub mod on_biter_base_built;
pub use on_biter_base_built::OnBiterBaseBuilt;
pub mod on_area_cloned;
pub use on_area_cloned::OnAreaCloned;
pub mod on_ai_command_completed;
pub use on_ai_command_completed::OnAiCommandCompleted;
pub mod on_achievement_gained;
pub use on_achievement_gained::OnAchievementGained;
pub mod custom_input_event;
pub use custom_input_event::CustomInputEvent;
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "serde_tag")]
pub enum Events {
    CustomInputEvent(Box<CustomInputEvent>),
    OnAchievementGained(Box<OnAchievementGained>),
    OnAiCommandCompleted(Box<OnAiCommandCompleted>),
    OnAreaCloned(Box<OnAreaCloned>),
    OnBiterBaseBuilt(Box<OnBiterBaseBuilt>),
    OnBrushCloned(Box<OnBrushCloned>),
    OnBuildBaseArrived(Box<OnBuildBaseArrived>),
    OnBuiltEntity(Box<OnBuiltEntity>),
    OnCancelledDeconstruction(Box<OnCancelledDeconstruction>),
    OnCancelledUpgrade(Box<OnCancelledUpgrade>),
    OnCargoPodFinishedAscending(Box<OnCargoPodFinishedAscending>),
    OnCharacterCorpseExpired(Box<OnCharacterCorpseExpired>),
    OnChartTagAdded(Box<OnChartTagAdded>),
    OnChartTagModified(Box<OnChartTagModified>),
    OnChartTagRemoved(Box<OnChartTagRemoved>),
    OnChunkCharted(Box<OnChunkCharted>),
    OnChunkDeleted(Box<OnChunkDeleted>),
    OnChunkGenerated(Box<OnChunkGenerated>),
    OnCombatRobotExpired(Box<OnCombatRobotExpired>),
    OnConsoleChat(Box<OnConsoleChat>),
    OnConsoleCommand(Box<OnConsoleCommand>),
    OnCutsceneCancelled(Box<OnCutsceneCancelled>),
    OnCutsceneFinished(Box<OnCutsceneFinished>),
    OnCutsceneStarted(Box<OnCutsceneStarted>),
    OnCutsceneWaypointReached(Box<OnCutsceneWaypointReached>),
    OnEntityCloned(Box<OnEntityCloned>),
    OnEntityColorChanged(Box<OnEntityColorChanged>),
    OnEntityDamaged(Box<OnEntityDamaged>),
    OnEntityDied(Box<OnEntityDied>),
    OnEntityLogisticSlotChanged(Box<OnEntityLogisticSlotChanged>),
    OnEntityRenamed(Box<OnEntityRenamed>),
    OnEntitySettingsPasted(Box<OnEntitySettingsPasted>),
    OnEntitySpawned(Box<OnEntitySpawned>),
    OnEquipmentInserted(Box<OnEquipmentInserted>),
    OnEquipmentRemoved(Box<OnEquipmentRemoved>),
    OnForceCeaseFireChanged(Box<OnForceCeaseFireChanged>),
    OnForceCreated(Box<OnForceCreated>),
    OnForceFriendsChanged(Box<OnForceFriendsChanged>),
    OnForceReset(Box<OnForceReset>),
    OnForcesMerged(Box<OnForcesMerged>),
    OnForcesMerging(Box<OnForcesMerging>),
    OnGameCreatedFromScenario(Box<OnGameCreatedFromScenario>),
    OnGuiCheckedStateChanged(Box<OnGuiCheckedStateChanged>),
    OnGuiClick(Box<OnGuiClick>),
    OnGuiClosed(Box<OnGuiClosed>),
    OnGuiConfirmed(Box<OnGuiConfirmed>),
    OnGuiElemChanged(Box<OnGuiElemChanged>),
    OnGuiHover(Box<OnGuiHover>),
    OnGuiLeave(Box<OnGuiLeave>),
    OnGuiLocationChanged(Box<OnGuiLocationChanged>),
    OnGuiOpened(Box<OnGuiOpened>),
    OnGuiSelectedTabChanged(Box<OnGuiSelectedTabChanged>),
    OnGuiSelectionStateChanged(Box<OnGuiSelectionStateChanged>),
    OnGuiSwitchStateChanged(Box<OnGuiSwitchStateChanged>),
    OnGuiTextChanged(Box<OnGuiTextChanged>),
    OnGuiValueChanged(Box<OnGuiValueChanged>),
    OnLandMineArmed(Box<OnLandMineArmed>),
    OnLuaShortcut(Box<OnLuaShortcut>),
    OnMarkedForDeconstruction(Box<OnMarkedForDeconstruction>),
    OnMarkedForUpgrade(Box<OnMarkedForUpgrade>),
    OnMarketItemPurchased(Box<OnMarketItemPurchased>),
    OnModItemOpened(Box<OnModItemOpened>),
    OnObjectDestroyed(Box<OnObjectDestroyed>),
    OnPermissionGroupAdded(Box<OnPermissionGroupAdded>),
    OnPermissionGroupDeleted(Box<OnPermissionGroupDeleted>),
    OnPermissionGroupEdited(Box<OnPermissionGroupEdited>),
    OnPermissionStringImported(Box<OnPermissionStringImported>),
    OnPickedUpItem(Box<OnPickedUpItem>),
    OnPlayerAltReverseSelectedArea(Box<OnPlayerAltReverseSelectedArea>),
    OnPlayerAltSelectedArea(Box<OnPlayerAltSelectedArea>),
    OnPlayerAmmoInventoryChanged(Box<OnPlayerAmmoInventoryChanged>),
    OnPlayerArmorInventoryChanged(Box<OnPlayerArmorInventoryChanged>),
    OnPlayerBanned(Box<OnPlayerBanned>),
    OnPlayerBuiltTile(Box<OnPlayerBuiltTile>),
    OnPlayerCancelledCrafting(Box<OnPlayerCancelledCrafting>),
    OnPlayerChangedForce(Box<OnPlayerChangedForce>),
    OnPlayerChangedPosition(Box<OnPlayerChangedPosition>),
    OnPlayerChangedSurface(Box<OnPlayerChangedSurface>),
    OnPlayerCheatModeDisabled(Box<OnPlayerCheatModeDisabled>),
    OnPlayerCheatModeEnabled(Box<OnPlayerCheatModeEnabled>),
    OnPlayerClickedGpsTag(Box<OnPlayerClickedGpsTag>),
    OnPlayerConfiguredBlueprint(Box<OnPlayerConfiguredBlueprint>),
    OnPlayerControllerChanged(Box<OnPlayerControllerChanged>),
    OnPlayerCraftedItem(Box<OnPlayerCraftedItem>),
    OnPlayerCreated(Box<OnPlayerCreated>),
    OnPlayerCursorStackChanged(Box<OnPlayerCursorStackChanged>),
    OnPlayerDeconstructedArea(Box<OnPlayerDeconstructedArea>),
    OnPlayerDemoted(Box<OnPlayerDemoted>),
    OnPlayerDied(Box<OnPlayerDied>),
    OnPlayerDisplayDensityScaleChanged(Box<OnPlayerDisplayDensityScaleChanged>),
    OnPlayerDisplayResolutionChanged(Box<OnPlayerDisplayResolutionChanged>),
    OnPlayerDisplayScaleChanged(Box<OnPlayerDisplayScaleChanged>),
    OnPlayerDrivingChangedState(Box<OnPlayerDrivingChangedState>),
    OnPlayerDroppedItem(Box<OnPlayerDroppedItem>),
    OnPlayerFastTransferred(Box<OnPlayerFastTransferred>),
    OnPlayerFlippedEntity(Box<OnPlayerFlippedEntity>),
    OnPlayerFlushedFluid(Box<OnPlayerFlushedFluid>),
    OnPlayerGunInventoryChanged(Box<OnPlayerGunInventoryChanged>),
    OnPlayerInputMethodChanged(Box<OnPlayerInputMethodChanged>),
    OnPlayerJoinedGame(Box<OnPlayerJoinedGame>),
    OnPlayerKicked(Box<OnPlayerKicked>),
    OnPlayerLeftGame(Box<OnPlayerLeftGame>),
    OnPlayerLocaleChanged(Box<OnPlayerLocaleChanged>),
    OnPlayerMainInventoryChanged(Box<OnPlayerMainInventoryChanged>),
    OnPlayerMinedEntity(Box<OnPlayerMinedEntity>),
    OnPlayerMinedItem(Box<OnPlayerMinedItem>),
    OnPlayerMinedTile(Box<OnPlayerMinedTile>),
    OnPlayerMuted(Box<OnPlayerMuted>),
    OnPlayerPipette(Box<OnPlayerPipette>),
    OnPlayerPlacedEquipment(Box<OnPlayerPlacedEquipment>),
    OnPlayerPromoted(Box<OnPlayerPromoted>),
    OnPlayerRemoved(Box<OnPlayerRemoved>),
    OnPlayerRemovedEquipment(Box<OnPlayerRemovedEquipment>),
    OnPlayerRepairedEntity(Box<OnPlayerRepairedEntity>),
    OnPlayerRespawned(Box<OnPlayerRespawned>),
    OnPlayerReverseSelectedArea(Box<OnPlayerReverseSelectedArea>),
    OnPlayerRotatedEntity(Box<OnPlayerRotatedEntity>),
    OnPlayerSelectedArea(Box<OnPlayerSelectedArea>),
    OnPlayerSetQuickBarSlot(Box<OnPlayerSetQuickBarSlot>),
    OnPlayerSetupBlueprint(Box<OnPlayerSetupBlueprint>),
    OnPlayerToggledAltMode(Box<OnPlayerToggledAltMode>),
    OnPlayerToggledMapEditor(Box<OnPlayerToggledMapEditor>),
    OnPlayerTrashInventoryChanged(Box<OnPlayerTrashInventoryChanged>),
    OnPlayerUnbanned(Box<OnPlayerUnbanned>),
    OnPlayerUnmuted(Box<OnPlayerUnmuted>),
    OnPlayerUsedCapsule(Box<OnPlayerUsedCapsule>),
    OnPlayerUsedSpidertronRemote(Box<OnPlayerUsedSpidertronRemote>),
    OnPostEntityDied(Box<OnPostEntityDied>),
    OnPreBuild(Box<OnPreBuild>),
    OnPreChunkDeleted(Box<OnPreChunkDeleted>),
    OnPreEntitySettingsPasted(Box<OnPreEntitySettingsPasted>),
    OnPreGhostDeconstructed(Box<OnPreGhostDeconstructed>),
    OnPreGhostUpgraded(Box<OnPreGhostUpgraded>),
    OnPrePermissionGroupDeleted(Box<OnPrePermissionGroupDeleted>),
    OnPrePermissionStringImported(Box<OnPrePermissionStringImported>),
    OnPrePlayerCraftedItem(Box<OnPrePlayerCraftedItem>),
    OnPrePlayerDied(Box<OnPrePlayerDied>),
    OnPrePlayerLeftGame(Box<OnPrePlayerLeftGame>),
    OnPrePlayerMinedItem(Box<OnPrePlayerMinedItem>),
    OnPrePlayerRemoved(Box<OnPrePlayerRemoved>),
    OnPrePlayerToggledMapEditor(Box<OnPrePlayerToggledMapEditor>),
    OnPreRobotExplodedCliff(Box<OnPreRobotExplodedCliff>),
    OnPreScenarioFinished(Box<OnPreScenarioFinished>),
    OnPreScriptInventoryResized(Box<OnPreScriptInventoryResized>),
    OnPreSurfaceCleared(Box<OnPreSurfaceCleared>),
    OnPreSurfaceDeleted(Box<OnPreSurfaceDeleted>),
    OnRedoApplied(Box<OnRedoApplied>),
    OnResearchCancelled(Box<OnResearchCancelled>),
    OnResearchFinished(Box<OnResearchFinished>),
    OnResearchMoved(Box<OnResearchMoved>),
    OnResearchReversed(Box<OnResearchReversed>),
    OnResearchStarted(Box<OnResearchStarted>),
    OnResourceDepleted(Box<OnResourceDepleted>),
    OnRobotBuiltEntity(Box<OnRobotBuiltEntity>),
    OnRobotBuiltTile(Box<OnRobotBuiltTile>),
    OnRobotExplodedCliff(Box<OnRobotExplodedCliff>),
    OnRobotMined(Box<OnRobotMined>),
    OnRobotMinedEntity(Box<OnRobotMinedEntity>),
    OnRobotMinedTile(Box<OnRobotMinedTile>),
    OnRobotPreMined(Box<OnRobotPreMined>),
    OnRocketLaunchOrdered(Box<OnRocketLaunchOrdered>),
    OnRocketLaunched(Box<OnRocketLaunched>),
    OnRuntimeModSettingChanged(Box<OnRuntimeModSettingChanged>),
    OnScriptInventoryResized(Box<OnScriptInventoryResized>),
    OnScriptPathRequestFinished(Box<OnScriptPathRequestFinished>),
    OnScriptTriggerEffect(Box<OnScriptTriggerEffect>),
    OnSectorScanned(Box<OnSectorScanned>),
    OnSegmentEntityCreated(Box<OnSegmentEntityCreated>),
    OnSelectedEntityChanged(Box<OnSelectedEntityChanged>),
    OnSpacePlatformBuiltEntity(Box<OnSpacePlatformBuiltEntity>),
    OnSpacePlatformBuiltTile(Box<OnSpacePlatformBuiltTile>),
    OnSpacePlatformChangedState(Box<OnSpacePlatformChangedState>),
    OnSpacePlatformMinedEntity(Box<OnSpacePlatformMinedEntity>),
    OnSpacePlatformMinedItem(Box<OnSpacePlatformMinedItem>),
    OnSpacePlatformMinedTile(Box<OnSpacePlatformMinedTile>),
    OnSpacePlatformPreMined(Box<OnSpacePlatformPreMined>),
    OnSpiderCommandCompleted(Box<OnSpiderCommandCompleted>),
    OnStringTranslated(Box<OnStringTranslated>),
    OnSurfaceCleared(Box<OnSurfaceCleared>),
    OnSurfaceCreated(Box<OnSurfaceCreated>),
    OnSurfaceDeleted(Box<OnSurfaceDeleted>),
    OnSurfaceImported(Box<OnSurfaceImported>),
    OnSurfaceRenamed(Box<OnSurfaceRenamed>),
    OnTechnologyEffectsReset(Box<OnTechnologyEffectsReset>),
    OnTick(Box<OnTick>),
    OnTrainChangedState(Box<OnTrainChangedState>),
    OnTrainCreated(Box<OnTrainCreated>),
    OnTrainScheduleChanged(Box<OnTrainScheduleChanged>),
    OnTriggerCreatedEntity(Box<OnTriggerCreatedEntity>),
    OnTriggerFiredArtillery(Box<OnTriggerFiredArtillery>),
    OnUndoApplied(Box<OnUndoApplied>),
    OnUnitAddedToGroup(Box<OnUnitAddedToGroup>),
    OnUnitGroupCreated(Box<OnUnitGroupCreated>),
    OnUnitGroupFinishedGathering(Box<OnUnitGroupFinishedGathering>),
    OnUnitRemovedFromGroup(Box<OnUnitRemovedFromGroup>),
    OnWorkerRobotExpired(Box<OnWorkerRobotExpired>),
    ScriptRaisedBuilt(Box<ScriptRaisedBuilt>),
    ScriptRaisedDestroy(Box<ScriptRaisedDestroy>),
    ScriptRaisedRevive(Box<ScriptRaisedRevive>),
    ScriptRaisedSetTiles(Box<ScriptRaisedSetTiles>),
    ScriptRaisedTeleported(Box<ScriptRaisedTeleported>),
}
