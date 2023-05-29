// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize)]
pub struct LuaPollutionFlowStatistics {
    pub force: MaybeCycle<LuaForce>,
    pub input_counts: HashMap<String, u32>,
    pub output_counts: HashMap<String, u32>,
    pub object_name: String,
    pub valid: bool,
}

// ========= MANUAL PATCH =========
// ================================
