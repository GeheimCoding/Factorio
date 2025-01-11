pub struct DestroyDecorativesTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    decoratives_with_trigger_only: bool,
    from_render_layer: crate::types::RenderLayer,
    include_decals: bool,
    include_soft_decoratives: bool,
    invoke_decorative_trigger: bool,
    radius: f32,
    to_render_layer: crate::types::RenderLayer,
    type_: String,
}
