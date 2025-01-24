#[derive(serde::Deserialize)]
pub struct FuelCategory {
    base_: crate::prototypes::Prototype,
    // default: `{"description.fuel-value"}`
    fuel_value_type: Option<crate::types::LocalisedString>,
}
