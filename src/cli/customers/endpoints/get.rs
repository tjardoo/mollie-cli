use mollie_api::resources::customers::MollieCustomer;

use crate::{api::get_client, cli::customers::resource::Customer, errors::CliError};

pub fn command(id: &str) -> Result<(), CliError> {
    let customer: MollieCustomer = get_client().customers().get(id)?;

    let customer: Customer = customer.into();

    println!("{}", customer);

    Ok(())
}
