use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, diesel_derive_enum::DbEnum)]
pub enum ProjectStatus {
    Planning,
    Active,
    OnHold,
    Blocked,
    InReview,
    Completed,
    Canceled,
}

impl FromStr for ProjectStatus {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Planning" => Ok(ProjectStatus::Planning),
            "Active" => Ok(ProjectStatus::Active),
            "On Hold" => Ok(ProjectStatus::OnHold),
            "Blocked" => Ok(ProjectStatus::Blocked),
            "In Review" => Ok(ProjectStatus::InReview),
            "Completed" => Ok(ProjectStatus::Completed),
            "Canceled" => Ok(ProjectStatus::Canceled),
            _ => Err(format!("Invalid project status: {}", input)),
        }
    }
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            ProjectStatus::Planning => "Planning",
            ProjectStatus::Active => "Active",
            ProjectStatus::OnHold => "On Hold",
            ProjectStatus::Blocked => "Blocked",
            ProjectStatus::InReview => "In Review",
            ProjectStatus::Completed => "Completed",
            ProjectStatus::Canceled => "Canceled",
        };
        write!(f, "{}", status_str)
    }
}
