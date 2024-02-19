use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MollieMethod {
    pub id: String,
    pub description: String,
    pub status: Option<MollieMethodStatus>,
}

#[derive(Deserialize, Debug)]
pub enum MollieMethodStatus {
    #[serde(rename = "activated")]
    Activated,
    #[serde(rename = "pending-boarding")]
    PendingBoarding,
    #[serde(rename = "pending-review")]
    PendingReview,
    #[serde(rename = "pending-external")]
    PendingExternal,
    #[serde(rename = "rejected")]
    Rejected,
}
