#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub category_name: String,
    pub category_search: String,
    pub language_id: String,
    pub language_permalink: String,
    pub position: i32,
}
