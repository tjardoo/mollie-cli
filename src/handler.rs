use mollie_cli::{errors::CliError, Command};

use crate::Cli;

pub fn handle_cli_and_run(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Command::Customers(customers) => mollie_cli::cli::customers::command(customers)?,
        Command::Payments(payments) => mollie_cli::cli::payments::command(payments)?,
        Command::Methods(methods) => mollie_cli::cli::methods::command(methods)?,
    };

    Ok(())
}
