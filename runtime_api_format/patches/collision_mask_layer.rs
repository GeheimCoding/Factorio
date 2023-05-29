// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum CollisionMaskLayerVariants {
    GroundTile,
    WaterTile,
    ResourceLayer,
    DoodadLayer,
    FloorLayer,
    ItemLayer,
    GhostLayer,
    ObjectLayer,
    PlayerLayer,
    TrainLayer,
    RailLayer,
    TransportBeltLayer,
    NotSetup,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum CollisionMaskLayer {
    Variant(CollisionMaskLayerVariants),
    String(String),
}

// TODO: find a solution with serde?

// ========= MANUAL PATCH =========
// ================================
