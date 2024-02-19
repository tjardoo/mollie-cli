use mollie_api::resources::methods::MollieMethod;

use crate::{api::get_client, cli::methods::resource::Method};

pub fn command(id: &str) {
    let method: MollieMethod = get_client().methods().get(id).unwrap();

    let method: Method = method.into();

    println!("{}", method);
}
