#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate url;

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

    pub fn get_language_concepts<'a>(&self,
                                     language_permalink: &'a str,
                                     fields: Option<Vec<&str>>,
                                     limit: Option<i32>,
                                     sort: Option<&str>)
                                     -> Result<Vec<Category>, reqwest::Error> {
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


    pub fn search_language_concepts<'a>(&self,
                                        language_permalink: &'a str,
                                        search: &'a str,
                                        fields: Option<Vec<&str>>,
                                        limit: Option<i32>)
                                        -> Result<Vec<Concept>, reqwest::Error> {
        let base_url = format!("{}/languages/{}/concepts/search/{}",
                               BASE_API_URL,
                               language_permalink,
                               search);
        let mut url = url::Url::parse(&base_url).unwrap();
        if let Some(f) = fields {
            url.query_pairs_mut().append_pair("fields", &f.join(","));
        };

        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        };


        self.client.get(url.as_str()).send()?.json()

    }

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

    pub fn get_language_concepts_by_concept_id(&self,
                                               language_permalink: &str,
                                               concept_permalink: &str,
                                               fields: Option<Vec<&str>>)
                                               -> Result<Vec<Concept>, reqwest::Error> {
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
        assert_eq!(languages[0].language_name, "C");
    }

    #[test]
    fn get_language_by_permalink() {
        let client = SyntaxDbClient::new();
        let languages = client
            .get_languages(None, Some(1), Some("language_name"))
            .unwrap();
        let language = client
            .get_language_by_permalink(&languages[0].language_permalink, None)
            .unwrap();
        assert_eq!(language.id, 2);
    }
}
