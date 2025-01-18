#[derive(serde::Deserialize)]
pub struct SetRecipeTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_any_quality")]
    any_quality: bool,
    #[serde(default = "default_consecutive")]
    consecutive: bool,
    machine: crate::types::EntityID,
    recipe: crate::types::RecipeID,
    #[serde(rename = "type")]
    type_: String,
    // default: any
    uses_fluid: bool,
}
fn default_any_quality() -> bool {
    false
}
fn default_consecutive() -> bool {
    false
}
