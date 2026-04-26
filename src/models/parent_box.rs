use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug)]
pub enum ParentError {
    InvalidDateRange, // Vai ter mais coisa aqui, por enquanto e so isso
}

#[derive(Debug)]
pub struct ParentBox {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub start_date: OffsetDateTime,
    pub end_date: OffsetDateTime,
    // pub children: Vec<Uuid>, // TODO - Implementar Children
    pub archived: bool, // TODO - Implementar Archived At e Archived Reason
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ParentBox {
    pub fn new(
        name: String,
        description: Option<String>,
        start_date: OffsetDateTime,
        end_date: OffsetDateTime,
    ) -> Result<Self, ParentError> {
        let now: OffsetDateTime = OffsetDateTime::now_utc();
        if end_date <= start_date {
            Err(ParentError::InvalidDateRange)
        } else {
            Ok(Self {
                id: Uuid::new_v4(),
                name,
                description,
                start_date,
                end_date,
                archived: false,
                created_at: now,
                updated_at: now,
            })
        }
    }

    pub fn available_time(&self) -> Duration {
        self.end_date - self.start_date
    }

    pub fn can_fit(&self, child_duration: Duration) {
        todo!("Implementar quanto existir Children")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Duration, OffsetDateTime};

    #[test]
    fn test_valid_parent_creation() {
        let start = OffsetDateTime::now_utc();
        let end = start + Duration::days(7);

        let parent = ParentBox::new(
            "Hello TickBox v1!".to_string(),
            Option::default(),
            start,
            end,
        );

        assert!(parent.is_ok());
        let p = parent.unwrap();
        assert_eq!(p.name, "Hello TickBox v1!");
        assert!(!p.archived);
    }

    #[test]
    fn test_invalid_date_range() {
        let start = OffsetDateTime::now_utc();
        let end = start - Duration::days(1);

        let parent = ParentBox::new(
            "Hello TickBox Err!".to_string(),
            Option::default(),
            start,
            end,
        );

        assert!(parent.is_err());
        matches!(parent.unwrap_err(), ParentError::InvalidDateRange);
    }
}
