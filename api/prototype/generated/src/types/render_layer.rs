#[derive(Debug, serde::Deserialize)]
pub enum RenderLayer {
    #[serde(rename = "zero")]
    Zero,
    #[serde(rename = "background-transitions")]
    BackgroundTransitions,
    #[serde(rename = "under-tiles")]
    UnderTiles,
    #[serde(rename = "decals")]
    Decals,
    #[serde(rename = "above-tiles")]
    AboveTiles,
    #[serde(rename = "ground-layer-1")]
    GroundLayer1,
    #[serde(rename = "ground-layer-2")]
    GroundLayer2,
    #[serde(rename = "ground-layer-3")]
    GroundLayer3,
    #[serde(rename = "ground-layer-4")]
    GroundLayer4,
    #[serde(rename = "ground-layer-5")]
    GroundLayer5,
    #[serde(rename = "lower-radius-visualization")]
    LowerRadiusVisualization,
    #[serde(rename = "radius-visualization")]
    RadiusVisualization,
    #[serde(rename = "transport-belt-integration")]
    TransportBeltIntegration,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "building-smoke")]
    BuildingSmoke,
    #[serde(rename = "rail-stone-path-lower")]
    RailStonePathLower,
    #[serde(rename = "rail-stone-path")]
    RailStonePath,
    #[serde(rename = "rail-tie")]
    RailTie,
    #[serde(rename = "decorative")]
    Decorative,
    #[serde(rename = "ground-patch")]
    GroundPatch,
    #[serde(rename = "ground-patch-higher")]
    GroundPatchHigher,
    #[serde(rename = "ground-patch-higher2")]
    GroundPatchHigher2,
    #[serde(rename = "rail-chain-signal-metal")]
    RailChainSignalMetal,
    #[serde(rename = "rail-screw")]
    RailScrew,
    #[serde(rename = "rail-metal")]
    RailMetal,
    #[serde(rename = "remnants")]
    Remnants,
    #[serde(rename = "floor")]
    Floor,
    #[serde(rename = "transport-belt")]
    TransportBelt,
    #[serde(rename = "transport-belt-endings")]
    TransportBeltEndings,
    #[serde(rename = "floor-mechanics-under-corpse")]
    FloorMechanicsUnderCorpse,
    #[serde(rename = "corpse")]
    Corpse,
    #[serde(rename = "floor-mechanics")]
    FloorMechanics,
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "transport-belt-reader")]
    TransportBeltReader,
    #[serde(rename = "lower-object")]
    LowerObject,
    #[serde(rename = "transport-belt-circuit-connector")]
    TransportBeltCircuitConnector,
    #[serde(rename = "lower-object-above-shadow")]
    LowerObjectAboveShadow,
    #[serde(rename = "lower-object-overlay")]
    LowerObjectOverlay,
    #[serde(rename = "object-under")]
    ObjectUnder,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "cargo-hatch")]
    CargoHatch,
    #[serde(rename = "higher-object-under")]
    HigherObjectUnder,
    #[serde(rename = "higher-object-above")]
    HigherObjectAbove,
    #[serde(rename = "train-stop-top")]
    TrainStopTop,
    #[serde(rename = "item-in-inserter-hand")]
    ItemInInserterHand,
    #[serde(rename = "above-inserter")]
    AboveInserter,
    #[serde(rename = "wires")]
    Wires,
    #[serde(rename = "under-elevated")]
    UnderElevated,
    #[serde(rename = "elevated-rail-stone-path-lower")]
    ElevatedRailStonePathLower,
    #[serde(rename = "elevated-rail-stone-path")]
    ElevatedRailStonePath,
    #[serde(rename = "elevated-rail-tie")]
    ElevatedRailTie,
    #[serde(rename = "elevated-rail-screw")]
    ElevatedRailScrew,
    #[serde(rename = "elevated-rail-metal")]
    ElevatedRailMetal,
    #[serde(rename = "elevated-lower-object")]
    ElevatedLowerObject,
    #[serde(rename = "elevated-object")]
    ElevatedObject,
    #[serde(rename = "elevated-higher-object")]
    ElevatedHigherObject,
    #[serde(rename = "fluid-visualization")]
    FluidVisualization,
    #[serde(rename = "wires-above")]
    WiresAbove,
    #[serde(rename = "entity-info-icon")]
    EntityInfoIcon,
    #[serde(rename = "entity-info-icon-above")]
    EntityInfoIconAbove,
    #[serde(rename = "explosion")]
    Explosion,
    #[serde(rename = "projectile")]
    Projectile,
    #[serde(rename = "smoke")]
    Smoke,
    #[serde(rename = "air-object")]
    AirObject,
    #[serde(rename = "air-entity-info-icon")]
    AirEntityInfoIcon,
    #[serde(rename = "light-effect")]
    LightEffect,
    #[serde(rename = "selection-box")]
    SelectionBox,
    #[serde(rename = "higher-selection-box")]
    HigherSelectionBox,
    #[serde(rename = "collision-selection-box")]
    CollisionSelectionBox,
    #[serde(rename = "arrow")]
    Arrow,
    #[serde(rename = "cursor")]
    Cursor,
}
