#[derive(serde::Deserialize)]
pub struct SwitchStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    active_label: Option<crate::types::LabelStyleSpecification>,
    button: Option<crate::types::ButtonStyleSpecification>,
    default_background: Option<crate::types::Sprite>,
    disabled_background: Option<crate::types::Sprite>,
    hover_background: Option<crate::types::Sprite>,
    inactive_label: Option<crate::types::LabelStyleSpecification>,
    left_button_position: Option<u32>,
    middle_button_position: Option<u32>,
    right_button_position: Option<u32>,
    #[serde(rename = "type")]
    type_: String,
}
