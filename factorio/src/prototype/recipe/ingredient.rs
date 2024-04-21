use crate::prototype::recipe::map_type;
use crate::prototype::Type;
use api::{IngredientPrototype, ItemIngredientPrototype};

#[derive(Clone, Debug, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub amount: f64,
    pub type_: Type,
}

impl From<&IngredientPrototype> for Ingredient {
    fn from(ingredient: &IngredientPrototype) -> Self {
        match ingredient {
            IngredientPrototype::ItemIngredientPrototype(item) => match item {
                ItemIngredientPrototype::ItemIngredientPrototype {
                    name,
                    amount,
                    type_,
                    ..
                } => Self {
                    name: name.clone(),
                    amount: *amount as f64,
                    type_: map_type(type_),
                },
                ItemIngredientPrototype::ItemIDU16((name, amount)) => Self {
                    name: name.clone(),
                    amount: *amount as f64,
                    type_: Type::Item,
                },
            },
            IngredientPrototype::FluidIngredientPrototype(fluid) => Self {
                name: fluid.name.clone(),
                amount: *fluid.amount.as_value().unwrap(),
                type_: Type::Fluid,
            },
        }
    }
}
