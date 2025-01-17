#[derive(serde::Deserialize)]
pub struct AmmoTurretPrototype {
    base_: crate::prototypes::TurretPrototype,
    automated_ammo_count: crate::types::ItemCountType,
    energy_per_shot: crate::types::Energy,
    energy_source: crate::types::ElectricEnergySource,
    inventory_size: crate::types::ItemStackIndex,
    prepare_with_no_ammo: bool,
}
