#[derive(serde::Deserialize)]
pub struct SetRecipeTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    any_quality: bool,
    consecutive: bool,
    machine: crate::types::EntityID,
    recipe: crate::types::RecipeID,
    type_: String,
    uses_fluid: bool,
}
