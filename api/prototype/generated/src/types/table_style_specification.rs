#[derive(serde::Deserialize)]
pub struct TableStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    apply_row_graphical_set_per_column: bool,
    background_graphical_set: crate::types::ElementImageSet,
    border: crate::types::BorderImageSet,
    bottom_cell_padding: i16,
    cell_padding: i16,
    clicked_graphical_set: crate::types::ElementImageSet,
    column_alignments: Vec<crate::types::ColumnAlignment>,
    column_graphical_set: crate::types::ElementImageSet,
    column_ordering_ascending_button_style: crate::types::ButtonStyleSpecification,
    column_ordering_descending_button_style: crate::types::ButtonStyleSpecification,
    column_widths: TableStyleSpecificationColumnWidths,
    default_row_graphical_set: crate::types::ElementImageSet,
    even_row_graphical_set: crate::types::ElementImageSet,
    horizontal_line_color: crate::types::Color,
    horizontal_spacing: i32,
    hovered_graphical_set: crate::types::ElementImageSet,
    hovered_row_color: crate::types::Color,
    inactive_column_ordering_ascending_button_style: crate::types::ButtonStyleSpecification,
    inactive_column_ordering_descending_button_style: crate::types::ButtonStyleSpecification,
    left_cell_padding: i16,
    odd_row_graphical_set: crate::types::ElementImageSet,
    right_cell_padding: i16,
    selected_clicked_graphical_set: crate::types::ElementImageSet,
    selected_graphical_set: crate::types::ElementImageSet,
    selected_hovered_graphical_set: crate::types::ElementImageSet,
    selected_row_color: crate::types::Color,
    top_cell_padding: i16,
    #[serde(rename = "type")]
    type_: String,
    vertical_line_color: crate::types::Color,
    vertical_spacing: i32,
    wide_as_column_count: bool,
}
#[derive(serde::Deserialize)]
pub enum TableStyleSpecificationColumnWidths {
    #[serde(untagged)]
    ColumnWidthItem(Box<crate::types::ColumnWidthItem>),
    #[serde(untagged)]
    VecColumnWidth(Vec<crate::types::ColumnWidth>),
}
