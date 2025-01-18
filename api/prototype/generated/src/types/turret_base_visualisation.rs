#[derive(serde::Deserialize)]
pub struct TurretBaseVisualisation {
    animation: crate::types::Animation4Way,
    #[serde(default = "default_draw_when_frozen")]
    draw_when_frozen: bool,
    #[serde(default = "default_draw_when_has_ammo")]
    draw_when_has_ammo: bool,
    #[serde(default = "default_draw_when_has_energy")]
    draw_when_has_energy: bool,
    #[serde(default = "default_draw_when_no_ammo")]
    draw_when_no_ammo: bool,
    #[serde(default = "default_draw_when_no_energy")]
    draw_when_no_energy: bool,
    #[serde(default = "default_draw_when_not_frozen")]
    draw_when_not_frozen: bool,
    enabled_states: Vec<crate::types::TurretState>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_secondary_draw_order")]
    secondary_draw_order: i8,
}
fn default_draw_when_frozen() -> bool {
    true
}
fn default_draw_when_has_ammo() -> bool {
    true
}
fn default_draw_when_has_energy() -> bool {
    true
}
fn default_draw_when_no_ammo() -> bool {
    true
}
fn default_draw_when_no_energy() -> bool {
    true
}
fn default_draw_when_not_frozen() -> bool {
    true
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::LowerObject
}
fn default_secondary_draw_order() -> i8 {
    0
}
