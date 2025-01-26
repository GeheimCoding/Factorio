#[derive(Debug, serde::Deserialize)]
pub struct StyleWithClickableGraphicalSetSpecification {
    base_: crate::types::BaseStyleSpecification,
    clicked_graphical_set: Option<crate::types::ElementImageSet>,
    default_graphical_set: Option<crate::types::ElementImageSet>,
    disabled_graphical_set: Option<crate::types::ElementImageSet>,
    game_controller_selected_hovered_graphical_set: Option<crate::types::ElementImageSet>,
    hovered_graphical_set: Option<crate::types::ElementImageSet>,
    left_click_sound: Option<crate::types::Sound>,
    selected_clicked_graphical_set: Option<crate::types::ElementImageSet>,
    selected_graphical_set: Option<crate::types::ElementImageSet>,
    selected_hovered_graphical_set: Option<crate::types::ElementImageSet>,
}
