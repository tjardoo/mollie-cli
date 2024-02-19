use crate::{errors::ApiError, resources::payments::MolliePayment, Client};

pub struct Payments<'a> {
    client: &'a Client,
}

impl Client {
    pub fn payments(&self) -> Payments {
        Payments { client: self }
    }
}

impl Payments<'_> {
    pub fn get(&self, payment_id: &str) -> Result<MolliePayment, ApiError> {
        let response = self.client.get(format!("payments/{}", payment_id));

        let payment: MolliePayment = serde_json::from_value(response?)?;

        Ok(payment)
    }
}
