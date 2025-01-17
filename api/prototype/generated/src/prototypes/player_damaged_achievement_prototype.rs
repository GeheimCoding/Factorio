#[derive(serde::Deserialize)]
pub struct PlayerDamagedAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    minimum_damage: f32,
    should_survive: bool,
    type_of_dealer: String,
}
