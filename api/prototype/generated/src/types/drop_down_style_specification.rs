#[derive(Debug, serde::Deserialize)]
pub struct DropDownStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    button_style: Option<crate::types::ButtonStyleSpecification>,
    icon: Option<crate::types::Sprite>,
    list_box_style: Option<crate::types::ListBoxStyleSpecification>,
    opened_sound: Option<crate::types::Sound>,
    selector_and_title_spacing: Option<i16>,
}
