use clap::Parser;
use miette::Result;
use mollie_cli::Cli;

mod handler;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match handler::handle_cli_and_run(cli) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);
        }
    }

    Ok(())
}
