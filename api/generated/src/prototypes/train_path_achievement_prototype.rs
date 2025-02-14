#[derive(Debug, serde::Deserialize)]
pub struct TrainPathAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    minimum_distance: f64,
}
