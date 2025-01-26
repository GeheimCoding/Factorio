#[derive(Debug, serde::Deserialize)]
pub struct ModuleTransferAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
    #[serde(default = "default_limited_to_one_game")]
    limited_to_one_game: bool,
    module: ModuleTransferAchievementPrototypeModule,
}
fn default_amount() -> u32 {
    1
}
fn default_limited_to_one_game() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum ModuleTransferAchievementPrototypeModule {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(Vec<crate::types::ItemID>),
}
