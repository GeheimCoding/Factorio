pub struct TipsAndTricksItem {
    category: String,
    dependencies: Vec<String>,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    image: crate::types::FileName,
    indent: u8,
    is_title: bool,
    order: crate::types::Order,
    player_input_method_filter: crate::types::PlayerInputMethodFilter,
    simulation: crate::types::SimulationDefinition,
    skip_trigger: crate::types::TipTrigger,
    starting_status: crate::types::TipStatus,
    tag: String,
    trigger: crate::types::TipTrigger,
    tutorial: String,
}