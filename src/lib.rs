use clap::{Parser, Subcommand};
use cli::{customers::command::CustomerCommand, payments::command::PaymentCommand};

pub mod api;
pub mod cli;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(subcommand)]
    Customers(CustomerCommand),
    #[clap(subcommand)]
    Payments(PaymentCommand),
}
