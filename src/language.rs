#[derive(Serialize, Deserialize)]
pub struct Language {
    pub id: Option<i32>,
    pub language_name: Option<String>,
    pub position: Option<i32>,
    pub language_version: Option<String>,
    pub language_permalink: Option<String>,
}
