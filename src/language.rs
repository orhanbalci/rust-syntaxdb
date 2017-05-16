#[derive(Serialize, Deserialize)]
pub struct Language {
    pub id: i32,
    pub language_name: String,
    pub position: i32,
    pub language_version: String,
    pub language_permalink: String,
}
