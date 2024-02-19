use self::command::CustomerCommand;

pub mod command;
pub mod endpoints;
pub mod resource;

pub fn command(command: CustomerCommand) {
    match command {
        CustomerCommand::Get { id } => endpoints::get::command(&id),
        _ => {
            unimplemented!();
        }
    }
}
