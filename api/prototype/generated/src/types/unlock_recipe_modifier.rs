#[derive(Debug, serde::Deserialize)]
pub struct UnlockRecipeModifier {
    base_: crate::types::BaseModifier,
    recipe: crate::types::RecipeID,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_use_icon_overlay_constant() -> bool {
    false
}
