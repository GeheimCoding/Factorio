pub struct AmmoItemPrototype {
    base_: crate::prototypes::ItemPrototype,
    ammo_category: crate::types::AmmoCategoryID,
    ammo_type: AmmoItemPrototypeAmmoType,
    magazine_size: f32,
    reload_time: f32,
    shoot_protected: bool,
}
pub enum AmmoItemPrototypeAmmoType {
    AmmoType(Box<crate::types::AmmoType>),
    VecAmmoType(Vec<crate::types::AmmoType>),
}
