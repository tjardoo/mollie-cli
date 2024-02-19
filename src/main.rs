use clap::Parser;
use handler::handle_cli;
use miette::Result;
use mollie_cli::Cli;

mod handler;

fn main() -> Result<()> {
    let cli = Cli::parse();

    handle_cli(cli);

    Ok(())
}
