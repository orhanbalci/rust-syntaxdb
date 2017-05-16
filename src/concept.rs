#[derive(Serialize, Deserialize)]
pub struct Concept {
    pub id: String,
    pub concept_name: String,
    pub catefory_id: i32,
    pub position: i32,
    pub language_id: i32,
    pub language_permalink: String,
    pub concept_search: String,
    pub concept_permalink: String,
    pub description: String,
    pub syntax: String,
    pub notes: String,
    pub example: String,
    pub keywords: String,
    pub related: String,
    pub documentation: String,
}
