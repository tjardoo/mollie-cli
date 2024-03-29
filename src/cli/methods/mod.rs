use crate::errors::CliError;

use self::command::MethodCommand;

pub mod command;
pub mod endpoints;
pub mod resource;

pub fn command(command: MethodCommand) -> Result<(), CliError> {
    match command {
        MethodCommand::List => endpoints::list::command()?,
        MethodCommand::Get { id } => endpoints::get::command(&id)?,
        MethodCommand::All => endpoints::all::command()?,
    };

    Ok(())
}
