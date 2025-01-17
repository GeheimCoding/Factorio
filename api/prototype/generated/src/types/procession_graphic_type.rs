#[derive(serde::Deserialize)]
pub enum ProcessionGraphicType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "sprite")]
    Sprite,
    #[serde(rename = "animation")]
    Animation,
    #[serde(rename = "pod_catalogue")]
    PodCatalogue,
    #[serde(rename = "location_catalogue")]
    LocationCatalogue,
    #[serde(rename = "hatch_location_catalogue_index")]
    HatchLocationCatalogueIndex,
}
