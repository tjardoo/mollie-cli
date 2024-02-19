use crate::{errors::ApiError, resources::methods::MollieMethod, Client};

pub struct Methods<'a> {
    client: &'a Client,
}

impl Client {
    pub fn methods(&self) -> Methods {
        Methods { client: self }
    }
}

impl Methods<'_> {
    pub fn get(&self, payment_id: &str) -> Result<MollieMethod, ApiError> {
        let response = self.client.get(format!("methods/{}", payment_id))?;

        let method: MollieMethod = serde_json::from_value(response).unwrap();

        Ok(method)
    }

    pub fn list(&self) -> Result<Vec<MollieMethod>, ApiError> {
        let response = self.client.get("methods".to_string())?;

        let methods: Vec<MollieMethod> =
            serde_json::from_value(response["_embedded"]["methods"].clone()).unwrap();

        Ok(methods)
    }

    pub fn all(&self) -> Result<Vec<MollieMethod>, ApiError> {
        let response = self.client.get("methods/all".to_string())?;

        dbg!(&response);

        let methods: Vec<MollieMethod> =
            serde_json::from_value(response["_embedded"]["methods"].clone()).unwrap();

        Ok(methods)
    }
}
