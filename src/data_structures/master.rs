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
const MASTER_ENDPOINT: &'static str = "/masters";


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Master {
    // Mandatory fields
    pub id: u32,
    pub resource_url: String,
    pub main_release: u32,
    pub main_release_url: String,

    // Optional fields
    pub title: Option<String>,
    pub year: Option<u32>,
    pub images: Option<Vec<Image>>,
    pub tracklist: Option<Vec<Track>>,
    pub uri: Option<String>,
    pub genres: Option<Vec<String>>,
    pub artists: Option<Vec<Artist>>,
    pub notes: Option<String>,
    pub videos: Option<Vec<Video>>,
    pub data_quality: Option<DataQuality>,
    pub num_for_sale: Option<u32>,
    pub styles: Option<Vec<String>>,
    pub versions_url: Option<String>,
    pub lowest_price: Option<f64>,
}

impl Master {
    /// Creates a new instance of `Master`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::Master;
    ///
    /// let master = Master::new(7896,
    ///                          "resource_url".to_string(),
    ///                          982,
    ///                          "main_release_url".to_string());
    /// ```
    pub fn new(id: u32,
               resource_url: String,
               main_release: u32,
               main_release_url: String) -> Master {
         Master {
            id: id,
            resource_url: resource_url,
            main_release: main_release,
            main_release_url: main_release_url,
            title: None,
            year: None,
            images: None,
            tracklist: None,
            uri: None,
            genres: None,
            artists: None,
            notes: None,
            videos: None,
            data_quality: None,
            num_for_sale: None,
            styles: None,
            versions_url: None,
            lowest_price: None,
        }
    }
}

// TODO: make a more comprehensive test
impl PartialEq for Master {
    fn eq(&self, other: &Master) -> bool {
        other.id == self.id
    }
}


pub struct MasterQueryBuilder {
    //master id
    id: u32,

    api_endpoint: String,
    user_agent: String,

    // Optional key and secret if necessary
    key: Option<String>,
    secret: Option<String>,
}

impl MasterQueryBuilder {
    /// Creates a new instance of `MasterQueryBuilder`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::MasterQueryBuilder;
    ///
    /// let mqb = MasterQueryBuilder::new(7896,
    ///                                   discogs::API_URL.to_string(),
    ///                                   "USER_AGENT".to_string(),
    ///                                   "CLIENT_KEY".to_string(),
    ///                                   "CLIENT_SECRET".to_string());
    /// ```
    pub fn new(id: u32,
               api_endpoint: String,
               user_agent: String,
               key: Option<String>,
               secret: Option<String>) -> MasterQueryBuilder {
        MasterQueryBuilder {
            id: id,
            api_endpoint: api_endpoint,
            user_agent: user_agent,
            key: key,
            secret:secret
        }
    }

    /// Perform request
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::MasterQueryBuilder;
    ///
    /// let master = MasterQueryBuilder::new(7896,
    ///                                  discogs::API_URL.to_string(),
    ///                                  "USER_AGENT".to_string())
    ///                                  .get();
    /// ```
    pub fn get(&self) -> Result<Master, QueryError> {
        let result: Result<String, QueryError> = self.perform_request();

        if let Err(error) = result {
            return Err(error);
        } else {
            let result_string = result.ok().unwrap();
            let json = serde_json::from_str(&result_string);

            if let Ok(master) = json {
                return Ok(master);
            } else {
                return Err(QueryError::JsonDecodeError {
                    serde_err: json.err()
                });
            }
        }
    }
}

impl QueryBuilder for MasterQueryBuilder {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn get_secret(&self) -> Option<String> {
        self.secret.clone()
    }

    fn get_query_url(&self) -> String {
        format!("{}{}/{}", self.api_endpoint, MASTER_ENDPOINT, self.id)
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

    fn mqb(id: u32) -> MasterQueryBuilder {
        Discogs::new("USER_AGENT").master(id)
    }

    #[test]
    fn test_request_builder() {
        let qb = mqb(789);

        assert_eq!(qb.id, 789);
        assert_eq!(qb.api_endpoint, API_URL.to_owned());
        assert_eq!(qb.user_agent, "USER_AGENT".to_string());
        assert!(qb.key.is_none());
        assert!(qb.secret.is_none());
    }

    #[test]
    fn test_master_new() {
        let master = Master::new(7896,
                                 "resource_url".to_string(),
                                 982,
                                 "main_release_url".to_string());

        assert_eq!(master.id, 7896);
        assert_eq!(master.resource_url, "resource_url".to_string());
        assert_eq!(master.main_release, 982);
        assert_eq!(master.main_release_url, "main_release_url".to_string());
    }

    #[test]
    fn test_master_eq() {
        let master = Master::new(7896,
                                 "resource_url".to_string(),
                                 982,
                                 "main_release_url".to_string());
        let master2 = Master::new(7896,
                                  "rj".to_string(),
                                  993,
                                  "masdsaddain_release_url".to_string());

        assert!(master == master2);
    }

    #[test]
    fn test_perform_master_request() {
        mock("GET", "/masters/7896")
            .with_status(200)
            .with_header("content-type", "text/json")
            .with_body(to_string(&json!({
                "id": 7896,
                "resource_url": "resource_url",
                "main_release": 982,
                "main_release_url": "main_release_url",
            })).unwrap().as_str())
        .create_for(|| {
            let master = Discogs::new("USER_AGENT")
                .master(7896)
                .get()
                .ok()
                .unwrap();

            assert_eq!(master.id, 7896);
            assert_eq!(master.resource_url, "resource_url".to_string());
            assert_eq!(master.main_release, 982);
            assert_eq!(master.main_release_url, "main_release_url".to_string());
        });
    }
}
