use self::command::MethodCommand;

pub mod command;
pub mod endpoints;
pub mod resource;

pub fn command(command: MethodCommand) {
    match command {
        MethodCommand::List => endpoints::list::command(),
        MethodCommand::Get { id } => endpoints::get::command(&id),
    }
}
