pub struct SetRecipeTipTrigger {
    any_quality: bool,
    consecutive: bool,
    machine: EntityID,
    recipe: RecipeID,
    type_: SetRecipe,
    uses_fluid: bool,
}
