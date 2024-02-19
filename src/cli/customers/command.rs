use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum CustomerCommand {
    Create,
    Get {
        #[arg(short = None, long = "id")]
        #[arg(value_parser = starts_with_prefix_cst)]
        id: String,
    },
    Update,
    Delete,
}

fn starts_with_prefix_cst(id: &str) -> Result<String, String> {
    if id.starts_with("cst_") {
        Ok(id.to_string())
    } else {
        Err("The id must start with 'cst_'".to_string())
    }
}
