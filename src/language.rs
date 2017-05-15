#[derive(Serialize, Deserialize)]
pub struct Language {
    id: i32,
    language_name: String,
    position: i32,
    language_version: String,
    language_permalink: String,
}
