#[derive(serde::Deserialize)]
pub struct CliffPrototype {
    base_: crate::prototypes::EntityPrototype,
    cliff_explosive: Option<crate::types::ItemID>,
    grid_offset: crate::types::Vector,
    grid_size: crate::types::Vector,
    orientations: OrientedCliffPrototypeSet,
    place_as_crater: Option<crate::types::CraterPlacementDefinition>,
}
#[derive(serde::Deserialize)]
pub struct OrientedCliffPrototypeSet {
    east_to_none: crate::types::OrientedCliffPrototype,
    east_to_north: crate::types::OrientedCliffPrototype,
    east_to_south: crate::types::OrientedCliffPrototype,
    east_to_west: crate::types::OrientedCliffPrototype,
    none_to_east: crate::types::OrientedCliffPrototype,
    none_to_north: crate::types::OrientedCliffPrototype,
    none_to_south: crate::types::OrientedCliffPrototype,
    none_to_west: crate::types::OrientedCliffPrototype,
    north_to_east: crate::types::OrientedCliffPrototype,
    north_to_none: crate::types::OrientedCliffPrototype,
    north_to_south: crate::types::OrientedCliffPrototype,
    north_to_west: crate::types::OrientedCliffPrototype,
    south_to_east: crate::types::OrientedCliffPrototype,
    south_to_none: crate::types::OrientedCliffPrototype,
    south_to_north: crate::types::OrientedCliffPrototype,
    south_to_west: crate::types::OrientedCliffPrototype,
    west_to_east: crate::types::OrientedCliffPrototype,
    west_to_none: crate::types::OrientedCliffPrototype,
    west_to_north: crate::types::OrientedCliffPrototype,
    west_to_south: crate::types::OrientedCliffPrototype,
}
