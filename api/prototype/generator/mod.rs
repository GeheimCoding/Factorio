use std::{fs, io};

use super::PrototypeApiFormat;

impl PrototypeApiFormat {
    pub fn generate_prototype_api(&self) -> io::Result<()> {
        let mut result = String::new();
        for proto in &self.prototypes {
            result.push_str(&format!("pub struct {} {{\n", proto.name));
            if let Some(parent) = &proto.parent {
                result.push_str(&format!("    parent_: {parent},\n"));
            }
            for prop in &proto.properties {
                result.push_str(&format!("    {}: {:?},\n", prop.name, prop.type_));
            }
            result.push_str("}\n\n");
        }
        fs::write("src/generated/test.txt", result)?;

        Ok(())
    }
}
