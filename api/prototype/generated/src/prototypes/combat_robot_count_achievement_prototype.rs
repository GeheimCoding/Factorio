#[derive(serde::Deserialize)]
pub struct CombatRobotCountAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    count: u32,
}
