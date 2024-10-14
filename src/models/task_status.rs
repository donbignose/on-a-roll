use std::str::FromStr;
#[derive(Debug, Clone, Copy, PartialEq, Eq, diesel_derive_enum::DbEnum)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    InReview,
    Completed,
    OnHold,
    Canceled,
}

impl FromStr for TaskStatus {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Todo" => Ok(TaskStatus::Todo),
            "In Progress" => Ok(TaskStatus::InProgress),
            "Blocked" => Ok(TaskStatus::Blocked),
            "In Review" => Ok(TaskStatus::InReview),
            "Completed" => Ok(TaskStatus::Completed),
            "On Hold" => Ok(TaskStatus::OnHold),
            "Canceled" => Ok(TaskStatus::Canceled),
            _ => Err(format!("Invalid task status: {}", input)),
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            TaskStatus::Todo => "Todo",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Blocked => "Blocked",
            TaskStatus::InReview => "In Review",
            TaskStatus::Completed => "Completed",
            TaskStatus::OnHold => "On Hold",
            TaskStatus::Canceled => "Canceled",
        };
        write!(f, "{}", status_str)
    }
}
