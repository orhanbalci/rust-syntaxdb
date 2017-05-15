#[derive(Serialize, Deserialize)]
pub struct Category {
    id: i32,
    category_name: String,
    category_search: String,
    language_id: String,
    language_permalink: String,
    position: i32,
}
