use crate::errors::CliError;

use self::command::CustomerCommand;

pub mod command;
pub mod endpoints;
pub mod resource;

pub fn command(command: CustomerCommand) -> Result<(), CliError> {
    match command {
        CustomerCommand::Get { id } => endpoints::get::command(&id)?,
        _ => {
            unimplemented!();
        }
    };

    Ok(())
}
