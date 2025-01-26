#[derive(Debug, serde::Deserialize)]
pub struct TipsAndTricksItem {
    base_: crate::prototypes::PrototypeBase,
    // default: the `name` of this prototype
    category: Option<String>,
    // default: none
    dependencies: Option<Vec<String>>,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<Vec<crate::types::IconData>>,
    #[serde(default = "default_image")]
    image: crate::types::FileName,
    #[serde(default = "default_indent")]
    indent: u8,
    #[serde(default = "default_is_title")]
    is_title: bool,
    // default: Value of `name`
    order: Option<crate::types::Order>,
    #[serde(default = "default_player_input_method_filter")]
    player_input_method_filter: crate::types::PlayerInputMethodFilter,
    simulation: Option<crate::types::SimulationDefinition>,
    skip_trigger: Option<crate::types::TipTrigger>,
    #[serde(default = "default_starting_status")]
    starting_status: crate::types::TipStatus,
    #[serde(default = "default_tag")]
    tag: String,
    trigger: Option<crate::types::TipTrigger>,
    #[serde(default = "default_tutorial")]
    tutorial: String,
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_image() -> crate::types::FileName {
    String::from("")
}
fn default_indent() -> u8 {
    0
}
fn default_is_title() -> bool {
    false
}
fn default_player_input_method_filter() -> crate::types::PlayerInputMethodFilter {
    crate::types::PlayerInputMethodFilter::All
}
fn default_starting_status() -> crate::types::TipStatus {
    crate::types::TipStatus::Locked
}
fn default_tag() -> String {
    String::from("")
}
fn default_tutorial() -> String {
    String::from("")
}
