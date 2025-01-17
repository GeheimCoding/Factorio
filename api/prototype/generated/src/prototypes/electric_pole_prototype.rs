#[derive(serde::Deserialize)]
pub struct ElectricPolePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    active_picture: crate::types::Sprite,
    auto_connect_up_to_n_wires: u8,
    connection_points: Vec<crate::types::WireConnectionPoint>,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    light: crate::types::LightDefinition,
    maximum_wire_distance: f64,
    pictures: crate::types::RotatedSprite,
    radius_visualisation_picture: crate::types::Sprite,
    supply_area_distance: f64,
    track_coverage_during_build_by_moving: bool,
}
