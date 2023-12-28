#![allow(dead_code)]

use std::{fmt::Display, io, str::FromStr};

use serde::{Deserialize, Deserializer};

/// ## [Prototype JSON Format](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#prototype-json-format)
///
/// The prototype API documentation is available in a machine-readable [JSON format](https://lua-api.factorio.com/1.1.101/prototype-api.json). It allows for the creation of developer tools that provide code completion and related functionality. This page documents the structure of this format.
///
/// The current `api_version` that these docs reflect is `4`, which was introduced with Factorio `1.1.89`. See [Changelog](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#Changelog).
///
/// ## [General notes](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#general-notes)
///
/// Some notes that apply to the format in general:
/// - If a member would be `null`, it is omitted from the JSON instead.
/// - Descriptions are generally empty (`""`) instead of `null` if they could exist on any given member, but just happen to be empty (ex. an empty attribute description).
/// - Every list is sorted alphabetically by name. To replicate the order seen on the website, it can be sorted by the `order` property of its members.
/// - Text (descriptions, examples, etc.) is formatted as [Markdown](https://daringfireball.net/projects/markdown/), which includes links, inline code, and code blocks. More on how links work right below.
///
/// ## [Link format](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#link-format)
///
/// All text can contain Markdown-type links. There are two broad categories for these: hyperlinks that link to any external website, and internal links that refer to another part of this documentation. All of them will have a title that should be displayed as the link's text.
///
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
pub struct PrototypeApiFormat {
    /// The application this documentation is for. Will always be `"factorio"`.
    application: String,
    /// Indicates the stage this documentation is for. Will always be `"prototype"` (as opposed to `"runtime"`; see the [data lifecycle](https://lua-api.factorio.com/1.1.101/auxiliary/data-lifecycle.html) for more detail).
    stage: String,
    /// The version of the game that this documentation is for. An example would be `"1.1.90"`.
    application_version: String,
    /// The version of the machine-readable format itself. It is incremented every time the format changes. The version this documentation reflects is stated at the top.
    api_version: u8,
    /// The list of prototypes that can be created. Equivalent to the [prototypes](https://lua-api.factorio.com/1.1.101/prototypes.html) page.
    prototypes: Vec<Prototype>,
    /// The list of types (concepts) that the format uses. Equivalent to the [types](https://lua-api.factorio.com/1.1.101/types.html) page.
    types: Vec<Concept>,
}

#[derive(Debug, Deserialize)]
struct Prototype {
    /// The name of the prototype.
    name: String,
    /// The order of the prototype as shown in the HTML.
    order: u16,
    /// The text description of the prototype.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the prototype.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the prototype.
    images: Option<Vec<Image>>,
    /// The name of the prototype's parent, if any.
    parent: Option<String>,
    /// Whether the prototype is abstract, and thus can't be created directly.
    #[serde(rename = "abstract")]
    abstract_: bool,
    /// The type name of the prototype, like `"boiler"`. `null` for abstract prototypes.
    typename: Option<String>,
    /// instance_limit :: number (optional): The maximum number of instances of this prototype that can be created, if any.
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_number_from_string")]
    instance_limit: Option<u8>,
    /// Whether the prototype is deprecated and shouldn't be used anymore.
    deprecated: bool,
    /// The list of properties that the prototype has. May be an empty array.
    properties: Vec<Property>,
    /// A special set of properties that the user can add an arbitrary number of. Specifies the type of the key and value of the custom property.
    custom_properties: Option<CustomProperties>,
}

#[derive(Debug, Deserialize)]
struct Concept {
    /// The name of the type.
    name: String,
    /// The order of the type as shown in the HTML.
    order: u16,
    /// The text description of the type.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the type.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the type.
    images: Option<Vec<Image>>,
    /// The name of the type's parent, if any.
    parent: Option<String>,
    /// Whether the type is abstract, and thus can't be created directly.
    #[serde(rename = "abstract")]
    abstract_: bool,
    /// Whether the type is inlined inside another property's description.
    inline: bool,
    /// The type of the type/concept (Yes, this naming is confusing). Either a proper [Type](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#Type), or the string `"builtin"`, indicating a fundamental type like `string` or `number`.
    #[serde(rename = "type")]
    type_: Type,
    /// The list of properties that the type has, if its type includes a struct. `null` otherwise.
    properties: Option<Vec<Property>>,
}

#[derive(Debug, Deserialize)]
struct Property {
    /// The name of the property.
    name: String,
    /// The order of the property as shown in the HTML.
    order: u16,
    /// The text description of the property.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the property.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the property.
    images: Option<Vec<Image>>,
    /// An alternative name for the property. Either this or name can be used to refer to the property.
    alt_name: Option<String>,
    /// Whether the property overrides a property of the same name in one of its parents.
    #[serde(rename = "override")]
    override_: bool,
    /// The type of the property.
    #[serde(rename = "type")]
    type_: Type,
    /// Whether the property is optional and can be omitted. If so, it falls back to a default value.
    optional: bool,
    /// default :: union[string, Literal] (optional): The default value of the property. Either a textual description or a literal value.
    default: Option<PropertyDefaultUnion>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PropertyDefaultUnion {
    String(String),
    Literal(Type),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Type {
    Simple(String),
    Complex(Box<ComplexType>),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "complex_type")]
#[serde(rename_all = "camelCase")]
enum ComplexType {
    Array {
        /// The type of the elements of the array.
        value: Type,
    },
    Dictionary {
        /// The type of the keys of the dictionary.
        key: Type,
        /// The type of the values of the dictionary.
        value: Type,
    },
    Tuple {
        /// The types of the members of this tuple in order.
        values: Vec<Type>,
    },
    Union {
        /// A list of all compatible types for this type.
        options: Vec<Type>,
        /// Whether the options of this union have a description or not.
        full_format: bool,
    },
    Literal {
        /// The value of the literal.
        value: ComplexTypeLiteralValueUnion,
        /// The text description of the literal, if any.
        description: Option<String>,
    },
    Type {
        /// The actual type. This format for types is used when they have descriptions attached to them.
        value: Type,
        /// The text description of the type.
        description: String,
    },
    /// Special type with no additional members. The properties themselves are listed on the API member that uses this type.
    Struct,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ComplexTypeLiteralValueUnion {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Deserialize)]
struct Image {
    /// The name of the image file to display. These files are placed into the `/static/images/` directory.
    filename: String,
    /// The explanatory text to show attached to the image.
    caption: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CustomProperties {
    /// The text description of the property.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the property.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the property.
    images: Option<Vec<Image>>,
    /// The type of the key of the custom property.
    key_type: Type,
    /// The type of the value of the custom property.
    value_type: Type,
}

fn deserialize_optional_number_from_string<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    String::deserialize(deserializer)?
        .parse::<T>()
        .map(|n| Some(n))
        .map_err(serde::de::Error::custom)
}

impl PrototypeApiFormat {
    pub fn generate_prototype_api(&self) -> io::Result<()> {
        Ok(())
    }
}
