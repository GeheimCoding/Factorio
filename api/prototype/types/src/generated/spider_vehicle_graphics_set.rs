pub struct SpiderVehicleGraphicsSet {
    autopilot_destination_on_map_visualisation: Animation,
    autopilot_destination_queue_on_map_visualisation: Animation,
    autopilot_destination_queue_visualisation: Animation,
    autopilot_destination_visualisation: Animation,
    autopilot_destination_visualisation_render_layer: RenderLayer,
    autopilot_path_visualisation_line_width: f32,
    autopilot_path_visualisation_on_map_line_width: f32,
    eye_light: LightDefinition,
    light: LightDefinition,
    light_positions: Vec<Vec<Vector>>,
}
