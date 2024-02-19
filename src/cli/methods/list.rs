use mollie_api::resources::methods::MollieMethod;

use crate::{api::get_client, cli::methods::resource::Method};

pub fn command() {
    let methods: Vec<MollieMethod> = get_client().methods().list().unwrap();

    for method in methods {
        let method: Method = method.into();

        println!("{}", method);
    }
}
