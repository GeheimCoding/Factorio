#[derive(serde::Deserialize)]
pub struct UnlockRecipeModifier {
    base_: crate::types::BaseModifier,
    recipe: crate::types::RecipeID,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_use_icon_overlay_constant() -> bool {
    false
}
