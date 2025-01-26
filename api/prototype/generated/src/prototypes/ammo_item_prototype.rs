#[derive(Debug, serde::Deserialize)]
pub struct AmmoItemPrototype {
    base_: crate::prototypes::ItemPrototype,
    ammo_category: crate::types::AmmoCategoryID,
    ammo_type: AmmoItemPrototypeAmmoType,
    #[serde(default = "default_magazine_size")]
    magazine_size: f32,
    #[serde(default = "default_reload_time")]
    reload_time: f32,
    #[serde(default = "default_shoot_protected")]
    shoot_protected: bool,
}
#[derive(Debug, serde::Deserialize)]
pub enum AmmoItemPrototypeAmmoType {
    #[serde(untagged)]
    AmmoType(Box<crate::types::AmmoType>),
    #[serde(untagged)]
    VecAmmoType(Vec<crate::types::AmmoType>),
}
fn default_magazine_size() -> f32 {
    1.0
}
fn default_reload_time() -> f32 {
    0.0
}
fn default_shoot_protected() -> bool {
    false
}
