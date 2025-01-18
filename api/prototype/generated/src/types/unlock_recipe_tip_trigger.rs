#[derive(serde::Deserialize)]
pub struct UnlockRecipeTipTrigger {
    recipe: crate::types::RecipeID,
    #[serde(rename = "type")]
    type_: String,
}
