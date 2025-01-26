#[derive(Debug, serde::Deserialize)]
pub enum PlayerInputMethodFilter {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "keyboard_and_mouse")]
    KeyboardAndMouse,
    #[serde(rename = "game_controller")]
    GameController,
}
