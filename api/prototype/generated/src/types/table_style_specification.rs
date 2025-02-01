#[derive(Debug, serde::Deserialize)]
pub struct TableStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    apply_row_graphical_set_per_column: Option<bool>,
    background_graphical_set: Option<crate::types::ElementImageSet>,
    border: Option<crate::types::BorderImageSet>,
    bottom_cell_padding: Option<i16>,
    cell_padding: Option<i16>,
    clicked_graphical_set: Option<crate::types::ElementImageSet>,
    column_alignments: Option<Vec<crate::types::ColumnAlignment>>,
    column_graphical_set: Option<crate::types::ElementImageSet>,
    column_ordering_ascending_button_style: Option<crate::types::ButtonStyleSpecification>,
    column_ordering_descending_button_style: Option<crate::types::ButtonStyleSpecification>,
    column_widths: Option<TableStyleSpecificationColumnWidths>,
    default_row_graphical_set: Option<crate::types::ElementImageSet>,
    even_row_graphical_set: Option<crate::types::ElementImageSet>,
    horizontal_line_color: Option<crate::types::Color>,
    horizontal_spacing: Option<i32>,
    hovered_graphical_set: Option<crate::types::ElementImageSet>,
    hovered_row_color: Option<crate::types::Color>,
    inactive_column_ordering_ascending_button_style: Option<crate::types::ButtonStyleSpecification>,
    inactive_column_ordering_descending_button_style:
        Option<crate::types::ButtonStyleSpecification>,
    left_cell_padding: Option<i16>,
    odd_row_graphical_set: Option<crate::types::ElementImageSet>,
    right_cell_padding: Option<i16>,
    selected_clicked_graphical_set: Option<crate::types::ElementImageSet>,
    selected_graphical_set: Option<crate::types::ElementImageSet>,
    selected_hovered_graphical_set: Option<crate::types::ElementImageSet>,
    selected_row_color: Option<crate::types::Color>,
    top_cell_padding: Option<i16>,
    vertical_line_color: Option<crate::types::Color>,
    vertical_spacing: Option<i32>,
    wide_as_column_count: Option<bool>,
}
#[derive(Debug, serde::Deserialize)]
pub enum TableStyleSpecificationColumnWidths {
    #[serde(untagged)]
    ColumnWidthItem(Box<crate::types::ColumnWidthItem>),
    #[serde(untagged)]
    VecColumnWidth(Vec<crate::types::ColumnWidth>),
}
