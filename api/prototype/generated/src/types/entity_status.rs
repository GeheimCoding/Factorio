pub enum EntityStatus {
    Working,
    Normal,
    Ghost,
    NotPluggedInElectricNetwork,
    NetworksConnected,
    NetworksDisconnected,
    NoAmmo,
    WaitingForTargetToBeBuilt,
    WaitingForTrain,
    NoPower,
    LowTemperature,
    Charging,
    Discharging,
    FullyCharged,
    NoFuel,
    NoFood,
    OutOfLogisticNetwork,
    NoRecipe,
    NoIngredients,
    NoInputFluid,
    NoResearchInProgress,
    NoMinableResources,
    LowInputFluid,
    LowPower,
    NotConnectedToRail,
    CantDivideSegments,
    RechargingAfterPowerOutage,
    NoModulesToTransmit,
    DisabledByControlBehavior,
    OpenedByCircuitNetwork,
    ClosedByCircuitNetwork,
    DisabledByScript,
    Disabled,
    TurnedOffDuringDaytime,
    FluidIngredientShortage,
    ItemIngredientShortage,
    FullOutput,
    NotEnoughSpaceInOutput,
    FullBurntResultOutput,
    MarkedForDeconstruction,
    MissingRequiredFluid,
    MissingSciencePacks,
    WaitingForSourceItems,
    WaitingForSpaceInDestination,
    PreparingRocketForLaunch,
    WaitingToLaunchRocket,
    WaitingForSpaceInPlatformHub,
    LaunchingRocket,
    ThrustNotRequired,
    NotEnoughThrust,
    OnTheWay,
    WaitingInOrbit,
    WaitingForRocketToArrive,
    NoPath,
    Broken,
    None,
    Frozen,
    Paused,
    NotConnectedToHubOrPad,
    ComputingNavigation,
    NoFilter,
    WaitingAtStop,
    DestinationStopFull,
    PipelineOverextended,
    NoSpotSeedableByInputs,
    WaitingForPlantsToGrow,
    RecipeNotResearched,
}