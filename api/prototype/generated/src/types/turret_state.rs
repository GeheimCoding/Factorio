#[derive(Debug, serde::Deserialize)]
pub enum TurretState {
    #[serde(rename = "folded")]
    Folded,
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "prepared")]
    Prepared,
    #[serde(rename = "starting_attack")]
    StartingAttack,
    #[serde(rename = "attacking")]
    Attacking,
    #[serde(rename = "ending_attack")]
    EndingAttack,
    #[serde(rename = "rotate_for_folding")]
    RotateForFolding,
    #[serde(rename = "folding")]
    Folding,
}
