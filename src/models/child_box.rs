use time::{Duration, OffsetDateTime};
use uuid::Uuid;

pub struct ChildBox {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub duration: Duration,
    pub actions: Vec<Uuid>,
    pub started: bool,
    pub finished: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ChildBox {}
