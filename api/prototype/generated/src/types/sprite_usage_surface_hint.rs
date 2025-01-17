#[derive(serde::Deserialize)]
pub enum SpriteUsageSurfaceHint {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "nauvis")]
    Nauvis,
    #[serde(rename = "vulcanus")]
    Vulcanus,
    #[serde(rename = "gleba")]
    Gleba,
    #[serde(rename = "fulgora")]
    Fulgora,
    #[serde(rename = "aquilo")]
    Aquilo,
    #[serde(rename = "space")]
    Space,
}
