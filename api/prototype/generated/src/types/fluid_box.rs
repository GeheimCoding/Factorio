#[derive(Debug, serde::Deserialize)]
pub struct FluidBox {
    always_draw_covers: Option<bool>,
    #[serde(default = "default_draw_only_when_connected")]
    draw_only_when_connected: bool,
    enable_working_visualisations: Option<crate::vec::Vec<String>>,
    filter: Option<crate::types::FluidID>,
    #[serde(default = "default_hide_connection_info")]
    hide_connection_info: bool,
    // default: Value of `UtilityConstants::default_pipeline_extent`
    max_pipeline_extent: Option<u32>,
    maximum_temperature: Option<f32>,
    minimum_temperature: Option<f32>,
    pipe_connections: crate::vec::Vec<crate::types::PipeConnectionDefinition>,
    pipe_covers: Option<crate::types::Sprite4Way>,
    pipe_covers_frozen: Option<crate::types::Sprite4Way>,
    pipe_picture: Option<crate::types::Sprite4Way>,
    pipe_picture_frozen: Option<crate::types::Sprite4Way>,
    #[serde(default = "default_production_type")]
    production_type: ProductionType,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_secondary_draw_order")]
    secondary_draw_order: i8,
    secondary_draw_orders: Option<FluidBoxSecondaryDrawOrders>,
    volume: crate::types::FluidAmount,
}
fn default_draw_only_when_connected() -> bool {
    false
}
fn default_hide_connection_info() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum ProductionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "input-output")]
    InputOutput,
    #[serde(rename = "output")]
    Output,
}
fn default_production_type() -> ProductionType {
    ProductionType::None
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_secondary_draw_order() -> i8 {
    1
}
#[derive(Debug, serde::Deserialize)]
pub struct FluidBoxSecondaryDrawOrders {
    east: Option<i8>,
    north: Option<i8>,
    south: Option<i8>,
    west: Option<i8>,
}
