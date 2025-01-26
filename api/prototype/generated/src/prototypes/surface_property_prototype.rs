#[derive(Debug, serde::Deserialize)]
pub struct SurfacePropertyPrototype {
    base_: crate::prototypes::Prototype,
    default_value: f64,
    #[serde(default = "default_is_time")]
    is_time: bool,
    // default: surface-property-unit.[prototype name]
    localised_unit_key: Option<String>,
}
fn default_is_time() -> bool {
    false
}
