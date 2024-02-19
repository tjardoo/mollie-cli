use std::fmt::Display;

use colored::Colorize;
use mollie_api::resources::payments::{MolliePayment, MolliePaymentMode, MolliePaymentStatus};

#[derive(Debug)]
pub struct Payment {
    pub id: String,
    pub mode: Mode,
    pub status: Status,
}

#[derive(Debug)]
pub enum Mode {
    Live,
    Test,
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

impl Display for Payment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nid: {}\nmode: {}\nstatus: {}",
            "-- Payment --".to_uppercase().bright_blue(),
            self.id.bold().underline(),
            match self.mode {
                Mode::Live => "live".to_uppercase(),
                Mode::Test => "test".to_uppercase(),
            },
            match self.status {
                Status::Open => "open".white().on_bright_blue(),
                Status::Canceled => "canceled".white().on_red(),
                Status::Pending => "pending".white().on_bright_blue(),
                Status::Authorized => "authorized".white().on_bright_blue(),
                Status::Expired => "expired".white().on_red(),
                Status::Failed => "failed".white().on_red(),
                Status::Paid => "paid".white().on_green(),
            }
        )
    }
}

impl From<MolliePayment> for Payment {
    fn from(payment: MolliePayment) -> Self {
        Self {
            id: payment.id,
            mode: match payment.mode {
                MolliePaymentMode::Live => Mode::Live,
                MolliePaymentMode::Test => Mode::Test,
            },
            status: match payment.status {
                MolliePaymentStatus::Open => Status::Open,
                MolliePaymentStatus::Canceled => Status::Canceled,
                MolliePaymentStatus::Pending => Status::Pending,
                MolliePaymentStatus::Authorized => Status::Authorized,
                MolliePaymentStatus::Expired => Status::Expired,
                MolliePaymentStatus::Failed => Status::Failed,
                MolliePaymentStatus::Paid => Status::Paid,
            },
        }
    }
}
