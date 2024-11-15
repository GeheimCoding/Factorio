pub enum Events {
    OnAchievementGained,
    OnAiCommandCompleted,
    OnAreaCloned,
    OnBiterBaseBuilt,
    OnBrushCloned,
    OnBuildBaseArrived,
    OnBuiltEntity,
    OnCancelledDeconstruction,
    OnCancelledUpgrade,
    OnCharacterCorpseExpired,
    OnChartTagAdded,
    OnChartTagModified,
    OnChartTagRemoved,
    OnChunkCharted,
    OnChunkDeleted,
    OnChunkGenerated,
    OnCombatRobotExpired,
    OnConsoleChat,
    OnConsoleCommand,
    OnCutsceneCancelled,
    OnCutsceneFinished,
    OnCutsceneStarted,
    OnCutsceneWaypointReached,
    OnEntityCloned,
    OnEntityColorChanged,
    OnEntityDamaged,
    OnEntityDied,
    OnEntityLogisticSlotChanged,
    OnEntityRenamed,
    OnEntitySettingsPasted,
    OnEntitySpawned,
    OnEquipmentInserted,
    OnEquipmentRemoved,
    OnForceCeaseFireChanged,
    OnForceCreated,
    OnForceFriendsChanged,
    OnForceReset,
    OnForcesMerged,
    OnForcesMerging,
    OnGameCreatedFromScenario,
    OnGuiCheckedStateChanged,
    OnGuiClick,
    OnGuiClosed,
    OnGuiConfirmed,
    OnGuiElemChanged,
    OnGuiHover,
    OnGuiLeave,
    OnGuiLocationChanged,
    OnGuiOpened,
    OnGuiSelectedTabChanged,
    OnGuiSelectionStateChanged,
    OnGuiSwitchStateChanged,
    OnGuiTextChanged,
    OnGuiValueChanged,
    OnLandMineArmed,
    OnLuaShortcut,
    OnMarkedForDeconstruction,
    OnMarkedForUpgrade,
    OnMarketItemPurchased,
    OnModItemOpened,
    OnObjectDestroyed,
    OnPermissionGroupAdded,
    OnPermissionGroupDeleted,
    OnPermissionGroupEdited,
    OnPermissionStringImported,
    OnPickedUpItem,
    OnPlayerAltReverseSelectedArea,
    OnPlayerAltSelectedArea,
    OnPlayerAmmoInventoryChanged,
    OnPlayerArmorInventoryChanged,
    OnPlayerBanned,
    OnPlayerBuiltTile,
    OnPlayerCancelledCrafting,
    OnPlayerChangedForce,
    OnPlayerChangedPosition,
    OnPlayerChangedSurface,
    OnPlayerCheatModeDisabled,
    OnPlayerCheatModeEnabled,
    OnPlayerClickedGpsTag,
    OnPlayerConfiguredBlueprint,
    OnPlayerControllerChanged,
    OnPlayerCraftedItem,
    OnPlayerCreated,
    OnPlayerCursorStackChanged,
    OnPlayerDeconstructedArea,
    OnPlayerDemoted,
    OnPlayerDied,
    OnPlayerDisplayDensityScaleChanged,
    OnPlayerDisplayResolutionChanged,
    OnPlayerDisplayScaleChanged,
    OnPlayerDrivingChangedState,
    OnPlayerDroppedItem,
    OnPlayerFastTransferred,
    OnPlayerFlippedEntity,
    OnPlayerFlushedFluid,
    OnPlayerGunInventoryChanged,
    OnPlayerInputMethodChanged,
    OnPlayerJoinedGame,
    OnPlayerKicked,
    OnPlayerLeftGame,
    OnPlayerLocaleChanged,
    OnPlayerMainInventoryChanged,
    OnPlayerMinedEntity,
    OnPlayerMinedItem,
    OnPlayerMinedTile,
    OnPlayerMuted,
    OnPlayerPipette,
    OnPlayerPlacedEquipment,
    OnPlayerPromoted,
    OnPlayerRemoved,
    OnPlayerRemovedEquipment,
    OnPlayerRepairedEntity,
    OnPlayerRespawned,
    OnPlayerReverseSelectedArea,
    OnPlayerRotatedEntity,
    OnPlayerSelectedArea,
    OnPlayerSetQuickBarSlot,
    OnPlayerSetupBlueprint,
    OnPlayerToggledAltMode,
    OnPlayerToggledMapEditor,
    OnPlayerTrashInventoryChanged,
    OnPlayerUnbanned,
    OnPlayerUnmuted,
    OnPlayerUsedCapsule,
    OnPlayerUsedSpidertronRemote,
    OnPostEntityDied,
    OnPreBuild,
    OnPreChunkDeleted,
    OnPreEntitySettingsPasted,
    OnPreGhostDeconstructed,
    OnPreGhostUpgraded,
    OnPrePermissionGroupDeleted,
    OnPrePermissionStringImported,
    OnPrePlayerCraftedItem,
    OnPrePlayerDied,
    OnPrePlayerLeftGame,
    OnPrePlayerMinedItem,
    OnPrePlayerRemoved,
    OnPrePlayerToggledMapEditor,
    OnPreRobotExplodedCliff,
    OnPreScenarioFinished,
    OnPreScriptInventoryResized,
    OnPreSurfaceCleared,
    OnPreSurfaceDeleted,
    OnRedoApplied,
    OnResearchCancelled,
    OnResearchFinished,
    OnResearchMoved,
    OnResearchReversed,
    OnResearchStarted,
    OnResourceDepleted,
    OnRobotBuiltEntity,
    OnRobotBuiltTile,
    OnRobotExplodedCliff,
    OnRobotMined,
    OnRobotMinedEntity,
    OnRobotMinedTile,
    OnRobotPreMined,
    OnRocketLaunchOrdered,
    OnRocketLaunched,
    OnRuntimeModSettingChanged,
    OnScriptInventoryResized,
    OnScriptPathRequestFinished,
    OnScriptTriggerEffect,
    OnSectorScanned,
    OnSegmentEntityCreated,
    OnSelectedEntityChanged,
    OnSpacePlatformBuiltEntity,
    OnSpacePlatformBuiltTile,
    OnSpacePlatformChangedState,
    OnSpacePlatformMinedEntity,
    OnSpacePlatformMinedItem,
    OnSpacePlatformMinedTile,
    OnSpacePlatformPreMined,
    OnSpiderCommandCompleted,
    OnStringTranslated,
    OnSurfaceCleared,
    OnSurfaceCreated,
    OnSurfaceDeleted,
    OnSurfaceImported,
    OnSurfaceRenamed,
    OnTechnologyEffectsReset,
    OnTick,
    OnTrainChangedState,
    OnTrainCreated,
    OnTrainScheduleChanged,
    OnTriggerCreatedEntity,
    OnTriggerFiredArtillery,
    OnUndoApplied,
    OnUnitAddedToGroup,
    OnUnitGroupCreated,
    OnUnitGroupFinishedGathering,
    OnUnitRemovedFromGroup,
    OnWorkerRobotExpired,
    ScriptRaisedBuilt,
    ScriptRaisedDestroy,
    ScriptRaisedRevive,
    ScriptRaisedSetTiles,
    ScriptRaisedTeleported,
}
