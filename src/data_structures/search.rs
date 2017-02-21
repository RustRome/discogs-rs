// Copyright (C) 2016  Afonso Bordado <afonsobordado@az8.co>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use data_structures::*;
use query::*;
use serde_json;
use std::collections::HashMap;
use itertools::Itertools;

/// The default API Endpoint
const SEARCH_ENDPOINT: &'static str = "/database";


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SearchResult  {
    Artist {
        data: Artist
    },
    Master {
        data: Master
    },
    Label {
        data: Label
    },
    Release {
        data: Release
    }
}

pub enum SearchType {
    Artist,
    Master,
    Label,
    Release
}

#[derive(Deserialize, Debug)]
struct SearchResultFull {
    pub pagination: Pagination,
    pub results: Vec<SearchResult>,
}


impl SearchType {
    pub fn to_string(&self) -> String {
        match *self {
            SearchType::Artist => "artist".to_string(),
            SearchType::Master => "master".to_string(),
            SearchType::Label => "label".to_string(),
            SearchType::Release => "release".to_string()
        }
    }
}

pub struct SearchQueryBuilder {
    api_endpoint: String,
    user_agent: String,

    /// The key and secret are required, however to keep
    /// a consistent styling across multiple QueryBuilders
    /// we wont require them on `new()`
    ///
    /// If none is provided the query will fail with a
    /// `QueryError::AuthenticationMissingError`
    key: Option<String>,
    secret: Option<String>,

    parameters: HashMap<String, String>,
}


impl SearchQueryBuilder {
    /// Creates a new instance of `SearchQueryBuilder`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let sqb = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                   env!("DISCOGS_USER_AGENT").to_string());
    /// ```
    pub fn new(api_endpoint: String, user_agent: String) -> SearchQueryBuilder {
        SearchQueryBuilder {
            api_endpoint: api_endpoint,
            user_agent: user_agent,
            key: None,
            secret: None,
            parameters: HashMap::new(),
        }
    }

    /// Set the query text to be sent in the query
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.query("query".to_string());
    /// ```
    pub fn query(&mut self, query: String) -> &mut Self {
        self.parameters.insert("query".to_string(), query);
        self
    }

    /// Set the search type of the query
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    /// use discogs::data_structures::SearchType;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.search_type(SearchType::Artist);
    /// ```
    pub fn search_type(&mut self, search_type: SearchType) -> &mut Self {
        self.parameters.insert("search_type".to_string(), search_type.to_string());
        self
    }


    /// Set the year in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.year("year".to_string());
    /// ```
    pub fn year(&mut self, year: String) -> &mut Self {
        self.parameters.insert("year".to_string(), year);
        self
    }

    /// Set the title in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.title("title".to_string());
    /// ```
    pub fn title(&mut self, title: String) -> &mut Self {
        self.parameters.insert("title".to_string(), title);
        self
    }

    /// Set the release_title in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.release_title("release_title".to_string());
    /// ```
    pub fn release_title(&mut self, release_title: String) -> &mut Self {
        self.parameters.insert("release_title".to_string(), release_title);
        self
    }

    /// Set the credit in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.credit("credit".to_string());
    /// ```
    pub fn credit(&mut self, credit: String) -> &mut Self {
        self.parameters.insert("credit".to_string(), credit);
        self
    }

    /// Set the artist in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.artist("artist".to_string());
    /// ```
    pub fn artist(&mut self, artist: String) -> &mut Self {
        self.parameters.insert("artist".to_string(), artist);
        self
    }

    /// Set the anv in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.anv("anv".to_string());
    /// ```
    pub fn anv(&mut self, anv: String) -> &mut Self {
        self.parameters.insert("anv".to_string(), anv);
        self
    }

    /// Set the label in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.label("label".to_string());
    /// ```
    pub fn label(&mut self, label: String) -> &mut Self {
        self.parameters.insert("label".to_string(), label);
        self
    }

    /// Set the genre in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.genre("genre".to_string());
    /// ```
    pub fn genre(&mut self, genre: String) -> &mut Self {
        self.parameters.insert("genre".to_string(), genre);
        self
    }

    /// Set the style in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.style("style".to_string());
    /// ```
    pub fn style(&mut self, style: String) -> &mut Self {
        self.parameters.insert("style".to_string(), style);
        self
    }

    /// Set the country in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.country("country".to_string());
    /// ```
    pub fn country(&mut self, country: String) -> &mut Self {
        self.parameters.insert("country".to_string(), country);
        self
    }
    /// Set the format in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.format("format".to_string());
    /// ```
    pub fn format(&mut self, format: String) -> &mut Self {
        self.parameters.insert("format".to_string(), format);
        self
    }

    /// Set the catno in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.catno("catno".to_string());
    /// ```
    pub fn catno(&mut self, catno: String) -> &mut Self {
        self.parameters.insert("catno".to_string(), catno);
        self
    }

    /// Set the barcode in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.barcode("barcode".to_string());
    /// ```
    pub fn barcode(&mut self, barcode: String) -> &mut Self {
        self.parameters.insert("barcode".to_string(), barcode);
        self
    }

    /// Set the track in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.track("track".to_string());
    /// ```
    pub fn track(&mut self, track: String) -> &mut Self {
        self.parameters.insert("track".to_string(), track);
        self
    }

    /// Set the submitter in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.submitter("submitter".to_string());
    /// ```
    pub fn submitter(&mut self, submitter: String) -> &mut Self {
        self.parameters.insert("submitter".to_string(), submitter);
        self
    }

    /// Set the contributor in the query to be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let mut search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// search.contributor("contributor".to_string());
    /// ```
    pub fn contributor(&mut self, contributor: String) -> &mut Self {
        self.parameters.insert("contributor".to_string(), contributor);
        self
    }

    /// Perform request
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::SearchQueryBuilder;
    ///
    /// let search = SearchQueryBuilder::new(discogs::API_URL.to_string(),
    ///                                      env!("DISCOGS_USER_AGENT").to_string())
    ///                                       .get();
    /// ```
    pub fn get(&self) -> Result<Vec<SearchResult>, QueryError> {
        let result: Result<String, QueryError> = self.perform_request();

        if let Err(error) = result {
            return Err(error);
        } else {
            let result_string = result.ok().unwrap();
            let result: Result<SearchResultFull, serde_json::Error> =
                serde_json::from_str(&result_string);

            if let Ok(srf) = result {
                return Ok(srf.results);
            } else {
                return Err(QueryError::JsonDecodeError {
                    serde_err: result.err()
                });
            }

        }
    }
}

impl QueryBuilder for SearchQueryBuilder {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn get_secret(&self) -> Option<String> {
        self.secret.clone()
    }

    //api.discogs.com/database/search?q= is a valid query, so is
    //api.discogs.com/database/search?q=&year=1
    fn get_query_url(&self) -> String {
        let mut url = format!("{}{}/search?q=", self.api_endpoint, SEARCH_ENDPOINT);

        if self.parameters.is_empty() {
            return url;
        }

        url.push_str(self.parameters["query"].as_str());

        if self.parameters.len() > 1 {
            url.push_str("&");

            url.push_str(
                self.parameters.iter()
                               .filter(|&(p, _)| p != "query")
                               .map(|(p, v)| format!("{}={},", p, v))
                               .intersperse(",".to_string())
                               .collect::<String>()
                               .as_str());
        }

        url
    }

    fn get_user_agent(&self) -> String {
        self.user_agent.clone()
    }
}
