#[derive(Debug, serde::Deserialize)]
pub struct RecipeCategory {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
}
