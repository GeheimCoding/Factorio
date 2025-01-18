#[derive(serde::Deserialize)]
pub struct TileTransitionsBetweenTransitions {
    base_: crate::types::TileTransitions,
    #[serde(rename = "transition_group1")]
    transition_group_1: u8,
    #[serde(rename = "transition_group2")]
    transition_group_2: u8,
}
