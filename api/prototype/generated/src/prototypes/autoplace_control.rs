pub struct AutoplaceControl {
    base_: crate::prototypes::Prototype,
    can_be_disabled: bool,
    category: AutoplaceControlCategory,
    richness: bool,
}
pub enum AutoplaceControlCategory {
    Resource,
    Terrain,
    Cliff,
    Enemy,
}
