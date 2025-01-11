pub struct AutoplaceControl {
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
