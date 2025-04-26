use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
}

impl DateRange {
    pub fn new(start_date: Option<Date>, end_date: Option<Date>) -> Self {
        Self {
            start_date,
            end_date,
        }
    }

    pub fn is_valid(&self) -> bool {
        match (self.start_date, self.end_date) {
            (Some(start), Some(end)) => start <= end,
            _ => true,
        }
    }
}
