mod ingredient;
mod product;

use crate::map_by_name;
use crate::prototype::crafting_machine::CraftingMachines;
use crate::prototype::recipe::ingredient::Ingredient;
use crate::prototype::recipe::product::Product;
use api::VehiclePrototypeBrakingPower::Double;
use api::{
    FloatingPoint, IngredientPrototype, ItemIngredientPrototype, ItemProductPrototype,
    LuaRecipePrototype, ProductAmountMax, ProductAmountMin, ProductPrototype, Prototype,
    RecipeData, RecipePrototype, RecipePrototypeNormal,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Item,
    Fluid,
}

// https://wiki.factorio.com/Materials_and_recipes
#[derive(Clone, Debug, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub category: String,
    pub ingredients: HashMap<String, Ingredient>,
    pub energy: f64,
    pub products: HashMap<String, Product>,
    pub crafting_machines: HashSet<String>,
}

pub type RecipesByName = HashMap<String, Recipe>;

#[derive(Debug)]
pub struct Recipes {
    pub by_name: RecipesByName,
    pub by_category: HashMap<String, RecipesByName>,
    pub by_ingredient: HashMap<String, RecipesByName>,
    pub by_product: HashMap<String, RecipesByName>,
    pub by_crafting_machine: HashMap<String, RecipesByName>,
}

impl From<(&RecipePrototype, &CraftingMachines)> for Recipe {
    fn from((recipe, crafting_machines): (&RecipePrototype, &CraftingMachines)) -> Self {
        let recipe_data =
            if let Some(RecipePrototypeNormal::RecipeData(recipe_data)) = recipe.normal.clone() {
                recipe_data
            } else {
                map_recipe_data(recipe)
            };
        let products = if let Some(results) = &recipe_data.results {
            map_by_name!(results.iter().map(Product::from).collect::<Vec<_>>())
        } else {
            let product = Product::from(&ProductPrototype::ItemProductPrototype(
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
            ingredients: map_by_name!(recipe_data
                .ingredients
                .iter()
                .map(Ingredient::from)
                .collect::<Vec<_>>()),
            energy: recipe_data
                .energy_required
                .clone()
                .map_or(0.5, |double| *double.as_value().unwrap()),
            products,
            crafting_machines: crafting_machines.by_name.keys().cloned().collect(),
        }
    }
}

impl From<(&Vec<Prototype>, &CraftingMachines)> for Recipes {
    fn from((prototypes, crafting_machines): (&Vec<Prototype>, &CraftingMachines)) -> Self {
        todo!()
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
