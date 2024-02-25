use crate::errors::CliError;

use self::command::CustomerCommand;

pub mod command;
pub mod endpoints;
pub mod resource;

pub fn command(command: CustomerCommand) -> Result<(), CliError> {
    match command {
        CustomerCommand::Get { id } => endpoints::get::command(&id)?,
        CustomerCommand::Update {
            id,
            name,
            email,
            locale,
        } => endpoints::update::command(&id, name, email, locale)?,
        _ => {
            unimplemented!();
        }
    };

    Ok(())
}
