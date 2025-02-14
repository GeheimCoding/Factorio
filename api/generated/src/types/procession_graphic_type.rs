#[derive(Debug, serde::Deserialize)]
pub enum ProcessionGraphicType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "sprite")]
    Sprite,
    #[serde(rename = "animation")]
    Animation,
    #[serde(rename = "pod-catalogue")]
    PodCatalogue,
    #[serde(rename = "location-catalogue")]
    LocationCatalogue,
    #[serde(rename = "hatch-location-catalogue-index")]
    HatchLocationCatalogueIndex,
}
