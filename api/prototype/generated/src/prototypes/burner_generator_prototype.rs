pub struct BurnerGeneratorPrototype {
    always_draw_idle_animation: bool,
    animation: crate::types::Animation4Way,
    burner: crate::types::BurnerEnergySource,
    energy_source: crate::types::ElectricEnergySource,
    idle_animation: crate::types::Animation4Way,
    max_power_output: crate::types::Energy,
    perceived_performance: crate::types::PerceivedPerformance,
}
