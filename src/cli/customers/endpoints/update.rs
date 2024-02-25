use mollie_api::resources::customers::{MollieCustomer, UpdateCustomerRequest};

use crate::{api::get_client, cli::customers::resource::Customer, errors::CliError};

pub fn command(
    id: &str,
    name: Option<String>,
    email: Option<String>,
    locale: Option<String>,
) -> Result<(), CliError> {
    let parameters = UpdateCustomerRequest {
        name,
        email,
        locale,
    };

    let customer: MollieCustomer = get_client().customers().update(id, parameters)?;

    let customer: Customer = customer.into();

    println!("{}", customer);

    Ok(())
}
