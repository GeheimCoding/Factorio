pub struct RailPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    build_grid_size: String,
    deconstruction_marker_positions: Vec<crate::types::Vector>,
    ending_shifts: Vec<crate::types::Vector>,
    extra_planner_goal_penalty: f64,
    extra_planner_penalty: f64,
    fence_pictures: crate::types::RailFenceGraphicsSet,
    forced_fence_segment_count: u8,
    pictures: crate::types::RailPictureSet,
    removes_soft_decoratives: bool,
    selection_box: crate::types::BoundingBox,
    walking_sound: crate::types::Sound,
}
