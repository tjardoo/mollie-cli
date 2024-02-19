use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MolliePayment {
    pub id: String,
    pub mode: MolliePaymentMode,
    pub status: MolliePaymentStatus,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MolliePaymentMode {
    Live,
    Test,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MolliePaymentStatus {
    Open,
    Canceled,
    Pending,
    Authorized,
    Expired,
    Failed,
    Paid,
}
