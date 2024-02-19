use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MolliePayment {
    pub id: String,
    pub status: MolliePaymentStatus,
    pub amount: MolliePaymentAmount,
    pub mode: MolliePaymentMode,
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

#[derive(Deserialize, Debug)]
pub struct MolliePaymentAmount {
    pub value: String,
    pub currency: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MolliePaymentMode {
    Live,
    Test,
}
