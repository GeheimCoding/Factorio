#[derive(serde::Deserialize)]
pub struct ModuleTransferAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
    limited_to_one_game: bool,
    module: ModuleTransferAchievementPrototypeModule,
}
#[derive(serde::Deserialize)]
pub enum ModuleTransferAchievementPrototypeModule {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(Vec<crate::types::ItemID>),
}
