#[derive(Serialize, Deserialize)]
pub struct Concept {
    id: String,
    concept_name: String,
    catefory_id: i32,
    position: i32,
    language_id: i32,
    language_permalink: String,
    concept_search: String,
    concept_permalink: String,
    description: String,
    syntax: String,
    notes: String,
    example: String,
    keywords: String,
    related: String,
    documentation: String,
}
