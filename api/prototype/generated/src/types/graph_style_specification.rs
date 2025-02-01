#[derive(Debug, serde::Deserialize)]
pub struct GraphStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    background_color: Option<crate::types::Color>,
    data_line_highlight_distance: Option<u32>,
    font: Option<String>,
    graph_right_margin: Option<u32>,
    graph_top_margin: Option<u32>,
    grid_lines_color: Option<crate::types::Color>,
    guide_lines_color: Option<crate::types::Color>,
    horizontal_label_style: Option<crate::types::LabelStyleSpecification>,
    horizontal_labels_margin: Option<u32>,
    line_colors: Option<Vec<crate::types::Color>>,
    minimal_horizontal_label_spacing: Option<u32>,
    minimal_vertical_label_spacing: Option<u32>,
    selection_dot_radius: Option<u32>,
    vertical_label_style: Option<crate::types::LabelStyleSpecification>,
    vertical_labels_margin: Option<u32>,
}
