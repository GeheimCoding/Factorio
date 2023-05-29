// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderLayerVariants {
    WaterTile = 15,
    GroundTile = 25,
    TileTransition = 26,
    Decals = 27,
    LowerRadiusVisualization = 29,
    RadiusVisualization = 30,
    TransportBeltIntegration = 65,
    Resource = 66,
    BuildingSmoke = 67,
    Decorative = 92,
    GroundPatch = 93,
    GroundPatchHigher = 94,
    GroundPatchHigher2 = 95,
    Remnants = 112,
    Floor = 113,
    TransportBelt = 114,
    TransportBeltEndings = 115,
    FloorMechanicsUnderCorpse = 120,
    Corpse = 121,
    FloorMechanics = 122,
    Item = 123,
    LowerObject = 124,
    TransportBeltCircuitConnector = 126,
    LowerObjectAboveShadow = 127,
    Object = 129,
    HigherObjectUnder = 131,
    HigherObjectAbove = 132,
    ItemInInserterHand = 134,
    Wires = 135,
    WiresAbove = 136,
    EntityInfoIcon = 138,
    EntityInfoIconAbove = 139,
    Explosion = 142,
    Projectile = 143,
    Smoke = 144,
    AirObject = 145,
    AirEntityInfoIcon = 147,
    LightEffect = 148,
    SelectionBox = 187,
    HigherSelectionBox = 188,
    CollisionSelectionBox = 189,
    Arrow = 190,
    Cursor = 210,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RenderLayer {
    Value(u8),
    Variant(RenderLayerVariants),
}

// ========= MANUAL PATCH =========
// ================================
