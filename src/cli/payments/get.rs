use mollie_api::resources::payments::MolliePayment;

use crate::{api::get_client, cli::payments::resource::Payment};

pub fn command(id: &str) {
    let payment: MolliePayment = get_client().payments().get(id).unwrap();

    let payment: Payment = payment.into();

    println!("{}", payment);
}
