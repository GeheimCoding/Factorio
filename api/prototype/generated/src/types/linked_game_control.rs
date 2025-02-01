#[derive(Debug, serde::Deserialize)]
pub enum LinkedGameControl {
    #[serde(rename = "move-up")]
    MoveUp,
    #[serde(rename = "move-down")]
    MoveDown,
    #[serde(rename = "move-left")]
    MoveLeft,
    #[serde(rename = "move-right")]
    MoveRight,
    #[serde(rename = "open-character-gui")]
    OpenCharacterGui,
    #[serde(rename = "open-gui")]
    OpenGui,
    #[serde(rename = "confirm-gui")]
    ConfirmGui,
    #[serde(rename = "toggle-free-cursor")]
    ToggleFreeCursor,
    #[serde(rename = "mine")]
    Mine,
    #[serde(rename = "build")]
    Build,
    #[serde(rename = "build-ghost")]
    BuildGhost,
    #[serde(rename = "super-forced-build")]
    SuperForcedBuild,
    #[serde(rename = "clear-cursor")]
    ClearCursor,
    #[serde(rename = "pipette")]
    Pipette,
    #[serde(rename = "rotate")]
    Rotate,
    #[serde(rename = "reverse-rotate")]
    ReverseRotate,
    #[serde(rename = "flip-horizontal")]
    FlipHorizontal,
    #[serde(rename = "flip-vertical")]
    FlipVertical,
    #[serde(rename = "pick-items")]
    PickItems,
    #[serde(rename = "drop-cursor")]
    DropCursor,
    #[serde(rename = "show-info")]
    ShowInfo,
    #[serde(rename = "shoot-enemy")]
    ShootEnemy,
    #[serde(rename = "shoot-selected")]
    ShootSelected,
    #[serde(rename = "next-weapon")]
    NextWeapon,
    #[serde(rename = "toggle-driving")]
    ToggleDriving,
    #[serde(rename = "zoom-in")]
    ZoomIn,
    #[serde(rename = "zoom-out")]
    ZoomOut,
    #[serde(rename = "use-item")]
    UseItem,
    #[serde(rename = "alternative-use-item")]
    AlternativeUseItem,
    #[serde(rename = "toggle-console")]
    ToggleConsole,
    #[serde(rename = "copy-entity-settings")]
    CopyEntitySettings,
    #[serde(rename = "paste-entity-settings")]
    PasteEntitySettings,
    #[serde(rename = "controller-gui-logistics-tab")]
    ControllerGuiLogisticsTab,
    #[serde(rename = "controller-gui-character-tab")]
    ControllerGuiCharacterTab,
    #[serde(rename = "controller-gui-crafting-tab")]
    ControllerGuiCraftingTab,
    #[serde(rename = "toggle-rail-layer")]
    ToggleRailLayer,
    #[serde(rename = "select-for-blueprint")]
    SelectForBlueprint,
    #[serde(rename = "select-for-cancel-deconstruct")]
    SelectForCancelDeconstruct,
    #[serde(rename = "select-for-super-forced-deconstruct")]
    SelectForSuperForcedDeconstruct,
    #[serde(rename = "reverse-select")]
    ReverseSelect,
    #[serde(rename = "alt-reverse-select")]
    AltReverseSelect,
    #[serde(rename = "deselect")]
    Deselect,
    #[serde(rename = "cycle-blueprint-forwards")]
    CycleBlueprintForwards,
    #[serde(rename = "cycle-blueprint-backwards")]
    CycleBlueprintBackwards,
    #[serde(rename = "focus-search")]
    FocusSearch,
    #[serde(rename = "larger-terrain-building-area")]
    LargerTerrainBuildingArea,
    #[serde(rename = "smaller-terrain-building-area")]
    SmallerTerrainBuildingArea,
    #[serde(rename = "remove-pole-cables")]
    RemovePoleCables,
    #[serde(rename = "build-with-obstacle-avoidance")]
    BuildWithObstacleAvoidance,
    #[serde(rename = "add-station")]
    AddStation,
    #[serde(rename = "add-temporary-station")]
    AddTemporaryStation,
    #[serde(rename = "rename-all")]
    RenameAll,
    #[serde(rename = "fast-wait-condition")]
    FastWaitCondition,
    #[serde(rename = "drag-map")]
    DragMap,
    #[serde(rename = "move-tag")]
    MoveTag,
    #[serde(rename = "place-in-chat")]
    PlaceInChat,
    #[serde(rename = "place-ping")]
    PlacePing,
    #[serde(rename = "pin")]
    Pin,
    #[serde(rename = "activate-tooltip")]
    ActivateTooltip,
    #[serde(rename = "next-surface")]
    NextSurface,
    #[serde(rename = "previous-surface")]
    PreviousSurface,
    #[serde(rename = "cycle-quality-up")]
    CycleQualityUp,
    #[serde(rename = "cycle-quality-down")]
    CycleQualityDown,
    #[serde(rename = "craft")]
    Craft,
    #[serde(rename = "craft-5")]
    Craft5,
    #[serde(rename = "craft-all")]
    CraftAll,
    #[serde(rename = "cancel-craft")]
    CancelCraft,
    #[serde(rename = "cancel-craft-5")]
    CancelCraft5,
    #[serde(rename = "cancel-craft-all")]
    CancelCraftAll,
    #[serde(rename = "pick-item")]
    PickItem,
    #[serde(rename = "stack-transfer")]
    StackTransfer,
    #[serde(rename = "inventory-transfer")]
    InventoryTransfer,
    #[serde(rename = "fast-entity-transfer")]
    FastEntityTransfer,
    #[serde(rename = "cursor-split")]
    CursorSplit,
    #[serde(rename = "stack-split")]
    StackSplit,
    #[serde(rename = "inventory-split")]
    InventorySplit,
    #[serde(rename = "fast-entity-split")]
    FastEntitySplit,
    #[serde(rename = "toggle-filter")]
    ToggleFilter,
    #[serde(rename = "open-item")]
    OpenItem,
    #[serde(rename = "copy-inventory-filter")]
    CopyInventoryFilter,
    #[serde(rename = "paste-inventory-filter")]
    PasteInventoryFilter,
    #[serde(rename = "show-quick-panel")]
    ShowQuickPanel,
    #[serde(rename = "next-quick-panel-page")]
    NextQuickPanelPage,
    #[serde(rename = "previous-quick-panel-page")]
    PreviousQuickPanelPage,
    #[serde(rename = "next-quick-panel-tab")]
    NextQuickPanelTab,
    #[serde(rename = "previous-quick-panel-tab")]
    PreviousQuickPanelTab,
    #[serde(rename = "rotate-active-quick-bars")]
    RotateActiveQuickBars,
    #[serde(rename = "next-active-quick-bar")]
    NextActiveQuickBar,
    #[serde(rename = "previous-active-quick-bar")]
    PreviousActiveQuickBar,
    #[serde(rename = "quick-bar-button-1")]
    QuickBarButton1,
    #[serde(rename = "quick-bar-button-2")]
    QuickBarButton2,
    #[serde(rename = "quick-bar-button-3")]
    QuickBarButton3,
    #[serde(rename = "quick-bar-button-4")]
    QuickBarButton4,
    #[serde(rename = "quick-bar-button-5")]
    QuickBarButton5,
    #[serde(rename = "quick-bar-button-6")]
    QuickBarButton6,
    #[serde(rename = "quick-bar-button-7")]
    QuickBarButton7,
    #[serde(rename = "quick-bar-button-8")]
    QuickBarButton8,
    #[serde(rename = "quick-bar-button-9")]
    QuickBarButton9,
    #[serde(rename = "quick-bar-button-10")]
    QuickBarButton10,
    #[serde(rename = "quick-bar-button-1-secondary")]
    QuickBarButton1Secondary,
    #[serde(rename = "quick-bar-button-2-secondary")]
    QuickBarButton2Secondary,
    #[serde(rename = "quick-bar-button-3-secondary")]
    QuickBarButton3Secondary,
    #[serde(rename = "quick-bar-button-4-secondary")]
    QuickBarButton4Secondary,
    #[serde(rename = "quick-bar-button-5-secondary")]
    QuickBarButton5Secondary,
    #[serde(rename = "quick-bar-button-6-secondary")]
    QuickBarButton6Secondary,
    #[serde(rename = "quick-bar-button-7-secondary")]
    QuickBarButton7Secondary,
    #[serde(rename = "quick-bar-button-8-secondary")]
    QuickBarButton8Secondary,
    #[serde(rename = "quick-bar-button-9-secondary")]
    QuickBarButton9Secondary,
    #[serde(rename = "quick-bar-button-10-secondary")]
    QuickBarButton10Secondary,
    #[serde(rename = "action-bar-select-page-1")]
    ActionBarSelectPage1,
    #[serde(rename = "action-bar-select-page-2")]
    ActionBarSelectPage2,
    #[serde(rename = "action-bar-select-page-3")]
    ActionBarSelectPage3,
    #[serde(rename = "action-bar-select-page-4")]
    ActionBarSelectPage4,
    #[serde(rename = "action-bar-select-page-5")]
    ActionBarSelectPage5,
    #[serde(rename = "action-bar-select-page-6")]
    ActionBarSelectPage6,
    #[serde(rename = "action-bar-select-page-7")]
    ActionBarSelectPage7,
    #[serde(rename = "action-bar-select-page-8")]
    ActionBarSelectPage8,
    #[serde(rename = "action-bar-select-page-9")]
    ActionBarSelectPage9,
    #[serde(rename = "action-bar-select-page-10")]
    ActionBarSelectPage10,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "cut")]
    Cut,
    #[serde(rename = "paste")]
    Paste,
    #[serde(rename = "cycle-clipboard-forwards")]
    CycleClipboardForwards,
    #[serde(rename = "cycle-clipboard-backwards")]
    CycleClipboardBackwards,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "redo")]
    Redo,
    #[serde(rename = "toggle-menu")]
    ToggleMenu,
    #[serde(rename = "toggle-map")]
    ToggleMap,
    #[serde(rename = "close-menu")]
    CloseMenu,
    #[serde(rename = "open-technology-gui")]
    OpenTechnologyGui,
    #[serde(rename = "production-statistics")]
    ProductionStatistics,
    #[serde(rename = "logistic-networks")]
    LogisticNetworks,
    #[serde(rename = "toggle-blueprint-library")]
    ToggleBlueprintLibrary,
    #[serde(rename = "open-trains-gui")]
    OpenTrainsGui,
    #[serde(rename = "open-factoriopedia")]
    OpenFactoriopedia,
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "pause-game")]
    PauseGame,
    #[serde(rename = "confirm-message")]
    ConfirmMessage,
    #[serde(rename = "previous-technology")]
    PreviousTechnology,
    #[serde(rename = "previous-mod")]
    PreviousMod,
    #[serde(rename = "connect-train")]
    ConnectTrain,
    #[serde(rename = "disconnect-train")]
    DisconnectTrain,
    #[serde(rename = "submit-feedback")]
    SubmitFeedback,
    #[serde(rename = "editor-next-variation")]
    EditorNextVariation,
    #[serde(rename = "editor-previous-variation")]
    EditorPreviousVariation,
    #[serde(rename = "editor-clone-item")]
    EditorCloneItem,
    #[serde(rename = "editor-delete-item")]
    EditorDeleteItem,
    #[serde(rename = "editor-toggle-pause")]
    EditorTogglePause,
    #[serde(rename = "editor-tick-once")]
    EditorTickOnce,
    #[serde(rename = "editor-speed-up")]
    EditorSpeedUp,
    #[serde(rename = "editor-speed-down")]
    EditorSpeedDown,
    #[serde(rename = "editor-reset-speed")]
    EditorResetSpeed,
    #[serde(rename = "editor-set-clone-brush-source")]
    EditorSetCloneBrushSource,
    #[serde(rename = "editor-set-clone-brush-destination")]
    EditorSetCloneBrushDestination,
    #[serde(rename = "editor-switch-to-surface")]
    EditorSwitchToSurface,
    #[serde(rename = "editor-remove-scripting-object")]
    EditorRemoveScriptingObject,
    #[serde(rename = "debug-toggle-atlas-gui")]
    DebugToggleAtlasGui,
    #[serde(rename = "debug-toggle-gui-visibility")]
    DebugToggleGuiVisibility,
    #[serde(rename = "debug-toggle-debug-settings")]
    DebugToggleDebugSettings,
    #[serde(rename = "debug-toggle-basic")]
    DebugToggleBasic,
    #[serde(rename = "debug-reset-zoom")]
    DebugResetZoom,
    #[serde(rename = "debug-reset-zoom-2x")]
    DebugResetZoom2x,
    #[serde(rename = "toggle-gui-debug")]
    ToggleGuiDebug,
    #[serde(rename = "toggle-gui-style-view")]
    ToggleGuiStyleView,
    #[serde(rename = "toggle-gui-shadows")]
    ToggleGuiShadows,
    #[serde(rename = "toggle-gui-glows")]
    ToggleGuiGlows,
    #[serde(rename = "open-prototypes-gui")]
    OpenPrototypesGui,
    #[serde(rename = "open-prototype-explorer-gui")]
    OpenPrototypeExplorerGui,
    #[serde(rename = "increase-ui-scale")]
    IncreaseUiScale,
    #[serde(rename = "decrease-ui-scale")]
    DecreaseUiScale,
    #[serde(rename = "reset-ui-scale")]
    ResetUiScale,
    #[serde(rename = "slash-editor")]
    SlashEditor,
    #[serde(rename = "toggle-entity")]
    ToggleEntity,
    #[serde(rename = "next-player-in-replay")]
    NextPlayerInReplay,
    #[serde(rename = "move-blueprint-absolute-grid-up")]
    MoveBlueprintAbsoluteGridUp,
    #[serde(rename = "move-blueprint-absolute-grid-down")]
    MoveBlueprintAbsoluteGridDown,
    #[serde(rename = "move-blueprint-absolute-grid-left")]
    MoveBlueprintAbsoluteGridLeft,
    #[serde(rename = "move-blueprint-absolute-grid-right")]
    MoveBlueprintAbsoluteGridRight,
    #[serde(rename = "move-blueprint-entities-up")]
    MoveBlueprintEntitiesUp,
    #[serde(rename = "move-blueprint-entities-down")]
    MoveBlueprintEntitiesDown,
    #[serde(rename = "move-blueprint-entities-left")]
    MoveBlueprintEntitiesLeft,
    #[serde(rename = "move-blueprint-entities-right")]
    MoveBlueprintEntitiesRight,
    #[serde(rename = "play-next-track")]
    PlayNextTrack,
    #[serde(rename = "play-previous-track")]
    PlayPreviousTrack,
    #[serde(rename = "pause-resume-music")]
    PauseResumeMusic,
}
