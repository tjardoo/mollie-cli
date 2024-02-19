use clap::{Parser, Subcommand};
use cli::{
    customers::command::CustomerCommand, methods::command::MethodCommand,
    payments::command::PaymentCommand,
};

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
    /// Manage customers
    #[clap(subcommand)]
    Customers(CustomerCommand),
    /// Manage payments
    #[clap(subcommand)]
    Payments(PaymentCommand),
    /// Manage methods
    #[clap(subcommand)]
    Methods(MethodCommand),
}
