use chrono::{DateTime, Utc};

pub struct Todo {
    id: u8,
    title: String,
    description:String,
    status:Status,
    createdAt:DateTime<Utc>,
    updatedAt:DateTime<Utc>,
}

enum Status{
    is_pending,
    is_done,
    not_started,
}

