use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum MethodCommand {
    List,
    Get {
        #[arg(short = None, long = "id")]
        id: String,
    },
}
