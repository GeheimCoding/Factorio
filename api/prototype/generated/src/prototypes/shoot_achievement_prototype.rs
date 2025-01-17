#[derive(serde::Deserialize)]
pub struct ShootAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    ammo_type: crate::types::ItemID,
    amount: u32,
}
