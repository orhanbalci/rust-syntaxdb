#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i32>,
    pub category_name: Option<String>,
    pub category_search: Option<String>,
    pub language_id: Option<i32>,
    pub language_permalink: Option<String>,
    pub position: Option<i32>,
}
