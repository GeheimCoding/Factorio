#[derive(Debug, serde::Deserialize)]
pub struct FootstepTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::CreateParticleTriggerEffectItem,
    actions: Option<crate::vec::Vec<crate::types::CreateParticleTriggerEffectItem>>,
    tiles: crate::vec::Vec<crate::types::TileID>,
    #[serde(default = "default_use_as_default")]
    use_as_default: bool,
}
fn default_use_as_default() -> bool {
    false
}
