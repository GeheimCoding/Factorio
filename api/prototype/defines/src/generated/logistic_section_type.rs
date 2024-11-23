#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum LogisticSectionType {
    CircuitControlled = 1,
    Manual = 0,
    RequestMissingMaterialsControlled = 3,
    TransitionalRequestControlled = 2,
}
