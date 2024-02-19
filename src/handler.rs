use mollie_cli::{
    cli::{customers, methods, payments},
    Command,
};

use crate::Cli;

pub fn handle_cli(cli: Cli) {
    match cli.command {
        Command::Customers(customers) => customers::command(customers),
        Command::Payments(payments) => payments::command(payments),
        Command::Methods(methods) => methods::command(methods),
    }
}
