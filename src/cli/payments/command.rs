use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum PaymentCommand {
    Create,
    Get {
        #[arg(short = None, long = "id")]
        #[arg(value_parser = starts_with_prefix_tr)]
        id: String,
    },
    Update,
    Delete,
}

fn starts_with_prefix_tr(id: &str) -> Result<String, String> {
    if id.starts_with("tr_") {
        Ok(id.to_string())
    } else {
        Err("The id must start with 'tr_'".to_string())
    }
}
