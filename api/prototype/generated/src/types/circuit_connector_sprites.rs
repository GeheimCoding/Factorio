#[derive(serde::Deserialize)]
pub struct CircuitConnectorSprites {
    blue_led_light_offset: Option<crate::types::Vector>,
    connector_main: Option<crate::types::Sprite>,
    connector_shadow: Option<crate::types::Sprite>,
    led_blue: crate::types::Sprite,
    led_blue_off: Option<crate::types::Sprite>,
    led_green: crate::types::Sprite,
    led_light: crate::types::LightDefinition,
    led_red: crate::types::Sprite,
    red_green_led_light_offset: Option<crate::types::Vector>,
    wire_pins: Option<crate::types::Sprite>,
    wire_pins_shadow: Option<crate::types::Sprite>,
}
