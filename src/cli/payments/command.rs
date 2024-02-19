use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum PaymentCommand {
    /// Create a new payment
    Create,
    /// Retrieve a payment
    Get {
        /// The id of the payment starting with 'tr_'
        #[arg(short = None, long = "id")]
        #[arg(value_parser = starts_with_prefix_tr)]
        id: String,
    },
    /// Update a payment
    Update,
    /// Cancel a payment
    Cancel,
}

fn starts_with_prefix_tr(id: &str) -> Result<String, String> {
    if id.starts_with("tr_") {
        Ok(id.to_string())
    } else {
        Err("The id must start with 'tr_'".to_string())
    }
}
