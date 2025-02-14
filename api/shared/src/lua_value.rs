#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LuaValue {
    String(String),
    Number(i64),
    State(State),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum State {
    Lookup,
    AllDifferent,
    // README: Adjustment [2]
    ContainsDuplicates,
    // README: Adjustment [2]
}

// TODO: add method here to get serde header
