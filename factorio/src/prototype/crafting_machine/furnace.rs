use api::FurnacePrototype;

#[derive(Debug)]
pub struct FurnaceData {
    pub result_inventory_size: u16,
    pub source_inventory_size: u16,
}

impl From<&FurnacePrototype> for FurnaceData {
    fn from(furnace: &FurnacePrototype) -> Self {
        Self {
            result_inventory_size: furnace.result_inventory_size,
            source_inventory_size: furnace.source_inventory_size,
        }
    }
}
