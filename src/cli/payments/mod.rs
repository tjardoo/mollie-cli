use self::command::PaymentCommand;

pub mod command;
pub mod get;
pub mod resource;

pub fn command(command: PaymentCommand) {
    match command {
        PaymentCommand::Get { id } => get::command(&id),
        _ => {
            unimplemented!();
        }
    }
}
