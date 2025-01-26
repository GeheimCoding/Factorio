#[derive(Debug, serde::Deserialize)]
pub enum RenderLayer {
    #[serde(rename = "zero")]
    Zero,
    #[serde(rename = "background_transitions")]
    BackgroundTransitions,
    #[serde(rename = "under_tiles")]
    UnderTiles,
    #[serde(rename = "decals")]
    Decals,
    #[serde(rename = "above_tiles")]
    AboveTiles,
    #[serde(rename = "ground_layer_1")]
    GroundLayer1,
    #[serde(rename = "ground_layer_2")]
    GroundLayer2,
    #[serde(rename = "ground_layer_3")]
    GroundLayer3,
    #[serde(rename = "ground_layer_4")]
    GroundLayer4,
    #[serde(rename = "ground_layer_5")]
    GroundLayer5,
    #[serde(rename = "lower_radius_visualization")]
    LowerRadiusVisualization,
    #[serde(rename = "radius_visualization")]
    RadiusVisualization,
    #[serde(rename = "transport_belt_integration")]
    TransportBeltIntegration,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "building_smoke")]
    BuildingSmoke,
    #[serde(rename = "rail_stone_path_lower")]
    RailStonePathLower,
    #[serde(rename = "rail_stone_path")]
    RailStonePath,
    #[serde(rename = "rail_tie")]
    RailTie,
    #[serde(rename = "decorative")]
    Decorative,
    #[serde(rename = "ground_patch")]
    GroundPatch,
    #[serde(rename = "ground_patch_higher")]
    GroundPatchHigher,
    #[serde(rename = "ground_patch_higher_2")]
    GroundPatchHigher2,
    #[serde(rename = "rail_chain_signal_metal")]
    RailChainSignalMetal,
    #[serde(rename = "rail_screw")]
    RailScrew,
    #[serde(rename = "rail_metal")]
    RailMetal,
    #[serde(rename = "remnants")]
    Remnants,
    #[serde(rename = "floor")]
    Floor,
    #[serde(rename = "transport_belt")]
    TransportBelt,
    #[serde(rename = "transport_belt_endings")]
    TransportBeltEndings,
    #[serde(rename = "floor_mechanics_under_corpse")]
    FloorMechanicsUnderCorpse,
    #[serde(rename = "corpse")]
    Corpse,
    #[serde(rename = "floor_mechanics")]
    FloorMechanics,
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "transport_belt_reader")]
    TransportBeltReader,
    #[serde(rename = "lower_object")]
    LowerObject,
    #[serde(rename = "transport_belt_circuit_connector")]
    TransportBeltCircuitConnector,
    #[serde(rename = "lower_object_above_shadow")]
    LowerObjectAboveShadow,
    #[serde(rename = "lower_object_overlay")]
    LowerObjectOverlay,
    #[serde(rename = "object_under")]
    ObjectUnder,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "cargo_hatch")]
    CargoHatch,
    #[serde(rename = "higher_object_under")]
    HigherObjectUnder,
    #[serde(rename = "higher_object_above")]
    HigherObjectAbove,
    #[serde(rename = "train_stop_top")]
    TrainStopTop,
    #[serde(rename = "item_in_inserter_hand")]
    ItemInInserterHand,
    #[serde(rename = "above_inserter")]
    AboveInserter,
    #[serde(rename = "wires")]
    Wires,
    #[serde(rename = "under_elevated")]
    UnderElevated,
    #[serde(rename = "elevated_rail_stone_path_lower")]
    ElevatedRailStonePathLower,
    #[serde(rename = "elevated_rail_stone_path")]
    ElevatedRailStonePath,
    #[serde(rename = "elevated_rail_tie")]
    ElevatedRailTie,
    #[serde(rename = "elevated_rail_screw")]
    ElevatedRailScrew,
    #[serde(rename = "elevated_rail_metal")]
    ElevatedRailMetal,
    #[serde(rename = "elevated_lower_object")]
    ElevatedLowerObject,
    #[serde(rename = "elevated_object")]
    ElevatedObject,
    #[serde(rename = "elevated_higher_object")]
    ElevatedHigherObject,
    #[serde(rename = "fluid_visualization")]
    FluidVisualization,
    #[serde(rename = "wires_above")]
    WiresAbove,
    #[serde(rename = "entity_info_icon")]
    EntityInfoIcon,
    #[serde(rename = "entity_info_icon_above")]
    EntityInfoIconAbove,
    #[serde(rename = "explosion")]
    Explosion,
    #[serde(rename = "projectile")]
    Projectile,
    #[serde(rename = "smoke")]
    Smoke,
    #[serde(rename = "air_object")]
    AirObject,
    #[serde(rename = "air_entity_info_icon")]
    AirEntityInfoIcon,
    #[serde(rename = "light_effect")]
    LightEffect,
    #[serde(rename = "selection_box")]
    SelectionBox,
    #[serde(rename = "higher_selection_box")]
    HigherSelectionBox,
    #[serde(rename = "collision_selection_box")]
    CollisionSelectionBox,
    #[serde(rename = "arrow")]
    Arrow,
    #[serde(rename = "cursor")]
    Cursor,
}
