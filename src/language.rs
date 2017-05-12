#[derive(Serialize, Deserialize)]
pub struct Language<'a> {
    id: i32,
    language_name: &'a str,
    position: i32,
    language_version: &'a str,
    language_permalink: &'a str,
}
