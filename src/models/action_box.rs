use time::OffsetDateTime;
use uuid::Uuid;

pub struct ActionBox {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub content: String, // Markdown
    pub durarion: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub finished_at: OffsetDateTime,
}

impl ActionBox {}
