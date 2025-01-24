#[derive(serde::Deserialize)]
pub struct DestroyDecorativesTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_decoratives_with_trigger_only")]
    decoratives_with_trigger_only: bool,
    // default: first layer
    from_render_layer: Option<crate::types::RenderLayer>,
    #[serde(default = "default_include_decals")]
    include_decals: bool,
    #[serde(default = "default_include_soft_decoratives")]
    include_soft_decoratives: bool,
    #[serde(default = "default_invoke_decorative_trigger")]
    invoke_decorative_trigger: bool,
    radius: f32,
    // default: last layer
    to_render_layer: Option<crate::types::RenderLayer>,
    #[serde(rename = "type")]
    type_: String,
}
fn default_decoratives_with_trigger_only() -> bool {
    false
}
fn default_include_decals() -> bool {
    false
}
fn default_include_soft_decoratives() -> bool {
    false
}
fn default_invoke_decorative_trigger() -> bool {
    true
}
