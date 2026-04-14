use chrono::{DateTime, Utc};
use sqlx::FromRow;
#[derive(FromRow, Debug, Clone)]
pub struct Todo {
    id: u8,
    title: String,
    description:String,
    status:Status,
    createdAt:DateTime<Utc>,
    updatedAt:DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "Status")]
pub enum Status{
    is_pending,
    is_done,
    not_started,
}

