#[derive(Debug, serde::Deserialize)]
pub struct AirbornePollutantPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    affects_evolution: bool,
    affects_water_tint: bool,
    chart_color: crate::types::Color,
    icon: crate::types::Sprite,
    #[serde(default = "default_localised_name_with_amount")]
    localised_name_with_amount: String,
}
fn default_localised_name_with_amount() -> String {
    String::from("airborne-pollutant-name-with-amount.<name>")
}
