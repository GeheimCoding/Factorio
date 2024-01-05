#![allow(dead_code)]
use serde::Deserialize;

use super::parameter::Parameter;

#[derive(Debug, Deserialize)]
pub struct Event {
    /// The name of the event.
    name: String,
    /// The order of the event as shown in the HTML.
    order: u16,
    /// The text description of the event.
    description: String,
    /// A list of strings containing additional information about the event.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The event-specific information that is provided.
    data: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
pub struct EventRaised {
    /// The name of the event being raised.
    name: String,
    /// The order of the member as shown in the HTML.
    order: u16,
    /// The text description of the raised event.
    description: String,
    /// The timeframe during which the event is raised. One of "instantly", "current_tick", or "future_tick".
    timeframe: String,
    /// Whether the event is always raised, or only dependant on a certain condition.
    optional: bool,
}
