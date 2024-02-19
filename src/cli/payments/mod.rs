use crate::errors::CliError;

use self::command::PaymentCommand;

pub mod command;
pub mod endpoints;
pub mod resource;

pub fn command(command: PaymentCommand) -> Result<(), CliError> {
    match command {
        PaymentCommand::Get { id } => endpoints::get::command(&id)?,
        _ => {
            unimplemented!();
        }
    };

    Ok(())
}
