#[derive(serde::Deserialize)]
pub struct DropDownStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    button_style: crate::types::ButtonStyleSpecification,
    icon: crate::types::Sprite,
    list_box_style: crate::types::ListBoxStyleSpecification,
    opened_sound: crate::types::Sound,
    selector_and_title_spacing: i16,
    #[serde(rename = "type")]
    type_: String,
}
