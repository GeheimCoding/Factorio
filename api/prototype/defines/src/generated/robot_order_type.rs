#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum RobotOrderType {
    Construct = 2,
    Deconstruct = 3,
    Deliver = 7,
    DeliverItems = 4,
    ExplodeCliff = 6,
    Pickup = 0,
    PickupItems = 8,
    Repair = 1,
    Upgrade = 5,
}
