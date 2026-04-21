use uuid::Uuid;

/// Generates a new UUID for todo identifiers.
pub fn new_todo_id() -> Uuid {
    Uuid::new_v4()
}
