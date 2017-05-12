#[derive(Serialize, Deserialize)]
pub struct Category<'a>{
    id : i32,
    category_name : &'a str,
    category_search : &'a str,
    language_id : &'a str,
    language_permalink : &'a str,
    position : i32,
}
