use crate::models::action_box::ActionBox;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct ChildBox {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub duration: OffsetDateTime, // Isso aqui vai ser calculado depois, esse datetime e so um pass
    pub actions: Vec<ActionBox>,
    pub started: bool,
    pub finished: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ChildBox {}
