use self::command::MethodCommand;

pub mod command;
pub mod get;
pub mod list;
pub mod resource;

pub fn command(command: MethodCommand) {
    match command {
        MethodCommand::List => list::command(),
        MethodCommand::Get { id } => get::command(&id),
    }
}
