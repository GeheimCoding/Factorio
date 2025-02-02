#[derive(Debug, serde::Deserialize)]
pub struct AirbornePollutantPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    affects_evolution: bool,
    affects_water_tint: bool,
    chart_color: crate::types::Color,
    icon: crate::types::Sprite,
}
