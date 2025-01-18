#[derive(serde::Deserialize)]
pub struct GraphStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    background_color: crate::types::Color,
    data_line_highlight_distance: u32,
    font: String,
    graph_right_margin: u32,
    graph_top_margin: u32,
    grid_lines_color: crate::types::Color,
    guide_lines_color: crate::types::Color,
    horizontal_label_style: crate::types::LabelStyleSpecification,
    horizontal_labels_margin: u32,
    line_colors: Vec<crate::types::Color>,
    minimal_horizontal_label_spacing: u32,
    minimal_vertical_label_spacing: u32,
    selection_dot_radius: u32,
    #[serde(rename = "type")]
    type_: String,
    vertical_label_style: crate::types::LabelStyleSpecification,
    vertical_labels_margin: u32,
}
