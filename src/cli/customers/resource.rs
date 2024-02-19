use std::fmt::Display;

use colored::Colorize;
use mollie_api::resources::customers::{MollieCustomer, MollieCustomerMode};

#[derive(Debug)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub locale: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub mode: Mode,
}

#[derive(Debug)]
pub enum Mode {
    Live,
    Test,
}

impl Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nid: {}\nname: {}\nemail: {}\nmode: {}",
            "-- Customer --".to_uppercase().bright_blue(),
            self.id.bold().underline(),
            self.name,
            self.email,
            match self.mode {
                Mode::Live => "live".to_uppercase(),
                Mode::Test => "test".to_uppercase(),
            },
        )
    }
}

impl From<MollieCustomer> for Customer {
    fn from(customer: MollieCustomer) -> Self {
        Self {
            id: customer.id,
            name: customer.name,
            email: customer.email,
            locale: customer.locale,
            metadata: customer.metadata,
            mode: match customer.mode {
                MollieCustomerMode::Live => Mode::Live,
                MollieCustomerMode::Test => Mode::Test,
            },
        }
    }
}
