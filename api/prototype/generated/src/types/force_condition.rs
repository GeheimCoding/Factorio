#[derive(Debug, serde::Deserialize)]
pub enum ForceCondition {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "enemy")]
    Enemy,
    #[serde(rename = "ally")]
    Ally,
    #[serde(rename = "friend")]
    Friend,
    #[serde(rename = "not_friend")]
    NotFriend,
    #[serde(rename = "same")]
    Same,
    #[serde(rename = "not_same")]
    NotSame,
}
