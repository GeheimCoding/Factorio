use api::VehiclePrototypeBrakingPower::Double;
use api::{
    FloatingPoint, IngredientPrototype, ItemIngredientPrototype, ItemProductPrototype,
    LuaRecipePrototype, ProductAmountMax, ProductAmountMin, ProductPrototype, RecipeData,
    RecipePrototype, RecipePrototypeNormal,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Item,
    Fluid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub amount: f64,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub name: String,
    pub amount: f64,
    pub probability: f64,
    pub type_: Type,
}

// https://wiki.factorio.com/Materials_and_recipes
#[derive(Clone, Debug, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub category: String,
    pub ingredients: HashMap<String, Ingredient>,
    pub energy: f64,
    pub products: HashMap<String, Product>,
}

pub fn map_recipe(recipe: &RecipePrototype) -> Recipe {
    let recipe_data =
        if let Some(RecipePrototypeNormal::RecipeData(recipe_data)) = recipe.normal.clone() {
            recipe_data
        } else {
            map_recipe_data(recipe)
        };
    let products = if let Some(results) = &recipe_data.results {
        results
            .iter()
            .map(map_product)
            .map(|product| (product.name.clone(), product))
            .collect()
    } else {
        let product = map_product(&ProductPrototype::ItemProductPrototype(
            ItemProductPrototype::ItemIDU16((
                recipe_data.result.clone().unwrap(),
                recipe_data.result_count.unwrap_or(1),
            )),
        ));
        HashMap::from([(product.name.clone(), product)])
    };
    Recipe {
        name: recipe.parent_.name.clone(),
        category: recipe.category.clone().unwrap_or("crafting".to_owned()),
        ingredients: recipe_data
            .ingredients
            .iter()
            .map(map_ingredient)
            .map(|ingredient| (ingredient.name.clone(), ingredient))
            .collect(),
        energy: recipe_data
            .energy_required
            .clone()
            .map_or(0.5, |double| *double.as_value().unwrap()),
        products,
    }
}

fn map_ingredient(ingredient: &IngredientPrototype) -> Ingredient {
    match ingredient {
        IngredientPrototype::ItemIngredientPrototype(item) => match item {
            ItemIngredientPrototype::ItemIngredientPrototype {
                name,
                amount,
                type_,
                ..
            } => Ingredient {
                name: name.clone(),
                amount: *amount as f64,
                type_: map_type(type_),
            },
            ItemIngredientPrototype::ItemIDU16((name, amount)) => Ingredient {
                name: name.clone(),
                amount: *amount as f64,
                type_: Type::Item,
            },
        },
        IngredientPrototype::FluidIngredientPrototype(fluid) => Ingredient {
            name: fluid.name.clone(),
            amount: *fluid.amount.as_value().unwrap(),
            type_: Type::Fluid,
        },
    }
}

fn map_product(product: &ProductPrototype) -> Product {
    match product {
        ProductPrototype::ItemProductPrototype(item) => match item {
            ItemProductPrototype::ItemProductPrototype {
                name,
                amount,
                amount_min,
                amount_max,
                probability,
                type_,
                ..
            } => {
                if amount_min.is_some() || amount_max.is_some() {
                    unimplemented!("only amount is implemented for now");
                }
                Product {
                    name: name.clone(),
                    amount: amount.map_or(1.0, |amount| amount as f64),
                    probability: map_double(probability),
                    type_: map_type(type_),
                }
            }
            ItemProductPrototype::ItemIDU16((name, amount)) => Product {
                name: name.clone(),
                amount: *amount as f64,
                probability: 1.0,
                type_: Type::Item,
            },
        },
        ProductPrototype::FluidProductPrototype(fluid) => Product {
            name: fluid.name.clone(),
            amount: map_double(&fluid.amount),
            probability: map_double(&fluid.probability),
            type_: Type::Fluid,
        },
    }
}

fn map_recipe_data(recipe: &RecipePrototype) -> RecipeData {
    RecipeData {
        allow_as_intermediate: recipe.allow_as_intermediate,
        allow_decomposition: recipe.allow_decomposition,
        allow_inserter_overload: recipe.allow_inserter_overload,
        allow_intermediates: recipe.allow_intermediates,
        always_show_made_in: recipe.always_show_made_in,
        always_show_products: recipe.always_show_products,
        emissions_multiplier: recipe.emissions_multiplier.clone(),
        enabled: recipe.enabled,
        energy_required: recipe.energy_required.clone(),
        hidden: recipe.hidden,
        hide_from_player_crafting: recipe.hide_from_player_crafting,
        hide_from_stats: recipe.hide_from_stats,
        ingredients: recipe.ingredients.clone().unwrap(),
        main_product: recipe.main_product.clone(),
        overload_multiplier: recipe.overload_multiplier,
        requester_paste_multiplier: recipe.requester_paste_multiplier,
        result: recipe.result.clone(),
        result_count: recipe.result_count,
        results: recipe.results.clone(),
        show_amount_in_title: recipe.show_amount_in_title,
        unlock_results: recipe.unlock_results,
    }
}

fn map_type(type_: &Option<String>) -> Type {
    type_.as_ref().map_or(Type::Item, |type_| {
        if type_ == "item" {
            Type::Item
        } else {
            Type::Fluid
        }
    })
}

fn map_double(double: &Option<FloatingPoint<f64>>) -> f64 {
    let precision = 10_000.0;
    (double
        .as_ref()
        .map_or(1.0, |double| *double.as_value().unwrap())
        * precision)
        .round()
        / precision
}
