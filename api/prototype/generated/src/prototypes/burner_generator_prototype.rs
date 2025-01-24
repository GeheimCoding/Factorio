#[derive(serde::Deserialize)]
pub struct BurnerGeneratorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_always_draw_idle_animation")]
    always_draw_idle_animation: bool,
    animation: Option<crate::types::Animation4Way>,
    burner: crate::types::BurnerEnergySource,
    energy_source: crate::types::ElectricEnergySource,
    idle_animation: Option<crate::types::Animation4Way>,
    max_power_output: crate::types::Energy,
    perceived_performance: Option<crate::types::PerceivedPerformance>,
}
fn default_always_draw_idle_animation() -> bool {
    false
}
