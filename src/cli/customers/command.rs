use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum CustomerCommand {
    /// Create a new customer
    Create,
    /// Retrieve a customer
    Get {
        /// The id of the customer starting with 'cst_'
        #[arg(short = None, long = "id")]
        #[arg(value_parser = starts_with_prefix_cst)]
        id: String,
    },
    /// Update a customer
    Update,
    /// Delete a customer
    Delete,
}

fn starts_with_prefix_cst(id: &str) -> Result<String, String> {
    if id.starts_with("cst_") {
        Ok(id.to_string())
    } else {
        Err("The id must start with 'cst_'".to_string())
    }
}
