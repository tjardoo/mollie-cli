use mollie_api::resources::methods::MollieMethod;

use crate::{api::get_client, cli::methods::resource::Method, errors::CliError};

pub fn command(id: &str) -> Result<(), CliError> {
    let method: MollieMethod = get_client().methods().get(id)?;

    let method: Method = method.into();

    println!("{}", method);

    Ok(())
}
