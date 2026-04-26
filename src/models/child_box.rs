use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub enum ChildError {
    ChildWithoutParent,
}

#[derive(Debug)]
pub struct ChildBox {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    // pub duration: Duration, TODO - Duração será calculada através do tempo das Actions + Descansos pomodoro
    // pub actions: Vec<Uuid>, TODO - Implementar Actions
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ChildBox {
    pub fn new(
        parent_id: Uuid,
        name: String,
        description: Option<String>,
    ) -> Result<Self, ChildError> {
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        if parent_id != id {
            // TODO - Check se já existe pai, usando id válido
            Err(ChildError::ChildWithoutParent)
        } else {
            Ok(Self {
                id,
                parent_id,
                name,
                description,
                created_at: now,
                updated_at: now,
            })
        }
    }
}
