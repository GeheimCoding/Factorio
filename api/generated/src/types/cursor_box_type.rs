#[derive(Debug, serde::Deserialize)]
pub enum CursorBoxType {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "multiplayer-entity")]
    MultiplayerEntity,
    #[serde(rename = "electricity")]
    Electricity,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "not-allowed")]
    NotAllowed,
    #[serde(rename = "pair")]
    Pair,
    #[serde(rename = "logistics")]
    Logistics,
    #[serde(rename = "train-visualization")]
    TrainVisualization,
    #[serde(rename = "blueprint-snap-rectangle")]
    BlueprintSnapRectangle,
    #[serde(rename = "spidertron-remote-selected")]
    SpidertronRemoteSelected,
    #[serde(rename = "spidertron-remote-to-be-selected")]
    SpidertronRemoteToBeSelected,
}
