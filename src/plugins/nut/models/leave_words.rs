use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: i64,
    pub body: String,
    pub media_type: String,
    pub created_at: NaiveDateTime,
}
