#[derive(Debug, serde::Deserialize)]
pub struct GunPrototype {
    base_: crate::prototypes::ItemPrototype,
    attack_parameters: crate::types::AttackParameters,
}
