use time::OffsetDateTime;
use uuid::Uuid;

enum ParentError {
    InvalidDateRange,
}
pub struct ParentBox {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub start_date: OffsetDateTime,
    pub end_date: OffsetDateTime,
    // pub children: Vec<Uuid>, // TODO - Implementar Children
    pub started: bool,
    pub finished: bool,
    pub archived: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ParentBox {
    fn new(
        name: String,
        description: Option<String>,
        start_date: OffsetDateTime,
        end_date: OffsetDateTime,
    ) -> Result<Self, ParentError> {
        let now: OffsetDateTime = OffsetDateTime::now_utc();
        if end_date >= start_date {
            Err(ParentError::InvalidDateRange)
        } else {
            Ok(Self {
                id: Uuid::new_v4(),
                name: name.to_string(),
                description,
                start_date,
                end_date,
                started: false,
                finished: false,
                archived: false,
                created_at: now,
                updated_at: now,
            })
        }
    }
}
