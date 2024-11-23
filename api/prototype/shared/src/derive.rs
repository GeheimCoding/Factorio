use std::fmt::{Display, Formatter};

pub struct Derive {
    derives: Vec<String>,
    other: Vec<String>,
}

impl Derive {
    pub fn new() -> Self {
        Self {
            derives: vec![String::from("Debug"), String::from("Clone")],
            other: vec![],
        }
    }

    pub fn with_serde(mut self) -> Self {
        self.add_derives(&["serde::Deserialize"]);
        self
    }

    pub fn with_kebab_case(mut self) -> Self {
        self.add_other(r#"#[serde(rename_all="kebab-case")]"#);
        self
    }

    pub fn with_hash(mut self) -> Self {
        self.add_derives(&["PartialEq", "Eq", "Hash"]);
        self
    }

    pub fn with_serde_repr(mut self) -> Self {
        self.add_derives(&["serde_repr::Deserialize_repr"]);
        self.add_other("#[repr(u16)]");
        self
    }

    fn add_derives(&mut self, derives: &[&str]) {
        for derive in derives {
            let derive = String::from(*derive);
            if !self.derives.contains(&derive) {
                self.derives.push(derive);
            }
        }
    }

    fn add_other(&mut self, other: &str) {
        let other = String::from(other);
        if !self.other.contains(&other) {
            self.other.push(other);
        }
    }
}

impl Display for Derive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("#[derive({})]", self.derives.join(",")))?;
        f.write_str(&self.other.join(","))
    }
}
