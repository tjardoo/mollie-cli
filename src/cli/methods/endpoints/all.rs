use mollie_api::resources::methods::MollieMethod;

use crate::{api::get_client, cli::methods::resource::Method, errors::CliError};

pub fn command() -> Result<(), CliError> {
    let methods: Vec<MollieMethod> = get_client().methods().all()?;

    for method in methods {
        let method: Method = method.into();

        println!("{}", method);
    }

    Ok(())
}
