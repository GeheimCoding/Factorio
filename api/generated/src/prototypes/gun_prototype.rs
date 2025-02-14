#[derive(Debug, serde::Deserialize)]
pub struct GunPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ItemPrototype,
    attack_parameters: crate::types::AttackParameters,
}
