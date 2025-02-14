#[derive(Debug, serde::Deserialize)]
pub struct ShootAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    ammo_type: Option<crate::types::ItemID>,
    #[serde(default = "default_amount")]
    amount: u32,
}
fn default_amount() -> u32 {
    1
}
