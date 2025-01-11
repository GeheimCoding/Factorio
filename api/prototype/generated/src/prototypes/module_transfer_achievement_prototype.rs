pub struct ModuleTransferAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
    limited_to_one_game: bool,
    module: ModuleTransferAchievementPrototypeModule,
}
pub enum ModuleTransferAchievementPrototypeModule {
    ItemID(crate::types::ItemID),
    VecItemID(Vec<crate::types::ItemID>),
}
