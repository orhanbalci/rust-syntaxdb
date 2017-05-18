//! This library is a wrapper around SyntaxDB rest API. 
//! SyntaxDB allows users to quickly look up syntax for programming languages. SyntaxDB is designed
//! for programmers who often need to do a Google search for their syntax needs. Primary usage area
//! for SyntaxDB is editor and IDE integrations. This library helps you if you plan to write an
//! extension for an editor using Rust
#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate url;

pub use language::Language;
pub use concept::Concept;
pub use category::Category;

pub mod language;
pub mod concept;
pub mod category;

static BASE_API_URL: &'static str = "http://syntaxdb.com/api/v1";

pub struct SyntaxDbClient {
    client: reqwest::Client,
}

impl SyntaxDbClient {
    pub fn new() -> SyntaxDbClient {
        SyntaxDbClient { client: reqwest::Client::new().unwrap() }
    }

    /// The Languages endpoint returns all languages available on SyntaxCenter, sorted by position
    /// by default.
    pub fn get_languages(&self,
                         fields: Option<Vec<&str>>,
                         limit: Option<i32>,
                         sort: Option<&str>)
                         -> Result<Vec<Language>, reqwest::Error> {
        let url_base = format!("{}/languages", BASE_API_URL);
        let mut url = url::Url::parse(&url_base).unwrap();

        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("query", &f.join(","));
        };

        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());

        };

        if let Some(s) = sort {
            url.query_pairs_mut().append_pair("sort", &s);
        };

        self.client.get(url.as_str()).send()?.json()
    }

    /// Returns the language corresponding to the permalink along with its information.
    pub fn get_language_by_permalink<'a>(&self,
                                         language_permalink: &'a str,
                                         fields: Option<Vec<&str>>)
                                         -> Result<Language, reqwest::Error> {
        let url_base = format!("{}/languages/{}", BASE_API_URL, language_permalink);
        let mut url = url::Url::parse(&url_base).unwrap();
        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("fields", &f.join(","));
        };

        self.client.get(url.as_str()).send()?.json()
    }

    /// The language categories endpoint returns all of the categories corresponding to the
    /// language.
    pub fn get_language_categories<'a>(&self,
                                       language_permalink: &'a str,
                                       fields: Option<Vec<&str>>,
                                       limit: Option<i32>,
                                       sort: Option<&str>)
                                       -> Result<Vec<Category>, reqwest::Error> {
        let base_url = format!("{}/languages/{}/categories",
                               BASE_API_URL,
                               language_permalink);
        let mut url = url::Url::parse(&base_url).unwrap();
        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("fields", &f.join(","));
        };

        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        };

        if let Some(s) = sort {
            url.query_pairs_mut().append_pair("sort", &s);
        };

        self.client.get(url.as_str()).send()?.json()
    }

    /// The category concepts endpoint returns all concepts belonging to the category, sorted by
    /// position by default. 
    pub fn get_language_category_concepts<'a>(&self,
                                              language_permalink: &'a str,
                                              category_id: i32,
                                              fields: Option<Vec<&str>>,
                                              limit: Option<i32>,
                                              sort: Option<&str>)
                                              -> Result<Vec<Concept>, reqwest::Error> {
        let base_url = format!("{}/languages/{}/categories/{}/concepts",
                               BASE_API_URL,
                               language_permalink,
                               category_id);
        let mut url = url::Url::parse(&base_url).unwrap();
        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("fields", &f.join(","));
        };

        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        };

        if let Some(s) = sort {
            url.query_pairs_mut().append_pair("sort", &s);
        };

        self.client.get(url.as_str()).send()?.json()
    }

    /// The language concepts endpoint returns all concepts belonging to the language, sorted by
    /// position by default. 
    pub fn get_language_concepts<'a>(&self,
                                     language_permalink: &'a str,
                                     fields: Option<Vec<&str>>,
                                     limit: Option<i32>,
                                     sort: Option<&str>)
                                     -> Result<Vec<Concept>, reqwest::Error> {
        let base_url = format!("{}/languages/{}/concepts", BASE_API_URL, language_permalink);
        let mut url = url::Url::parse(&base_url).unwrap();
        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("fields", &f.join(","));
        };

        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        };

        if let Some(s) = sort {
            url.query_pairs_mut().append_pair("sort", &s);
        };

        self.client.get(url.as_str()).send()?.json()
    }

    /// This search language concepts endpoint returns all concepts within the specified language
    /// that matches the query.
    pub fn search_language_concepts<'a>(&self,
                                        language_permalink: &'a str,
                                        search: &'a str,
                                        fields: Option<Vec<&str>>,
                                        limit: Option<i32>)
                                        -> Result<Vec<Concept>, reqwest::Error> {
        let base_url = format!("{}/languages/{}/concepts/search",
                               BASE_API_URL,
                               language_permalink);
        let mut url = url::Url::parse_with_params(&base_url, &[("q", search)]).unwrap();
        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("fields", &f.join(","));
        };

        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        };


        self.client.get(url.as_str()).send()?.json()

    }

    /// The Concepts endpoint returns an array of concepts contained in SyntaxCenter. By default,
    /// the array is limited to 20 concepts, sorted by ID in ascending order. 
    pub fn get_concepts(&self,
                        fields: Option<Vec<&str>>,
                        limit: Option<i32>,
                        sort: Option<&str>)
                        -> Result<Vec<Concept>, reqwest::Error> {
        let base_url = format!("{}/concepts", BASE_API_URL);
        let mut url = url::Url::parse(&base_url).unwrap();
        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("fields", &f.join(","));
        };

        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        };

        if let Some(s) = sort {
            url.query_pairs_mut().append_pair("sort", &s);
        };

        self.client.get(url.as_str()).send()?.json()
    }

    /// Full text search for matching concepts across the entire database.
    pub fn search_concepts<'a>(&self,
                               search: &'a str,
                               fields: Option<Vec<&str>>,
                               limit: Option<i32>)
                               -> Result<Vec<Concept>, reqwest::Error> {
        let base_url = format!("{}/concepts/search", BASE_API_URL);
        let mut url = url::Url::parse_with_params(&base_url, &[("q", search)]).unwrap();
        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("fields", &f.join(","));
        };

        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        };

        self.client.get(url.as_str()).send()?.json()
    }
    
    /// Returns the concept corresponding to the unique identifier.
    pub fn get_concept_by_id(&self,
                             concept_id: i32,
                             fields: Option<Vec<&str>>)
                             -> Result<Concept, reqwest::Error> {
        let base_url = format!("{}/concepts/{}", BASE_API_URL, concept_id);
        let mut url = url::Url::parse(&base_url).unwrap();
        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("fields", &f.join(","));
        }
        self.client.get(url.as_str()).send()?.json()
    }

    /// The language concepts endpoint returns all concepts belonging to the language, sorted by
    /// position by default. 
    pub fn get_language_concepts_by_concept_id(&self,
                                               language_permalink: &str,
                                               concept_permalink: &str,
                                               fields: Option<Vec<&str>>)
                                               -> Result<Concept, reqwest::Error> {
        let base_url = format!("{}/languages/{}/concepts/{}",
                               BASE_API_URL,
                               language_permalink,
                               concept_permalink);
        let url = match fields {
            Some(f) => url::Url::parse_with_params(&base_url, &[("fileds", &f.join(","))]),
            None => url::Url::parse(&base_url),
        };
        self.client.get(url.unwrap().as_str()).send()?.json()
    }
}

#[cfg(test)]
mod tests {
    use super::SyntaxDbClient;
    #[test]
    fn get_languages() {
        let client = SyntaxDbClient::new();
        let languages = client
            .get_languages(Some(vec!["id", "language_name"]),
                           None,
                           Some("language_name"))
            .unwrap();
        if let Some(ref ln) = languages[0].language_name {
            assert_eq!(ln, "C");
        }
    }

    #[test]
    fn get_language_by_permalink() {
        let client = SyntaxDbClient::new();
        let languages = client
            .get_languages(None, Some(1), Some("language_name"))
            .unwrap();
        if let Some(ref lp) = languages[0].language_permalink {
            let language = client.get_language_by_permalink(lp, None).unwrap();
            assert_eq!(language.id.unwrap(), 2);
        }
    }

    #[test]
    fn get_language_categories() {
        let client = SyntaxDbClient::new();
        let categories = client
            .get_language_categories("java", None, None, None)
            .unwrap();
        if let Some(ref cn) = categories[1].category_name {
            assert_eq!(cn, "Variables");
        }
    }

    #[test]
    fn get_language_category_concepts() {

        let client = SyntaxDbClient::new();
        let concepts = client
            .get_language_category_concepts("java", 2, None, None, None)
            .unwrap();
        if let Some(ref cn) = concepts[0].concept_name {
            assert_eq!(cn, "Primitive Data Types");
        }
    }

    #[test]
    fn get_language_concepts() {
        let client = SyntaxDbClient::new();
        let concepts = client
            .get_language_concepts("java", None, None, None)
            .unwrap();
        if let Some(ref cn) = concepts[0].concept_name {
            assert_eq!(cn, "Print");
        }
    }

    #[test]
    fn search_language_concepts() {
        let client = SyntaxDbClient::new();
        let concepts = client
            .search_language_concepts("java", "map", None, None)
            .unwrap();
        if let Some(ref cn) = concepts[0].concept_name {
            assert_eq!(cn, "HashMaps");
        }
    }

    #[test]
    fn get_concepts() {
        let client = SyntaxDbClient::new();
        let concepts = client.get_concepts(None, None, None).unwrap();
        if let Some(ref cn) = concepts[0].concept_name {
            assert_eq!(cn, "Print");
        }
    }

    #[test]
    fn search_concepts() {
        let client = SyntaxDbClient::new();
        let concepts = client.search_concepts("variable", None, None).unwrap();
        if let Some(ref cn) = concepts[0].concept_name {
            assert_eq!(cn, "Variable Declaration");
        }
    }

    #[test]
    fn get_concept_by_id() {
        let client = SyntaxDbClient::new();
        let concept = client.get_concept_by_id(12, None).unwrap();
        if let Some(ref cn) = concept.concept_name {
            assert_eq!(cn, "While Loop");
        }
    }

    #[test]
    fn get_language_concepts_by_concept_id() {
        let client = SyntaxDbClient::new();
        let concept = client
            .get_language_concepts_by_concept_id("java", "print", None)
            .unwrap();
        if let Some(ref cn) = concept.concept_name {
            assert_eq!(cn, "Print");
        }
    }
}
