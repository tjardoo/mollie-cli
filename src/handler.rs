use mollie_cli::{
    cli::{customers, payments},
    Command,
};

use crate::Cli;

pub fn handle_cli(cli: Cli) {
    match cli.command {
        Command::Customers(customers) => customers::command(customers),
        Command::Payments(payments) => payments::command(payments),
    }
}
