#[derive(serde::Deserialize)]
pub struct SliderStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    button: Option<crate::types::ButtonStyleSpecification>,
    draw_notches: Option<bool>,
    empty_bar: Option<crate::types::ElementImageSet>,
    empty_bar_disabled: Option<crate::types::ElementImageSet>,
    full_bar: Option<crate::types::ElementImageSet>,
    full_bar_disabled: Option<crate::types::ElementImageSet>,
    high_button: Option<crate::types::ButtonStyleSpecification>,
    notch: Option<crate::types::ElementImageSet>,
    #[serde(rename = "type")]
    type_: String,
}
