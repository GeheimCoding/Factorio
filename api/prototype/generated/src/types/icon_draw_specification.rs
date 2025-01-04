pub struct IconDrawSpecification {
    renderLayer: IconDrawSpecificationRenderLayer,
    scale: f32,
    scale_for_many: f32,
    shift: Vector,
}
pub enum IconDrawSpecificationRenderLayer {
    EntityInfoIconBelow,
    EntityInfoIconAbove,
    AirEntityInfoIcon,
}
