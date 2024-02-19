use mollie_api::resources::customers::MollieCustomer;

use crate::{api::get_client, cli::customers::resource::Customer};

pub fn command(id: &str) {
    let customer: MollieCustomer = get_client().customers().get(id).unwrap();

    let customer: Customer = customer.into();

    println!("{}", customer);
}
