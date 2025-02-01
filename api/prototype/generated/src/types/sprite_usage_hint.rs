#[derive(Debug, serde::Deserialize)]
pub enum SpriteUsageHint {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "mining")]
    Mining,
    #[serde(rename = "tile-artifical")]
    TileArtifical,
    #[serde(rename = "corpse-decay")]
    CorpseDecay,
    #[serde(rename = "enemy")]
    Enemy,
    #[serde(rename = "player")]
    Player,
    #[serde(rename = "train")]
    Train,
    #[serde(rename = "vehicle")]
    Vehicle,
    #[serde(rename = "explosion")]
    Explosion,
    #[serde(rename = "rail")]
    Rail,
    #[serde(rename = "elevated-rail")]
    ElevatedRail,
    #[serde(rename = "air")]
    Air,
    #[serde(rename = "remnant")]
    Remnant,
    #[serde(rename = "decorative")]
    Decorative,
}
