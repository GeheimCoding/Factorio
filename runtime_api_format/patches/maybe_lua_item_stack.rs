// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MaybeLuaItemStack {
    LuaItemStack(LuaItemStack),
    LuaItemStackInvalidForRead(LuaItemStackInvalidForRead),
}

#[derive(Debug, Deserialize)]
pub struct LuaItemStackInvalidForRead;

// TODO: maybe solve differently with default values?

// ========= MANUAL PATCH =========
// ================================
