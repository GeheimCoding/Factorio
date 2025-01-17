#[derive(serde::Deserialize)]
pub struct TrainVisualizationConstants {
    box_length: f32,
    box_width: f32,
    connection_distance: f32,
    final_margin: f32,
    joint_distance: f32,
    last_box_color: crate::types::Color,
    not_last_box_color: crate::types::Color,
    stock_number_scale: f32,
}
