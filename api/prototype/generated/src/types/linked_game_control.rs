#[derive(Debug, serde::Deserialize)]
pub enum LinkedGameControl {
    #[serde(rename = "move_up")]
    MoveUp,
    #[serde(rename = "move_down")]
    MoveDown,
    #[serde(rename = "move_left")]
    MoveLeft,
    #[serde(rename = "move_right")]
    MoveRight,
    #[serde(rename = "open_character_gui")]
    OpenCharacterGui,
    #[serde(rename = "open_gui")]
    OpenGui,
    #[serde(rename = "confirm_gui")]
    ConfirmGui,
    #[serde(rename = "toggle_free_cursor")]
    ToggleFreeCursor,
    #[serde(rename = "mine")]
    Mine,
    #[serde(rename = "build")]
    Build,
    #[serde(rename = "build_ghost")]
    BuildGhost,
    #[serde(rename = "super_forced_build")]
    SuperForcedBuild,
    #[serde(rename = "clear_cursor")]
    ClearCursor,
    #[serde(rename = "pipette")]
    Pipette,
    #[serde(rename = "rotate")]
    Rotate,
    #[serde(rename = "reverse_rotate")]
    ReverseRotate,
    #[serde(rename = "flip_horizontal")]
    FlipHorizontal,
    #[serde(rename = "flip_vertical")]
    FlipVertical,
    #[serde(rename = "pick_items")]
    PickItems,
    #[serde(rename = "drop_cursor")]
    DropCursor,
    #[serde(rename = "show_info")]
    ShowInfo,
    #[serde(rename = "shoot_enemy")]
    ShootEnemy,
    #[serde(rename = "shoot_selected")]
    ShootSelected,
    #[serde(rename = "next_weapon")]
    NextWeapon,
    #[serde(rename = "toggle_driving")]
    ToggleDriving,
    #[serde(rename = "zoom_in")]
    ZoomIn,
    #[serde(rename = "zoom_out")]
    ZoomOut,
    #[serde(rename = "use_item")]
    UseItem,
    #[serde(rename = "alternative_use_item")]
    AlternativeUseItem,
    #[serde(rename = "toggle_console")]
    ToggleConsole,
    #[serde(rename = "copy_entity_settings")]
    CopyEntitySettings,
    #[serde(rename = "paste_entity_settings")]
    PasteEntitySettings,
    #[serde(rename = "controller_gui_logistics_tab")]
    ControllerGuiLogisticsTab,
    #[serde(rename = "controller_gui_character_tab")]
    ControllerGuiCharacterTab,
    #[serde(rename = "controller_gui_crafting_tab")]
    ControllerGuiCraftingTab,
    #[serde(rename = "toggle_rail_layer")]
    ToggleRailLayer,
    #[serde(rename = "select_for_blueprint")]
    SelectForBlueprint,
    #[serde(rename = "select_for_cancel_deconstruct")]
    SelectForCancelDeconstruct,
    #[serde(rename = "select_for_super_forced_deconstruct")]
    SelectForSuperForcedDeconstruct,
    #[serde(rename = "reverse_select")]
    ReverseSelect,
    #[serde(rename = "alt_reverse_select")]
    AltReverseSelect,
    #[serde(rename = "deselect")]
    Deselect,
    #[serde(rename = "cycle_blueprint_forwards")]
    CycleBlueprintForwards,
    #[serde(rename = "cycle_blueprint_backwards")]
    CycleBlueprintBackwards,
    #[serde(rename = "focus_search")]
    FocusSearch,
    #[serde(rename = "larger_terrain_building_area")]
    LargerTerrainBuildingArea,
    #[serde(rename = "smaller_terrain_building_area")]
    SmallerTerrainBuildingArea,
    #[serde(rename = "remove_pole_cables")]
    RemovePoleCables,
    #[serde(rename = "build_with_obstacle_avoidance")]
    BuildWithObstacleAvoidance,
    #[serde(rename = "add_station")]
    AddStation,
    #[serde(rename = "add_temporary_station")]
    AddTemporaryStation,
    #[serde(rename = "rename_all")]
    RenameAll,
    #[serde(rename = "fast_wait_condition")]
    FastWaitCondition,
    #[serde(rename = "drag_map")]
    DragMap,
    #[serde(rename = "move_tag")]
    MoveTag,
    #[serde(rename = "place_in_chat")]
    PlaceInChat,
    #[serde(rename = "place_ping")]
    PlacePing,
    #[serde(rename = "pin")]
    Pin,
    #[serde(rename = "activate_tooltip")]
    ActivateTooltip,
    #[serde(rename = "next_surface")]
    NextSurface,
    #[serde(rename = "previous_surface")]
    PreviousSurface,
    #[serde(rename = "cycle_quality_up")]
    CycleQualityUp,
    #[serde(rename = "cycle_quality_down")]
    CycleQualityDown,
    #[serde(rename = "craft")]
    Craft,
    #[serde(rename = "craft_5")]
    Craft5,
    #[serde(rename = "craft_all")]
    CraftAll,
    #[serde(rename = "cancel_craft")]
    CancelCraft,
    #[serde(rename = "cancel_craft_5")]
    CancelCraft5,
    #[serde(rename = "cancel_craft_all")]
    CancelCraftAll,
    #[serde(rename = "pick_item")]
    PickItem,
    #[serde(rename = "stack_transfer")]
    StackTransfer,
    #[serde(rename = "inventory_transfer")]
    InventoryTransfer,
    #[serde(rename = "fast_entity_transfer")]
    FastEntityTransfer,
    #[serde(rename = "cursor_split")]
    CursorSplit,
    #[serde(rename = "stack_split")]
    StackSplit,
    #[serde(rename = "inventory_split")]
    InventorySplit,
    #[serde(rename = "fast_entity_split")]
    FastEntitySplit,
    #[serde(rename = "toggle_filter")]
    ToggleFilter,
    #[serde(rename = "open_item")]
    OpenItem,
    #[serde(rename = "copy_inventory_filter")]
    CopyInventoryFilter,
    #[serde(rename = "paste_inventory_filter")]
    PasteInventoryFilter,
    #[serde(rename = "show_quick_panel")]
    ShowQuickPanel,
    #[serde(rename = "next_quick_panel_page")]
    NextQuickPanelPage,
    #[serde(rename = "previous_quick_panel_page")]
    PreviousQuickPanelPage,
    #[serde(rename = "next_quick_panel_tab")]
    NextQuickPanelTab,
    #[serde(rename = "previous_quick_panel_tab")]
    PreviousQuickPanelTab,
    #[serde(rename = "rotate_active_quick_bars")]
    RotateActiveQuickBars,
    #[serde(rename = "next_active_quick_bar")]
    NextActiveQuickBar,
    #[serde(rename = "previous_active_quick_bar")]
    PreviousActiveQuickBar,
    #[serde(rename = "quick_bar_button_1")]
    QuickBarButton1,
    #[serde(rename = "quick_bar_button_2")]
    QuickBarButton2,
    #[serde(rename = "quick_bar_button_3")]
    QuickBarButton3,
    #[serde(rename = "quick_bar_button_4")]
    QuickBarButton4,
    #[serde(rename = "quick_bar_button_5")]
    QuickBarButton5,
    #[serde(rename = "quick_bar_button_6")]
    QuickBarButton6,
    #[serde(rename = "quick_bar_button_7")]
    QuickBarButton7,
    #[serde(rename = "quick_bar_button_8")]
    QuickBarButton8,
    #[serde(rename = "quick_bar_button_9")]
    QuickBarButton9,
    #[serde(rename = "quick_bar_button_10")]
    QuickBarButton10,
    #[serde(rename = "quick_bar_button_1secondary")]
    QuickBarButton1Secondary,
    #[serde(rename = "quick_bar_button_2secondary")]
    QuickBarButton2Secondary,
    #[serde(rename = "quick_bar_button_3secondary")]
    QuickBarButton3Secondary,
    #[serde(rename = "quick_bar_button_4secondary")]
    QuickBarButton4Secondary,
    #[serde(rename = "quick_bar_button_5secondary")]
    QuickBarButton5Secondary,
    #[serde(rename = "quick_bar_button_6secondary")]
    QuickBarButton6Secondary,
    #[serde(rename = "quick_bar_button_7secondary")]
    QuickBarButton7Secondary,
    #[serde(rename = "quick_bar_button_8secondary")]
    QuickBarButton8Secondary,
    #[serde(rename = "quick_bar_button_9secondary")]
    QuickBarButton9Secondary,
    #[serde(rename = "quick_bar_button_10secondary")]
    QuickBarButton10Secondary,
    #[serde(rename = "action_bar_select_page_1")]
    ActionBarSelectPage1,
    #[serde(rename = "action_bar_select_page_2")]
    ActionBarSelectPage2,
    #[serde(rename = "action_bar_select_page_3")]
    ActionBarSelectPage3,
    #[serde(rename = "action_bar_select_page_4")]
    ActionBarSelectPage4,
    #[serde(rename = "action_bar_select_page_5")]
    ActionBarSelectPage5,
    #[serde(rename = "action_bar_select_page_6")]
    ActionBarSelectPage6,
    #[serde(rename = "action_bar_select_page_7")]
    ActionBarSelectPage7,
    #[serde(rename = "action_bar_select_page_8")]
    ActionBarSelectPage8,
    #[serde(rename = "action_bar_select_page_9")]
    ActionBarSelectPage9,
    #[serde(rename = "action_bar_select_page_10")]
    ActionBarSelectPage10,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "cut")]
    Cut,
    #[serde(rename = "paste")]
    Paste,
    #[serde(rename = "cycle_clipboard_forwards")]
    CycleClipboardForwards,
    #[serde(rename = "cycle_clipboard_backwards")]
    CycleClipboardBackwards,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "redo")]
    Redo,
    #[serde(rename = "toggle_menu")]
    ToggleMenu,
    #[serde(rename = "toggle_map")]
    ToggleMap,
    #[serde(rename = "close_menu")]
    CloseMenu,
    #[serde(rename = "open_technology_gui")]
    OpenTechnologyGui,
    #[serde(rename = "production_statistics")]
    ProductionStatistics,
    #[serde(rename = "logistic_networks")]
    LogisticNetworks,
    #[serde(rename = "toggle_blueprint_library")]
    ToggleBlueprintLibrary,
    #[serde(rename = "open_trains_gui")]
    OpenTrainsGui,
    #[serde(rename = "open_factoriopedia")]
    OpenFactoriopedia,
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "pause_game")]
    PauseGame,
    #[serde(rename = "confirm_message")]
    ConfirmMessage,
    #[serde(rename = "previous_technology")]
    PreviousTechnology,
    #[serde(rename = "previous_mod")]
    PreviousMod,
    #[serde(rename = "connect_train")]
    ConnectTrain,
    #[serde(rename = "disconnect_train")]
    DisconnectTrain,
    #[serde(rename = "submit_feedback")]
    SubmitFeedback,
    #[serde(rename = "editor_next_variation")]
    EditorNextVariation,
    #[serde(rename = "editor_previous_variation")]
    EditorPreviousVariation,
    #[serde(rename = "editor_clone_item")]
    EditorCloneItem,
    #[serde(rename = "editor_delete_item")]
    EditorDeleteItem,
    #[serde(rename = "editor_toggle_pause")]
    EditorTogglePause,
    #[serde(rename = "editor_tick_once")]
    EditorTickOnce,
    #[serde(rename = "editor_speed_up")]
    EditorSpeedUp,
    #[serde(rename = "editor_speed_down")]
    EditorSpeedDown,
    #[serde(rename = "editor_reset_speed")]
    EditorResetSpeed,
    #[serde(rename = "editor_set_clone_brush_source")]
    EditorSetCloneBrushSource,
    #[serde(rename = "editor_set_clone_brush_destination")]
    EditorSetCloneBrushDestination,
    #[serde(rename = "editor_switch_to_surface")]
    EditorSwitchToSurface,
    #[serde(rename = "editor_remove_scripting_object")]
    EditorRemoveScriptingObject,
    #[serde(rename = "debug_toggle_atlas_gui")]
    DebugToggleAtlasGui,
    #[serde(rename = "debug_toggle_gui_visibility")]
    DebugToggleGuiVisibility,
    #[serde(rename = "debug_toggle_debug_settings")]
    DebugToggleDebugSettings,
    #[serde(rename = "debug_toggle_basic")]
    DebugToggleBasic,
    #[serde(rename = "debug_reset_zoom")]
    DebugResetZoom,
    #[serde(rename = "debug_reset_zoom_2x")]
    DebugResetZoom2x,
    #[serde(rename = "toggle_gui_debug")]
    ToggleGuiDebug,
    #[serde(rename = "toggle_gui_style_view")]
    ToggleGuiStyleView,
    #[serde(rename = "toggle_gui_shadows")]
    ToggleGuiShadows,
    #[serde(rename = "toggle_gui_glows")]
    ToggleGuiGlows,
    #[serde(rename = "open_prototypes_gui")]
    OpenPrototypesGui,
    #[serde(rename = "open_prototype_explorer_gui")]
    OpenPrototypeExplorerGui,
    #[serde(rename = "increase_ui_scale")]
    IncreaseUiScale,
    #[serde(rename = "decrease_ui_scale")]
    DecreaseUiScale,
    #[serde(rename = "reset_ui_scale")]
    ResetUiScale,
    #[serde(rename = "slash_editor")]
    SlashEditor,
    #[serde(rename = "toggle_entity")]
    ToggleEntity,
    #[serde(rename = "next_player_in_replay")]
    NextPlayerInReplay,
    #[serde(rename = "move_blueprint_absolute_grid_up")]
    MoveBlueprintAbsoluteGridUp,
    #[serde(rename = "move_blueprint_absolute_grid_down")]
    MoveBlueprintAbsoluteGridDown,
    #[serde(rename = "move_blueprint_absolute_grid_left")]
    MoveBlueprintAbsoluteGridLeft,
    #[serde(rename = "move_blueprint_absolute_grid_right")]
    MoveBlueprintAbsoluteGridRight,
    #[serde(rename = "move_blueprint_entities_up")]
    MoveBlueprintEntitiesUp,
    #[serde(rename = "move_blueprint_entities_down")]
    MoveBlueprintEntitiesDown,
    #[serde(rename = "move_blueprint_entities_left")]
    MoveBlueprintEntitiesLeft,
    #[serde(rename = "move_blueprint_entities_right")]
    MoveBlueprintEntitiesRight,
    #[serde(rename = "play_next_track")]
    PlayNextTrack,
    #[serde(rename = "play_previous_track")]
    PlayPreviousTrack,
    #[serde(rename = "pause_resume_music")]
    PauseResumeMusic,
}
