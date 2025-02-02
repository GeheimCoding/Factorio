#[derive(Debug, serde::Deserialize)]
pub struct DeconstructWithRobotsAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
}
