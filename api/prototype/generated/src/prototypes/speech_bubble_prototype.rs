#[derive(serde::Deserialize)]
pub struct SpeechBubblePrototype {
    base_: crate::prototypes::EntityPrototype,
    #[serde(default = "default_fade_in_out_ticks")]
    fade_in_out_ticks: u32,
    style: String,
    #[serde(default = "default_wrapper_flow_style")]
    wrapper_flow_style: String,
    #[serde(default = "default_y_offset")]
    y_offset: f64,
}
fn default_fade_in_out_ticks() -> u32 {
    60
}
fn default_wrapper_flow_style() -> String {
    String::from("flow_style")
}
fn default_y_offset() -> f64 {
    0.0
}
