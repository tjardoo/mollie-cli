use crate::{errors::ApiError, resources::customers::MollieCustomer, Client};

pub struct Customers<'a> {
    client: &'a Client,
}

impl Client {
    pub fn customers(&self) -> Customers {
        Customers { client: self }
    }
}

impl Customers<'_> {
    pub fn get(&self, customer_id: &str) -> Result<MollieCustomer, ApiError> {
        let response = self.client.get(format!("customers/{}", customer_id));

        let customer: MollieCustomer = serde_json::from_value(response?)?;

        Ok(customer)
    }
}
