use mollie_api::Client;

pub fn get_client() -> Client {
    let api_key = std::env::var("MOLLIE_API_KEY").expect("$MOLLIE_API_KEY is not set");

    Client::new(api_key)
}
