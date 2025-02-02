#[derive(Debug, serde::Deserialize)]
pub struct DeliverByRobotsAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
}
