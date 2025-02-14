#[derive(Debug, serde::Deserialize)]
pub struct UnlockRecipeTipTrigger {
    recipe: crate::types::RecipeID,
}
