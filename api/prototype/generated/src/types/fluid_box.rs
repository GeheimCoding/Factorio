pub struct FluidBox {
    always_draw_covers: bool,
    draw_only_when_connected: bool,
    enable_working_visualisations: Vec<String>,
    filter: crate::types::FluidID,
    hide_connection_info: bool,
    max_pipeline_extent: u32,
    maximum_temperature: f32,
    minimum_temperature: f32,
    pipe_connections: Vec<crate::types::PipeConnectionDefinition>,
    pipe_covers: crate::types::Sprite4Way,
    pipe_covers_frozen: crate::types::Sprite4Way,
    pipe_picture: crate::types::Sprite4Way,
    pipe_picture_frozen: crate::types::Sprite4Way,
    production_type: ProductionType,
    render_layer: crate::types::RenderLayer,
    secondary_draw_order: i8,
    secondary_draw_orders: FluidBoxSecondaryDrawOrders,
    volume: crate::types::FluidAmount,
}
#[derive(serde::Deserialize)]
pub enum ProductionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "input_output")]
    InputOutput,
    #[serde(rename = "output")]
    Output,
}
pub struct FluidBoxSecondaryDrawOrders {
    east: i8,
    north: i8,
    south: i8,
    west: i8,
}
