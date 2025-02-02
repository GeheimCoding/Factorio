#[derive(Debug, serde::Deserialize)]
pub struct PlayerDamagedAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    minimum_damage: f32,
    should_survive: bool,
    #[serde(default = "default_type_of_dealer")]
    type_of_dealer: String,
}
fn default_type_of_dealer() -> String {
    String::from("")
}
