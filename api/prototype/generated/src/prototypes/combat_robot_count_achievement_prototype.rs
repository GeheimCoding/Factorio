#[derive(Debug, serde::Deserialize)]
pub struct CombatRobotCountAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_count")]
    count: u32,
}
fn default_count() -> u32 {
    1
}
