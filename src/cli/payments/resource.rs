use std::fmt::Display;

use colored::Colorize;
use mollie_api::resources::payments::{MolliePayment, MolliePaymentMode, MolliePaymentStatus};

#[derive(Debug)]
pub struct Payment {
    pub id: String,
    pub status: Status,
    pub amount: Amount,
    pub mode: Mode,
}

#[derive(Debug)]
pub enum Status {
    Open,
    Canceled,
    Pending,
    Authorized,
    Expired,
    Failed,
    Paid,
}

#[derive(Debug)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

#[derive(Debug)]
pub enum Mode {
    Live,
    Test,
}

impl Display for Payment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nid: {}\nstatus: {}\namount {}\nmode: {}",
            "Payment".to_uppercase().bright_blue(),
            self.id.bold().underline(),
            match self.status {
                Status::Open => "open".bright_blue(),
                Status::Canceled => "canceled".red(),
                Status::Pending => "pending".bright_blue(),
                Status::Authorized => "authorized".bright_blue(),
                Status::Expired => "expired".red(),
                Status::Failed => "failed".red(),
                Status::Paid => "paid".green(),
            },
            self.amount,
            match self.mode {
                Mode::Live => "live".to_uppercase(),
                Mode::Test => "test".to_uppercase(),
            },
        )
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.currency)
    }
}

impl From<MolliePayment> for Payment {
    fn from(payment: MolliePayment) -> Self {
        Self {
            id: payment.id,
            status: match payment.status {
                MolliePaymentStatus::Open => Status::Open,
                MolliePaymentStatus::Canceled => Status::Canceled,
                MolliePaymentStatus::Pending => Status::Pending,
                MolliePaymentStatus::Authorized => Status::Authorized,
                MolliePaymentStatus::Expired => Status::Expired,
                MolliePaymentStatus::Failed => Status::Failed,
                MolliePaymentStatus::Paid => Status::Paid,
            },
            amount: Amount {
                value: payment.amount.value,
                currency: payment.amount.currency,
            },
            mode: match payment.mode {
                MolliePaymentMode::Live => Mode::Live,
                MolliePaymentMode::Test => Mode::Test,
            },
        }
    }
}
