use api::{LuaRecipePrototype, ProductAmountMax, ProductAmountMin};

#[derive(Clone, Debug)]
pub enum Type {
    Item,
    Fluid,
}

#[derive(Clone, Debug)]
pub struct Ingredient {
    pub name: String,
    pub amount: f64,
    pub type_: Type,
}

#[derive(Clone, Debug)]
pub struct Product {
    pub name: String,
    // If not specified, `amount_min`, `amount_max` and `probability` must all be specified.
    pub amount: Option<f64>,
    pub amount_min: Option<f64>,
    pub amount_max: Option<f64>,
    pub probability: Option<f64>,
    pub type_: Type,
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub name: String,
    pub category: String,
    pub ingredients: Vec<Ingredient>,
    pub energy: f64,
    pub products: Vec<Product>,
}

pub fn map_recipe(recipe: &LuaRecipePrototype) -> Recipe {
    Recipe {
        name: recipe.name.clone(),
        category: recipe.category.clone(),
        ingredients: recipe
            .ingredients
            .iter()
            .map(|ingredient| Ingredient {
                name: ingredient.name.clone(),
                amount: *ingredient.amount.as_value().unwrap(),
                type_: if ingredient.type_ == "item" {
                    Type::Item
                } else {
                    Type::Fluid
                },
            })
            .collect(),
        energy: *recipe.energy.as_value().unwrap(),
        products: recipe
            .products
            .iter()
            .map(|product| Product {
                name: product.name.clone(),
                amount: product
                    .amount
                    .as_ref()
                    .map(|amount| *amount.as_value().unwrap()),
                amount_min: product.amount_min.as_ref().map(|amount| match amount {
                    ProductAmountMin::U32(value) => *value as f64,
                    ProductAmountMin::Double(value) => *value.as_value().unwrap(),
                }),
                amount_max: product.amount_max.as_ref().map(|amount| match amount {
                    ProductAmountMax::U32(value) => *value as f64,
                    ProductAmountMax::Double(value) => *value.as_value().unwrap(),
                }),
                probability: product
                    .probability
                    .as_ref()
                    .map(|amount| *amount.as_value().unwrap()),
                type_: if product.type_ == "item" {
                    Type::Item
                } else {
                    Type::Fluid
                },
            })
            .collect(),
    }
}
