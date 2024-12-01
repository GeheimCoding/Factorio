pub struct TurretBaseVisualisation {
    animation: Animation4Way,
    draw_when_frozen: bool,
    draw_when_has_ammo: bool,
    draw_when_has_energy: bool,
    draw_when_no_ammo: bool,
    draw_when_no_energy: bool,
    draw_when_not_frozen: bool,
    enabled_states: Vec<TurretState>,
    render_layer: RenderLayer,
    secondary_draw_order: i8,
}
