#[derive(serde::Deserialize)]
pub struct AutoplaceControl {
    base_: crate::prototypes::Prototype,
    can_be_disabled: bool,
    category: AutoplaceControlCategory,
    richness: bool,
}
#[derive(serde::Deserialize)]
pub enum AutoplaceControlCategory {
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "terrain")]
    Terrain,
    #[serde(rename = "cliff")]
    Cliff,
    #[serde(rename = "enemy")]
    Enemy,
}
