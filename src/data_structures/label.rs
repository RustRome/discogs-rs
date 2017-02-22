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

/// The default host address for the API.
const LABEL_ENDPOINT: &'static str = "/labels";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    pub id: u32,
    pub resource_url: String,
    pub name: String,
    pub profile: Option<String>,
    pub releases_url: Option<String>,
    pub contact_info: Option<String>,
    pub uri: Option<String>,
    pub urls: Option<Vec<String>>,
    pub data_quality: Option<DataQuality>,
    pub sublabels: Option<Vec<Label>>,
}

impl Label {
    /// Creates a new instance of `Label`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::Label;
    ///
    /// let label = Label::new(4567,
    ///                        "name".to_string(),
    ///                        "resource_url".to_string());
    /// ```
    pub fn new(id: u32, name: String, resource_url: String) -> Label {
         Label {
            id: id,
            name: name,
            resource_url: resource_url,
            profile: None,
            releases_url: None,
            contact_info: None,
            uri: None,
            urls: None,
            data_quality: None,
            sublabels: None,
        }
    }

}

// TODO: make a more comprehensive test
impl PartialEq for Label {
    fn eq(&self, other: &Label) -> bool {
        other.id == self.id
    }
}

pub struct LabelQueryBuilder {
    //artist id
    id: u32,

    api_endpoint: String,
    user_agent: String,

    // Optional key and secret if necessary
    key: Option<String>,
    secret: Option<String>,
    page: i16,
    per_page: i16,
    releases : bool
}

impl LabelQueryBuilder {
    /// Creates a new instance of `LabelQueryBuilder`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::LabelQueryBuilder;
    ///
    /// let lqb = LabelQueryBuilder::new(4567,
    ///                                   discogs::API_URL.to_string(),
    ///                                   "USER_AGENT".to_string(),
    ///                                   Some("CLIENT_KEY".to_string()),
    ///                                   Some("CLIENT_SECRET".to_string()));
    /// ```
    pub fn new(id: u32,
               api_endpoint: String,
               user_agent: String,
               key: Option<String>,
               secret: Option<String>) -> LabelQueryBuilder {
        LabelQueryBuilder {
            id: id,
            api_endpoint: api_endpoint,
            user_agent: user_agent,
            key: key,
            secret:secret,
            page  : 1,
            per_page : 50,
            releases : false
        }
    }

    /// Perform request
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use discogs::Discogs;
    ///
    /// let label = Discogs::new("USER_AGENT")
    ///                       .label(1234)
    ///                       .get();
    /// ```
    pub fn get(&self) -> Result<Label, QueryError> {
        let result: Result<String, QueryError> = self.perform_request();

        if let Err(error) = result {
            return Err(error);
        } else {
            let result_string = result.ok().unwrap();
            let json = serde_json::from_str(&result_string);

            if let Ok(artist) = json {
                return Ok(artist);
            } else {
                return Err(QueryError::JsonDecodeError {
                    serde_err: json.err()
                });
            }
        }
    }

    pub fn pagination(&mut self, page: i16, per_page: i16) -> &mut LabelQueryBuilder {
        self.page = page;
        self.per_page = per_page;
        self
    }
    /// Perform request for Label Releases
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use discogs::Discogs;
    ///
    /// let releases = Discogs::new("USER_AGENT")
    ///                       .label(4567)
    ///                       .get_releases();
    /// ```
    pub fn get_releases(&mut self) -> Result<ArtistReleases, QueryError> {

        self.releases = true;

        let result: Result<String, QueryError> = self.perform_request();

        if let Err(error) = result {
            return Err(error);
        } else {
            let result_string = result.ok().unwrap();
            let json = serde_json::from_str(&result_string);

            if let Ok(label) = json {
                return Ok(label);
            } else {
                return Err(QueryError::JsonDecodeError {
                    serde_err: json.err()
                });
            }
        }
    }
}

impl QueryBuilder for LabelQueryBuilder {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn get_secret(&self) -> Option<String> {
        self.secret.clone()
    }

    fn get_query_url(&self) -> String {
        match self.releases {
            false => format!("{}{}/{}", self.api_endpoint, LABEL_ENDPOINT, self.id),
            true => format!("{}{}/{}/releases?page={}&per_page={}", self.api_endpoint, LABEL_ENDPOINT, self.id, self.page, self.per_page)
        }
    }

    fn get_user_agent(&self) -> String {
        self.user_agent.clone()
    }
}

#[cfg(test)]
mod tests {
    use discogs::*;
    use data_structures::*;
    use mockito::mock;
    use serde_json;
    use serde_json::to_string;

    fn lqb(id: u32) -> LabelQueryBuilder {
        Discogs::new("USER_AGENT").label(id)
    }

    #[test]
    fn test_request_builder() {
        let qb = lqb(999);

        assert_eq!(qb.id, 999);
        assert_eq!(qb.api_endpoint, API_URL.to_owned());
        assert_eq!(qb.user_agent, "USER_AGENT".to_string());
        assert!(qb.key.is_none());
        assert!(qb.secret.is_none());
    }

    #[test]
    fn test_label_new() {
        let label = Label::new(1234,
                               "name".to_string(),
                               "resource_url".to_string());

        assert_eq!(label.id, 1234);
        assert_eq!(label.name, "name".to_string());
        assert_eq!(label.resource_url, "resource_url".to_string());
    }

    #[test]
    fn test_label_eq() {
        let label = Label::new(1234,
                               "name".to_string(),
                               "resource_url".to_string());
        let label2 = Label::new(1234,
                                "adsh".to_string(),
                                "dasdas".to_string());

        assert!(label == label2);
    }

    #[test]
    fn test_perform_label_request() {
        mock("GET", "/labels/1234")
            .with_status(200)
            .with_header("content-type", "text/json")
            .with_body(to_string(&json!({
                "id": 1234,
                "resource_url": "https://api.discogs.com/labels/1234",
                "name": "Skunkworks"
            })).unwrap().as_str())
            .create_for(|| {
                let label = Discogs::new("USER_AGENT")
                    .label(1234)
                    .get()
                    .ok()
                    .unwrap();

                assert_eq!(label.id, 1234);
                assert_eq!(label.resource_url, "https://api.discogs.com/labels/1234".to_string());
                assert_eq!(label.name, "Skunkworks".to_string());
            });
    }

    #[test]
    fn test_perform_label_releases_request() {
        mock("GET", "/labels/4567/releases?page=1&per_page=2")
            .with_status(200)
            .with_header("content-type", "text/json")
            .with_body(to_string(&json!(
            {
                "pagination": {
                    "per_page": 2,
                    "items": 9,
                    "page": 1,
                    "urls": {
                        "last": "https://api.discogs.com/labels/4567/releases?per_page=2&page=5",
                        "next": "https://api.discogs.com/labels/4567/releases?per_page=2&page=2"
                    },
                    "pages": 5
                },
                "releases": [{
                    "status": "Accepted",
                    "thumb": "",
                    "format": "LP, TP, W/Lbl",
                    "title": "Split",
                    "catno": "F01",
                    "year": 2001,
                    "resource_url": "https://api.discogs.com/releases/1166635",
                    "artist": "Magmax / Des Esseintes",
                    "id": 1166635
                }, {
                    "status": "Accepted",
                    "thumb": "",
                    "format": "LP, Ltd",
                    "title": "Split",
                    "catno": "F01",
                    "year": 2001,
                    "resource_url": "https://api.discogs.com/releases/324223",
                    "artist": "Magmax / Des Esseintes",
                    "id": 324223
                }]
            }
            )).unwrap().as_str())
            .create_for(|| {
                let mut releases = Discogs::new("USER_AGENT")
                    .label(4567)
                    .pagination(1, 2)
                    .get_releases()
                    .ok()
                    .unwrap();



                assert_eq!(releases.pagination.page,1);
                assert_eq!(releases.pagination.per_page,2);
                assert_eq!(releases.pagination.items,9);
                assert_eq!(releases.pagination.pages,5);


                assert_eq!(releases.releases.len(), 2);

                assert_eq!(releases.releases[0].title, "Split");
                assert_eq!(releases.releases[0].id, 1166635);
                assert_eq!(releases.releases[0].year, 2001);
                assert_eq!(releases.releases[0].resource_url, "https://api.discogs.com/releases/1166635");
                assert_eq!(releases.releases[0].artist, Some(String::from("Magmax / Des Esseintes")));


            });
    }
}
