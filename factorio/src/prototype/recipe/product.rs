use crate::prototype::recipe::{map_double, map_type};
use crate::prototype::Type;
use api::{ItemProductPrototype, ProductPrototype};

#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub name: String,
    pub amount: f64,
    pub probability: f64,
    pub type_: Type,
}

impl From<&ProductPrototype> for Product {
    fn from(product: &ProductPrototype) -> Self {
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
                    Self {
                        name: name.clone(),
                        amount: amount.map_or(1.0, |amount| amount as f64),
                        probability: map_double(probability),
                        type_: map_type(type_),
                    }
                }
                ItemProductPrototype::ItemIDU16((name, amount)) => Self {
                    name: name.clone(),
                    amount: *amount as f64,
                    probability: 1.0,
                    type_: Type::Item,
                },
            },
            ProductPrototype::FluidProductPrototype(fluid) => Self {
                name: fluid.name.clone(),
                amount: map_double(&fluid.amount),
                probability: map_double(&fluid.probability),
                type_: Type::Fluid,
            },
        }
    }
}
