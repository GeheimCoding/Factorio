#[derive(Debug, serde::Deserialize)]
pub struct SpaceConnectionDistanceTraveledAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    distance: u32,
    reversed: bool,
    tracked_connection: crate::types::SpaceConnectionID,
}
