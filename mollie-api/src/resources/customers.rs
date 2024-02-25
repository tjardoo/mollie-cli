use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct MollieCustomer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub locale: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub mode: MollieCustomerMode,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MollieCustomerMode {
    Live,
    Test,
}

#[derive(Serialize, Debug)]
pub struct UpdateCustomerRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub locale: Option<String>,
}
