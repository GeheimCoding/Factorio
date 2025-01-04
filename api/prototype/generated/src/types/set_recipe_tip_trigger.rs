pub struct SetRecipeTipTrigger {
    any_quality: bool,
    consecutive: bool,
    machine: crate::types::EntityID,
    recipe: crate::types::RecipeID,
    type_: SetRecipe,
    uses_fluid: bool,
}
