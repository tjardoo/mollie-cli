use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum MethodCommand {
    /// List all available payment methods
    List,
    /// Retrieve a payment method
    Get {
        /// The id of the payment method
        #[arg(short = None, long = "id")]
        id: String,
    },
}
