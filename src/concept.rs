#[derive(Serialize, Deserialize)]
pub struct Concept<'a> {
    id: &'a str,
    concept_name: &'a str,
    catefory_id: i32,
    position: i32,
    language_id: i32,
    language_permalink: &'a str,
    concept_search: &'a str,
    concept_permalink: &'a str,
    description: &'a str,
    syntax: &'a str,
    notes: &'a str,
    example: &'a str,
    keywords: &'a str,
    related: &'a str,
    documentation: &'a str,
}
