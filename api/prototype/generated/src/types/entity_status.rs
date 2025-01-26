#[derive(Debug, serde::Deserialize)]
pub enum EntityStatus {
    #[serde(rename = "working")]
    Working,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "ghost")]
    Ghost,
    #[serde(rename = "not_plugged_in_electric_network")]
    NotPluggedInElectricNetwork,
    #[serde(rename = "networks_connected")]
    NetworksConnected,
    #[serde(rename = "networks_disconnected")]
    NetworksDisconnected,
    #[serde(rename = "no_ammo")]
    NoAmmo,
    #[serde(rename = "waiting_for_target_to_be_built")]
    WaitingForTargetToBeBuilt,
    #[serde(rename = "waiting_for_train")]
    WaitingForTrain,
    #[serde(rename = "no_power")]
    NoPower,
    #[serde(rename = "low_temperature")]
    LowTemperature,
    #[serde(rename = "charging")]
    Charging,
    #[serde(rename = "discharging")]
    Discharging,
    #[serde(rename = "fully_charged")]
    FullyCharged,
    #[serde(rename = "no_fuel")]
    NoFuel,
    #[serde(rename = "no_food")]
    NoFood,
    #[serde(rename = "out_of_logistic_network")]
    OutOfLogisticNetwork,
    #[serde(rename = "no_recipe")]
    NoRecipe,
    #[serde(rename = "no_ingredients")]
    NoIngredients,
    #[serde(rename = "no_input_fluid")]
    NoInputFluid,
    #[serde(rename = "no_research_in_progress")]
    NoResearchInProgress,
    #[serde(rename = "no_minable_resources")]
    NoMinableResources,
    #[serde(rename = "low_input_fluid")]
    LowInputFluid,
    #[serde(rename = "low_power")]
    LowPower,
    #[serde(rename = "not_connected_to_rail")]
    NotConnectedToRail,
    #[serde(rename = "cant_divide_segments")]
    CantDivideSegments,
    #[serde(rename = "recharging_after_power_outage")]
    RechargingAfterPowerOutage,
    #[serde(rename = "no_modules_to_transmit")]
    NoModulesToTransmit,
    #[serde(rename = "disabled_by_control_behavior")]
    DisabledByControlBehavior,
    #[serde(rename = "opened_by_circuit_network")]
    OpenedByCircuitNetwork,
    #[serde(rename = "closed_by_circuit_network")]
    ClosedByCircuitNetwork,
    #[serde(rename = "disabled_by_script")]
    DisabledByScript,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "turned_off_during_daytime")]
    TurnedOffDuringDaytime,
    #[serde(rename = "fluid_ingredient_shortage")]
    FluidIngredientShortage,
    #[serde(rename = "item_ingredient_shortage")]
    ItemIngredientShortage,
    #[serde(rename = "full_output")]
    FullOutput,
    #[serde(rename = "not_enough_space_in_output")]
    NotEnoughSpaceInOutput,
    #[serde(rename = "full_burnt_result_output")]
    FullBurntResultOutput,
    #[serde(rename = "marked_for_deconstruction")]
    MarkedForDeconstruction,
    #[serde(rename = "missing_required_fluid")]
    MissingRequiredFluid,
    #[serde(rename = "missing_science_packs")]
    MissingSciencePacks,
    #[serde(rename = "waiting_for_source_items")]
    WaitingForSourceItems,
    #[serde(rename = "waiting_for_space_in_destination")]
    WaitingForSpaceInDestination,
    #[serde(rename = "preparing_rocket_for_launch")]
    PreparingRocketForLaunch,
    #[serde(rename = "waiting_to_launch_rocket")]
    WaitingToLaunchRocket,
    #[serde(rename = "waiting_for_space_in_platform_hub")]
    WaitingForSpaceInPlatformHub,
    #[serde(rename = "launching_rocket")]
    LaunchingRocket,
    #[serde(rename = "thrust_not_required")]
    ThrustNotRequired,
    #[serde(rename = "not_enough_thrust")]
    NotEnoughThrust,
    #[serde(rename = "on_the_way")]
    OnTheWay,
    #[serde(rename = "waiting_in_orbit")]
    WaitingInOrbit,
    #[serde(rename = "waiting_for_rocket_to_arrive")]
    WaitingForRocketToArrive,
    #[serde(rename = "no_path")]
    NoPath,
    #[serde(rename = "broken")]
    Broken,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "frozen")]
    Frozen,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "not_connected_to_hub_or_pad")]
    NotConnectedToHubOrPad,
    #[serde(rename = "computing_navigation")]
    ComputingNavigation,
    #[serde(rename = "no_filter")]
    NoFilter,
    #[serde(rename = "waiting_at_stop")]
    WaitingAtStop,
    #[serde(rename = "destination_stop_full")]
    DestinationStopFull,
    #[serde(rename = "pipeline_overextended")]
    PipelineOverextended,
    #[serde(rename = "no_spot_seedable_by_inputs")]
    NoSpotSeedableByInputs,
    #[serde(rename = "waiting_for_plants_to_grow")]
    WaitingForPlantsToGrow,
    #[serde(rename = "recipe_not_researched")]
    RecipeNotResearched,
}
