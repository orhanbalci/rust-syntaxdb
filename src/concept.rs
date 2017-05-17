#[derive(Serialize, Deserialize)]
pub struct Concept {
    pub id: Option<i32>,
    pub concept_name: Option<String>,
    pub category_id: Option<i32>,
    pub position: Option<i32>,
    pub language_id: Option<i32>,
    pub language_permalink: Option<String>,
    pub concept_search: Option<String>,
    pub concept_permalink: Option<String>,
    pub description: Option<String>,
    pub syntax: Option<String>,
    pub notes: Option<String>,
    pub example: Option<String>,
    pub keywords: Option<String>,
    pub related: Option<String>,
    pub documentation: Option<String>,
}
