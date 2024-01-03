#![allow(dead_code)]

use std::io;

use serde::Deserialize;

/// ## [Runtime JSON Format](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#runtime-json-format)
///
/// The runtime API documentation is available in a machine-readable [JSON format](https://lua-api.factorio.com/1.1.101/runtime-api.json). It allows for the creation of developer tools that provide code completion and related functionality. This page documents the structure of this format.
///
/// The current api_version that these docs reflect is 4, which was introduced with Factorio 1.1.89. See [Changelog](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#Changelog).
///
/// ## [General notes](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#general-notes)
///
/// Some notes that apply to the format in general:
/// - If a member would be `null`, it is omitted from the JSON instead.
/// - Descriptions are generally empty (`""`) instead of `null` if they could exist on any given member, but just happen to be empty (ex. an empty attribute description).
/// - Inversely, descriptions are `null` (and thus omitted) if they don't exist at all (ex. the `variant_parameter_description` for a method without variant parameters).
/// - Every list is sorted alphabetically by name. To replicate the order seen on the website, it can be sorted by the `order` property of its members.
/// - Text (descriptions, examples, etc.) is formatted as [Markdown](https://daringfireball.net/projects/markdown/), which includes links, inline code, and code blocks. More on how links work right below.
///
/// ## [Link format](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#link-format)
///
/// All text can contain Markdown-type links. There are two broad categories for these: hyperlinks that link to any external website, and internal links that refer to another part of this documentation. All of them will have a title that should be displayed as the link's text.
/// - External links work like standard Markdown links, meaning they always start with `https://`, and are followed by the URL. Example: `[Factorio](https://factorio.com)`.
/// - Internal links are a bit more complex. They aren't valid hyperlinks, but instead use a custom shorthand format to refer to specific parts of the API. This format has three main parts:
///     - They always start with either `runtime:` or `prototype:`, indicating the stage that is linked to. The two stages are separate namespaces, as there would be naming conflicts otherwise. So this first part indicates whether to look for the API member among classes, events, etc., or among prototypes and types.
///     - The second part is the name of the API member being linked to. What this can be depends on the stage that's indicated beforehand. Examples would be `LuaGuiElement` or `on_player_created` for `runtime:`, and `RecipePrototype` or `EnergySource` for `prototype:`.
///         - Note that this can be the name of some stage-specific auxiliary pages instead. Namely, 'classes', 'events', 'concepts', 'defines' and 'builtin_types' for `runtime:`, and 'prototypes', 'types' for `prototype:`.
///     - The third, optional part of an internal link can specify a certain sub-member to refer to. Its format is `::<name>`, where name is the name of a class method or attribute, or a prototype/type property. It is invalid for any other member type.
///
/// Examples:
///
/// - `[LuaGuiElement](runtime:LuaGuiElement)` links to the `LuaGuiElement` class.
/// - `[results](prototype:RecipePrototype::results)` links to the `results` property of the `RecipePrototype` prototype.
/// - `[concepts](runtime:concepts)` links to the [Concepts](https://lua-api.factorio.com/1.1.101/concepts.html) overview page.
#[derive(Debug, Deserialize)]
pub struct RuntimeApiFormat {
    /// The application this documentation is for. Will always be `"factorio"`.
    application: String,
    /// Indicates the stage this documentation is for. Will always be `"runtime"` (as opposed to `"prototype"`; see the [Data Lifecycle](https://lua-api.factorio.com/1.1.101/auxiliary/data-lifecycle.html) for more detail).
    stage: String,
    /// The version of the game that this documentation is for. An example would be `"1.1.35"`.
    application_version: String,
    /// The version of the machine-readable format itself. It is incremented every time the format changes. The version this documentation reflects is stated at the top.
    api_version: u8,
    /// The list of classes (LuaObjects) the API provides. Equivalent to the [classes](https://lua-api.factorio.com/1.1.101/classes.html) page.
    classes: Vec<Class>,
    /// The list of events that the API provides. Equivalent to the [events](https://lua-api.factorio.com/1.1.101/events.html) page.
    events: Vec<Event>,
    /// The list of defines that the game uses. Equivalent to the [defines](https://lua-api.factorio.com/1.1.101/defines.html) page.
    defines: Vec<Define>,
    /// The list of types that are built into Lua itself. Equivalent to the [built-in types](https://lua-api.factorio.com/1.1.101/builtin-types.html) page.
    builtin_types: Vec<BuiltinType>,
    /// The list of concepts of various types that the API uses. Equivalent to the [concepts](https://lua-api.factorio.com/1.1.101/concepts.html) page.
    concepts: Vec<Concept>,
    /// The list of objects that the game provides as global variables to serve as entry points to the API.
    global_objects: Vec<GlobalObject>,
    /// The list of functions that the game provides as global variables to provide some specific functionality.
    global_functions: Vec<Method>,
}

#[derive(Debug, Deserialize)]
pub struct Class {
    /// The name of the class.
    name: String,
    /// The order of the class as shown in the HTML.
    order: u16,
    /// The text description of the class.
    description: String,
    /// A list of strings containing additional information about the class.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The methods that are part of the class.
    methods: Vec<Method>,
    /// The attributes that are part of the class.
    attributes: Vec<Attribute>,
    /// A list of operators on the class. They are called `call`, `index`, or `length` and have the format of either a [Method](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#Method) or an [Attribute](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#Attribute).
    operators: Vec<Operator>,
    /// Whether the class is never itself instantiated, only inherited from.
    #[serde(rename = "abstract")]
    abstract_: bool,
    /// A list of the names of the classes that his class inherits from.
    base_classes: Option<Vec<String>>,
}

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
pub struct Define {
    /// The name of the define.
    name: String,
    /// The order of the define as shown in the HTML.
    order: u16,
    /// The text description of the define.
    description: String,
    /// The members of the define.
    values: Option<Vec<BasicMember>>,
    /// A list of sub-defines.
    subkeys: Option<Vec<Define>>,
}

#[derive(Debug, Deserialize)]
pub struct BuiltinType {
    /// The name of the built-in type.
    name: String,
    /// The order of the built-in type as shown in the HTML.
    order: u16,
    /// The text description of the built-in type.
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct Concept {
    /// The name of the concept.
    name: String,
    /// The order of the concept as shown in the HTML.
    order: u16,
    /// The text description of the concept.
    description: String,
    /// A list of strings containing additional information about the concept.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The type of the concept.
    #[serde(rename = "type")]
    type_: Type,
}

#[derive(Debug, Deserialize)]
pub struct GlobalObject {
    /// The global variable name of the object.
    name: String,
    /// The order of the global object as shown in the HTML.
    order: u16,
    /// The text description of the global object.
    description: String,
    /// The class name of the global object.
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Deserialize)]
pub struct BasicMember {
    /// The name of the member.
    name: String,
    /// The order of the member as shown in the HTML.
    order: u16,
    /// The text description of the member.
    description: String,
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

#[derive(Debug, Deserialize)]
pub struct Type {}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    /// The name of the parameter.
    name: String,
    /// The order of the parameter as shown in the HTML.
    order: u16,
    /// The text description of the parameter.
    description: String,
    /// The type of the parameter.
    #[serde(rename = "type")]
    type_: Type,
    /// Whether the type is optional or not.
    optional: bool,
}

#[derive(Debug, Deserialize)]
pub struct ParameterGroup {
    /// The name of the parameter group.
    name: String,
    /// The order of the parameter group as shown in the HTML.
    order: u16,
    /// The text description of the parameter group.
    description: String,
    /// The parameters that the group adds.
    parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
pub struct Method {
    /// The name of the method.
    name: String,
    /// The order of the method as shown in the HTML.
    order: u16,
    /// The text description of the method.
    description: String,
    /// A list of strings containing additional information about the method.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// A list of events that this method might raise when called.
    raises: Option<Vec<EventRaised>>,
    /// A list of strings specifying the sub-type (of the class) that the method applies to.
    subclasses: Option<Vec<String>>,
    /// The parameters of the method. How to interpret them depends on the `takes_table` member.
    parameters: Vec<Parameter>,
    /// The optional parameters that depend on one of the main parameters. Only applies if `takes_table` is `true`.
    variant_parameter_groups: Option<Vec<ParameterGroup>>,
    /// The text description of the optional parameter groups.
    variant_parameter_description: Option<String>,
    /// The type of the variadic arguments of the method, if it accepts any.
    variadic_type: Option<Type>,
    /// The description of the variadic arguments of the method, if it accepts any.
    variadic_description: Option<String>,
    /// Whether the method takes a single table with named parameters or a sequence of unnamed parameters.
    takes_table: bool,
    /// If `takes_table` is `true`, whether that whole table is optional or not.
    table_is_optional: Option<bool>,
    /// The return values of this method, which can contain zero, one, or multiple values. Note that these have the same structure as parameters, but do not specify a name.
    return_values: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
pub struct Attribute {
    /// The name of the attribute.
    name: String,
    /// The order of the attribute as shown in the HTML.
    order: u16,
    /// The text description of the attribute.
    description: String,
    /// A list of strings containing additional information about the attribute.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// A list of events that this attribute might raise when written to.
    raises: Option<Vec<EventRaised>>,
    /// A list of strings specifying the sub-type (of the class) that the attribute applies to.
    subclasses: Option<Vec<String>>,
    /// The type of the attribute.
    #[serde(rename = "type")]
    type_: Type,
    /// Whether the attribute is optional or not.
    optional: bool,
    /// Whether the attribute can be read from.
    read: bool,
    /// Whether the attribute can be written to.
    write: bool,
}

#[derive(Debug, Deserialize)]
pub struct Operator {}

impl RuntimeApiFormat {
    pub fn generate_runtime_api(&self) -> io::Result<()> {
        Ok(())
    }
}
