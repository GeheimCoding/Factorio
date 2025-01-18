#[derive(serde::Deserialize)]
pub struct ShootAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    ammo_type: crate::types::ItemID,
    #[serde(default = "default_amount")]
    amount: u32,
}
fn default_amount() -> u32 {
    1
}
