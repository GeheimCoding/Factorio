// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum CollisionMaskFlags {
    /// Any two entities that both have this option enabled on their prototype and have an identical collision mask layers list will not collide. Other collision mask options are not included in the identical layer list check. This does mean that two different prototypes with the same collision mask layers and this option enabled will not collide.
    NotCollidingWithItself,
    /// Uses the prototypes position rather than its collision box when doing collision checks with tile prototypes. Allows the prototype to overlap colliding tiles up until its center point. This is only respected for character movement and cars driven by players.
    ConsiderTileTransitions,
    /// Any prototype with this collision option will only be checked for collision with other prototype's collision masks if they are a tile.
    CollidingWithTilesOnly,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum CollisionMaskWithFlagsUnion {
    CollisionMaskLayer(CollisionMaskLayer),
    CollisionMaskFlags(CollisionMaskFlags),
}

// TODO: find a solution with serde?

// ========= MANUAL PATCH =========
// ================================
