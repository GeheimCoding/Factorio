#[derive(serde::Deserialize)]
pub enum CursorBoxType {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "multiplayer_entity")]
    MultiplayerEntity,
    #[serde(rename = "electricity")]
    Electricity,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "not_allowed")]
    NotAllowed,
    #[serde(rename = "pair")]
    Pair,
    #[serde(rename = "logistics")]
    Logistics,
    #[serde(rename = "train_visualization")]
    TrainVisualization,
    #[serde(rename = "blueprint_snap_rectangle")]
    BlueprintSnapRectangle,
    #[serde(rename = "spidertron_remote_selected")]
    SpidertronRemoteSelected,
    #[serde(rename = "spidertron_remote_to_be_selected")]
    SpidertronRemoteToBeSelected,
}
