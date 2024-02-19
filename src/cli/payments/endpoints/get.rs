use mollie_api::resources::payments::MolliePayment;

use crate::{api::get_client, cli::payments::resource::Payment, errors::CliError};

pub fn command(id: &str) -> Result<(), CliError> {
    let payment: MolliePayment = get_client().payments().get(id)?;

    let payment: Payment = payment.into();

    println!("{}", payment);

    Ok(())
}
