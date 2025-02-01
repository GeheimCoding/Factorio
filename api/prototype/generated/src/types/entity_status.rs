#[derive(Debug, serde::Deserialize)]
pub enum EntityStatus {
    #[serde(rename = "working")]
    Working,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "ghost")]
    Ghost,
    #[serde(rename = "not-plugged-in-electric-network")]
    NotPluggedInElectricNetwork,
    #[serde(rename = "networks-connected")]
    NetworksConnected,
    #[serde(rename = "networks-disconnected")]
    NetworksDisconnected,
    #[serde(rename = "no-ammo")]
    NoAmmo,
    #[serde(rename = "waiting-for-target-to-be-built")]
    WaitingForTargetToBeBuilt,
    #[serde(rename = "waiting-for-train")]
    WaitingForTrain,
    #[serde(rename = "no-power")]
    NoPower,
    #[serde(rename = "low-temperature")]
    LowTemperature,
    #[serde(rename = "charging")]
    Charging,
    #[serde(rename = "discharging")]
    Discharging,
    #[serde(rename = "fully-charged")]
    FullyCharged,
    #[serde(rename = "no-fuel")]
    NoFuel,
    #[serde(rename = "no-food")]
    NoFood,
    #[serde(rename = "out-of-logistic-network")]
    OutOfLogisticNetwork,
    #[serde(rename = "no-recipe")]
    NoRecipe,
    #[serde(rename = "no-ingredients")]
    NoIngredients,
    #[serde(rename = "no-input-fluid")]
    NoInputFluid,
    #[serde(rename = "no-research-in-progress")]
    NoResearchInProgress,
    #[serde(rename = "no-minable-resources")]
    NoMinableResources,
    #[serde(rename = "low-input-fluid")]
    LowInputFluid,
    #[serde(rename = "low-power")]
    LowPower,
    #[serde(rename = "not-connected-to-rail")]
    NotConnectedToRail,
    #[serde(rename = "cant-divide-segments")]
    CantDivideSegments,
    #[serde(rename = "recharging-after-power-outage")]
    RechargingAfterPowerOutage,
    #[serde(rename = "no-modules-to-transmit")]
    NoModulesToTransmit,
    #[serde(rename = "disabled-by-control-behavior")]
    DisabledByControlBehavior,
    #[serde(rename = "opened-by-circuit-network")]
    OpenedByCircuitNetwork,
    #[serde(rename = "closed-by-circuit-network")]
    ClosedByCircuitNetwork,
    #[serde(rename = "disabled-by-script")]
    DisabledByScript,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "turned-off-during-daytime")]
    TurnedOffDuringDaytime,
    #[serde(rename = "fluid-ingredient-shortage")]
    FluidIngredientShortage,
    #[serde(rename = "item-ingredient-shortage")]
    ItemIngredientShortage,
    #[serde(rename = "full-output")]
    FullOutput,
    #[serde(rename = "not-enough-space-in-output")]
    NotEnoughSpaceInOutput,
    #[serde(rename = "full-burnt-result-output")]
    FullBurntResultOutput,
    #[serde(rename = "marked-for-deconstruction")]
    MarkedForDeconstruction,
    #[serde(rename = "missing-required-fluid")]
    MissingRequiredFluid,
    #[serde(rename = "missing-science-packs")]
    MissingSciencePacks,
    #[serde(rename = "waiting-for-source-items")]
    WaitingForSourceItems,
    #[serde(rename = "waiting-for-space-in-destination")]
    WaitingForSpaceInDestination,
    #[serde(rename = "preparing-rocket-for-launch")]
    PreparingRocketForLaunch,
    #[serde(rename = "waiting-to-launch-rocket")]
    WaitingToLaunchRocket,
    #[serde(rename = "waiting-for-space-in-platform-hub")]
    WaitingForSpaceInPlatformHub,
    #[serde(rename = "launching-rocket")]
    LaunchingRocket,
    #[serde(rename = "thrust-not-required")]
    ThrustNotRequired,
    #[serde(rename = "not-enough-thrust")]
    NotEnoughThrust,
    #[serde(rename = "on-the-way")]
    OnTheWay,
    #[serde(rename = "waiting-in-orbit")]
    WaitingInOrbit,
    #[serde(rename = "waiting-for-rocket-to-arrive")]
    WaitingForRocketToArrive,
    #[serde(rename = "no-path")]
    NoPath,
    #[serde(rename = "broken")]
    Broken,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "frozen")]
    Frozen,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "not-connected-to-hub-or-pad")]
    NotConnectedToHubOrPad,
    #[serde(rename = "computing-navigation")]
    ComputingNavigation,
    #[serde(rename = "no-filter")]
    NoFilter,
    #[serde(rename = "waiting-at-stop")]
    WaitingAtStop,
    #[serde(rename = "destination-stop-full")]
    DestinationStopFull,
    #[serde(rename = "pipeline-overextended")]
    PipelineOverextended,
    #[serde(rename = "no-spot-seedable-by-inputs")]
    NoSpotSeedableByInputs,
    #[serde(rename = "waiting-for-plants-to-grow")]
    WaitingForPlantsToGrow,
    #[serde(rename = "recipe-not-researched")]
    RecipeNotResearched,
}
