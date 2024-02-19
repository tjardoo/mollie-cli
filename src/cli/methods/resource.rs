use std::fmt::Display;

use colored::Colorize;
use mollie_api::resources::methods::{MollieMethod, MollieMethodStatus};

#[derive(Debug)]
pub struct Method {
    pub id: String,
    pub description: String,
    pub status: Status,
}

#[derive(Debug)]
pub enum Status {
    Activated,
    PendingBoarding,
    PendingReview,
    PendingExternal,
    Rejected,
    NotRequested,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nid: {}\ndescription: {}\nstatus: {}",
            "Method".to_uppercase().bright_blue(),
            self.id.bold().underline(),
            self.description,
            match self.status {
                Status::Activated => "activated".green(),
                Status::PendingBoarding => "pending boarding".yellow(),
                Status::PendingReview => "pending review".yellow(),
                Status::PendingExternal => "pending external".yellow(),
                Status::Rejected => "rejected".red(),
                Status::NotRequested => "not requested".red(),
            }
        )
    }
}

impl From<MollieMethod> for Method {
    fn from(method: MollieMethod) -> Self {
        Self {
            id: method.id,
            description: method.description,
            status: match method.status {
                Some(MollieMethodStatus::Activated) => Status::Activated,
                Some(MollieMethodStatus::PendingBoarding) => Status::PendingBoarding,
                Some(MollieMethodStatus::PendingReview) => Status::PendingReview,
                Some(MollieMethodStatus::PendingExternal) => Status::PendingExternal,
                Some(MollieMethodStatus::Rejected) => Status::Rejected,
                None => Status::NotRequested,
            },
        }
    }
}
