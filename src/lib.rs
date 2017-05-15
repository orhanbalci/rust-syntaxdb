#[macro_use]
extern crate serde_derive;

extern crate reqwest;

pub use language::Language;
pub use concept::Concept;
pub use category::Category;

mod language;
mod concept;
mod category;

static BASE_API_URL: &'static str = "http://syntaxdb.com/api/v1";

pub struct SyntaxDbClient {
    client: reqwest::Client,
}

impl SyntaxDbClient {
    pub fn new() -> SyntaxDbClient {
        SyntaxDbClient { client: reqwest::Client::new().unwrap() }
    }

    pub fn get_languages(&self) -> Result<Vec<Language>, reqwest::Error> {
        let url = format!("{}/languages", BASE_API_URL);
        self.client.get(&url).send()?.json()
    }

    pub fn get_language_by_permalink<'a>(&self,
                                         language_permalink: &'a str)
                                         -> Result<Language, reqwest::Error> {
        let url = format!("{}/languages/{}", BASE_API_URL, language_permalink);
        self.client.get(&url).send()?.json()
    }

    pub fn get_language_categories<'a>(&self,
                                       language_permalink: &'a str)
                                       -> Result<Vec<Category>, reqwest::Error> {
        let url = format!("{}/languages/{}/categories",
                          BASE_API_URL,
                          language_permalink);
        self.client.get(&url).send()?.json()
    }

    pub fn get_language_category_concepts<'a>(&self,
                                              language_permalink: &'a str,
                                              category_id: i32)
                                              -> Result<Vec<Concept>, reqwest::Error> {
        let url = format!("{}/languages/{}/categories/{}/concepts",
                          BASE_API_URL,
                          language_permalink,
                          category_id);
        self.client.get(&url).send()?.json()
    }

    pub fn get_language_concepts<'a>(&self,
                                     language_permalink: &'a str)
                                     -> Result<Vec<Category>, reqwest::Error> {
        let url = format!("{}/languages/{}/concepts", BASE_API_URL, language_permalink);
        self.client.get(&url).send()?.json()
    }


    pub fn search_language_concepts<'a>(&self,
                                        language_permalink: &'a str,
                                        search: &'a str)
                                        -> Result<Vec<Concept>, reqwest::Error> {
        let url = format!("{}/languages/{}/concepts/search/{}",
                          BASE_API_URL,
                          language_permalink,
                          search);
        self.client.get(&url).send()?.json()

    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
