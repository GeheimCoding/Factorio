#[derive(serde::Deserialize)]
pub struct UnlockRecipeTipTrigger {
    recipe: crate::types::RecipeID,
    type_: String,
}
