#[derive(serde::Deserialize)]
pub struct SpiderVehicleGraphicsSet {
    base_: crate::types::SpiderTorsoGraphicsSet,
    autopilot_destination_on_map_visualisation: Option<crate::types::Animation>,
    autopilot_destination_queue_on_map_visualisation: Option<crate::types::Animation>,
    autopilot_destination_queue_visualisation: Option<crate::types::Animation>,
    autopilot_destination_visualisation: Option<crate::types::Animation>,
    #[serde(default = "default_autopilot_destination_visualisation_render_layer")]
    autopilot_destination_visualisation_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_autopilot_path_visualisation_line_width")]
    autopilot_path_visualisation_line_width: f32,
    #[serde(default = "default_autopilot_path_visualisation_on_map_line_width")]
    autopilot_path_visualisation_on_map_line_width: f32,
    eye_light: Option<crate::types::LightDefinition>,
    light: Option<crate::types::LightDefinition>,
    light_positions: Option<Vec<Vec<crate::types::Vector>>>,
}
fn default_autopilot_destination_visualisation_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_autopilot_path_visualisation_line_width() -> f32 {
    0.1
}
fn default_autopilot_path_visualisation_on_map_line_width() -> f32 {
    2.0
}
