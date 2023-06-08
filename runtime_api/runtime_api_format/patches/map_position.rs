// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize)]
pub struct MapPositionTable {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
pub struct MapPositionTuple {
    pub position: MapPositionTable,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MapPosition {
    Table(MapPositionTable),
    Tuple(MapPositionTuple),
}

// TODO: macro for table or tuple?

// ========= MANUAL PATCH =========
// ================================
