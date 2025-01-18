#[derive(serde::Deserialize)]
pub struct SliderStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    button: crate::types::ButtonStyleSpecification,
    draw_notches: bool,
    empty_bar: crate::types::ElementImageSet,
    empty_bar_disabled: crate::types::ElementImageSet,
    full_bar: crate::types::ElementImageSet,
    full_bar_disabled: crate::types::ElementImageSet,
    high_button: crate::types::ButtonStyleSpecification,
    notch: crate::types::ElementImageSet,
    #[serde(rename = "type")]
    type_: String,
}
