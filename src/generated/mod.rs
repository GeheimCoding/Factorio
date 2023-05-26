mod classes;
pub use classes::*;

mod events;
pub use events::*;

mod concepts;
pub use concepts::*;

mod defines;
pub use defines::*;

mod factorio_types;
pub use factorio_types::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MaybeCycle<T> {
    Cycle { cycle_id: u32 },
    Value(Box<T>),
}