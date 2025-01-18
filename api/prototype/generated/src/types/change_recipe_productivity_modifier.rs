#[derive(serde::Deserialize)]
pub struct ChangeRecipeProductivityModifier {
    base_: crate::types::BaseModifier,
    change: crate::types::EffectValue,
    recipe: crate::types::RecipeID,
    #[serde(rename = "type")]
    type_: String,
    use_icon_overlay_constant: bool,
}
