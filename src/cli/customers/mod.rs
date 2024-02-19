use self::command::CustomerCommand;

pub mod command;
pub mod get;
pub mod resource;

pub fn command(command: CustomerCommand) {
    match command {
        CustomerCommand::Get { id } => get::command(&id),
        _ => {
            unimplemented!();
        }
    }
}
