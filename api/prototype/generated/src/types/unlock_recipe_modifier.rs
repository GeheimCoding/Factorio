#[derive(serde::Deserialize)]
pub struct UnlockRecipeModifier {
    base_: crate::types::BaseModifier,
    recipe: crate::types::RecipeID,
    #[serde(rename = "type")]
    type_: String,
    use_icon_overlay_constant: bool,
}
