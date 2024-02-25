use crate::{
    errors::ApiError,
    resources::customers::{MollieCustomer, UpdateCustomerRequest},
    Client,
};

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
        let response = self.client.get(format!("customers/{}", customer_id))?;

        let customer: MollieCustomer = serde_json::from_value(response).unwrap();

        Ok(customer)
    }

    pub fn update(
        &self,
        customer_id: &str,
        parameters: UpdateCustomerRequest,
    ) -> Result<MollieCustomer, ApiError> {
        let response = self
            .client
            .patch(format!("customers/{}", customer_id), parameters)?;

        let customer: MollieCustomer = serde_json::from_value(response).unwrap();

        Ok(customer)
    }
}
