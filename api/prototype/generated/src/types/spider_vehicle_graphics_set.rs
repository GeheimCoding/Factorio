pub struct SpiderVehicleGraphicsSet {
    base_: crate::types::SpiderTorsoGraphicsSet,
    autopilot_destination_on_map_visualisation: crate::types::Animation,
    autopilot_destination_queue_on_map_visualisation: crate::types::Animation,
    autopilot_destination_queue_visualisation: crate::types::Animation,
    autopilot_destination_visualisation: crate::types::Animation,
    autopilot_destination_visualisation_render_layer: crate::types::RenderLayer,
    autopilot_path_visualisation_line_width: f32,
    autopilot_path_visualisation_on_map_line_width: f32,
    eye_light: crate::types::LightDefinition,
    light: crate::types::LightDefinition,
    light_positions: Vec<Vec<crate::types::Vector>>,
}
