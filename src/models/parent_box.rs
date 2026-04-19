use crate::models::child_box::ChildBox;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct ParentBox {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub start_date: OffsetDateTime,
    pub end_date: OffsetDateTime,
    pub children: Vec<ChildBox>,
    pub started: bool,
    pub finished: bool,
    pub archived: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ParentBox {}
