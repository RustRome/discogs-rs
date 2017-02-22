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
const RELEASE_ENDPOINT: &'static str = "/releases";


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    pub title: String,
    pub id: u32,
    pub status: Status,
    pub resource_url: String,
    pub year: u32,

    pub artists: Option<Vec<Artist>>,
    pub artist: Option<String>,
    pub uri: Option<String>,
    pub released: Option<String>,
    pub released_formatted: Option<String>,
    pub date_added: Option<String>,
    pub date_changed: Option<String>,
    pub data_quality: Option<DataQuality>,
    pub thumb: Option<String>,
    pub community: Option<Community>,
    pub companies: Option<Vec<Company>>,
    pub country: Option<String>,
    pub estimated_weight: Option<u32>,
    pub extra_artists: Option<Vec<Artist>>,
    pub format_quantity: Option<u32>,
    pub formats: Option<Vec<ReleaseFormat>>,
    pub genres: Option<Vec<String>>,
    pub identifiers: Option<Vec<Identifier>>,
    pub images: Option<Vec<Image>>,
    pub labels: Option<Vec<Label>>,
    pub label: Option<String>,
    pub lowest_price: Option<f64>,
    pub master_id: Option<u32>,
    pub master_url: Option<String>,
    pub notes: Option<String>,
    pub num_for_sale: Option<u32>,
    pub series: Option<Vec<Label>>,
    pub styles: Option<Vec<String>>,
    pub tracklist: Option<Vec<Track>>,
    pub videos: Option<Vec<Video>>,
}


impl Release {
    /// Creates a new instance of `Release`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::*;
    ///
    /// let release = Release::new(128,
    ///                            "title".to_string(),
    ///                            "released".to_string(),
    ///                            "released_formatted".to_string(),
    ///                            "resource_url".to_string(),
    ///                            "date_added".to_string(),
    ///                            "date_changed".to_string(),
    ///                            "uri".to_string(),
    ///                            2000,
    ///                            vec![Artist::new(1,
    ///                                             "name".to_string(),
    ///                                             "resource_url".to_string())],
    ///                            Status::Accepted);
    ///
    /// ```
    pub fn new(id: u32,
               title: String,
               released: String,
               released_formatted: String,
               resource_url: String,
               date_added: String,
               date_changed: String,
               uri: String,
               year: u32,
               artists: Vec<Artist>,
               status: Status) -> Release {

        Release {
            id: id,
            title: title,
            released: Some(released),
            released_formatted: Some(released_formatted),
            resource_url: resource_url,
            date_added: Some(date_added),
            date_changed: Some(date_changed),
            uri: Some(uri),
            year: year,
            artists: Some(artists),
            artist : None,
            status: status,
            data_quality: None,
            thumb: None,
            community: None,
            companies: None,
            country: None,
            estimated_weight: None,
            extra_artists: None,
            format_quantity: None,
            formats: None,
            genres: None,
            identifiers: None,
            images: None,
            label : None,
            labels: None,
            lowest_price: None,
            master_id: None,
            master_url: None,
            notes: None,
            num_for_sale: None,
            series: None,
            styles: None,
            tracklist: None,
            videos: None,
        }
    }

}

// TODO: make a more comprehensive test
impl PartialEq for Release {
    fn eq(&self, other: &Release) -> bool {
        other.id == self.id
    }
}

pub struct ReleaseQueryBuilder {
    //artist id
    id: u32,

    api_endpoint: String,
    user_agent: String,

    // Optional key and secret if necessary
    key: Option<String>,
    secret: Option<String>,
}

impl ReleaseQueryBuilder {
    /// Creates a new instance of `ReleaseQueryBuilder`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::ReleaseQueryBuilder;
    ///
    /// let rqb = ReleaseQueryBuilder::new(128,
    ///                                   discogs::API_URL.to_string(),
    ///                                   "USER_AGENT".to_string(),
    ///                                   Some("CLIENT_KEY".to_string()),
    ///                                   Some("CLIENT_SECRET".to_string()));
    /// ```
    pub fn new(id: u32,
               api_endpoint: String,
               user_agent: String,
               key: Option<String>,
               secret: Option<String>) -> ReleaseQueryBuilder {
        ReleaseQueryBuilder {
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
    /// use discogs::data_structures::ReleaseQueryBuilder;
    ///
    /// let rqb = ReleaseQueryBuilder::new(128,
    ///                                   discogs::API_URL.to_string(),
    ///                                   "USER_AGENT".to_string(),
    ///                                   Some("CLIENT_KEY".to_string()),
    ///                                   Some("CLIENT_SECRET".to_string()))
    ///                                    .get();
    /// ```
    pub fn get(&self) -> Result<Release, QueryError> {
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
}

impl QueryBuilder for ReleaseQueryBuilder {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn get_secret(&self) -> Option<String> {
        self.secret.clone()
    }

    fn get_query_url(&self) -> String {
        format!("{}{}/{}", self.api_endpoint, RELEASE_ENDPOINT, self.id)
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

    fn rqb(id: u32) -> ReleaseQueryBuilder {
        Discogs::new("USER_AGENT").release(id)
    }

    #[test]
    fn test_request_builder() {
        let qb = rqb(999);

        assert_eq!(qb.id, 999);
        assert_eq!(qb.api_endpoint, API_URL.to_owned());
        assert_eq!(qb.user_agent, "USER_AGENT".to_string());
        assert!(qb.key.is_none());
        assert!(qb.secret.is_none());
    }

    #[test]
    fn test_release_new() {
        let release = Release::new(128,
                                   "title".to_string(),
                                   "released".to_string(),
                                   "released_formatted".to_string(),
                                   "resource_url".to_string(),
                                   "date_added".to_string(),
                                   "date_changed".to_string(),
                                   "uri".to_string(),
                                   2000,
                                   vec![Artist::new(1,
                                                    "name".to_string(),
                                                    "resource_url".to_string())],
                                   Status::Accepted);

        assert_eq!(release.id, 128);
        assert_eq!(release.title, "title".to_string());
        assert_eq!(release.released, Some("released".to_string()));
        assert_eq!(release.released_formatted, Some("released_formatted".to_string()));
        assert_eq!(release.resource_url, "resource_url".to_string());
        assert_eq!(release.date_added, Some("date_added".to_string()));
        assert_eq!(release.date_changed, Some("date_changed".to_string()));
        assert_eq!(release.uri, Some("uri".to_string()));
        assert_eq!(release.year, 2000);
        assert_eq!(release.artists, Some(vec![
                   Artist::new(1,
                               "name".to_string(),
                               "resource_url".to_string())
        ]));
        assert_eq!(release.status, Status::Accepted);
    }

    #[test]
    fn test_release_eq() {
        let release = Release::new(128,
                                   "title".to_string(),
                                   "released".to_string(),
                                   "released_formatted".to_string(),
                                   "resource_url".to_string(),
                                   "date_added".to_string(),
                                   "date_changed".to_string(),
                                   "uri".to_string(),
                                   2000,
                                   vec![Artist::new(1,
                                                    "name".to_string(),
                                                    "resource_url".to_string())],
                                   Status::Accepted);

        let release2 = release.clone();

        assert!(release == release2);
    }

    #[test]
    fn test_perform_release_request() {
        mock("GET", "/releases/128")
            .with_status(200)
            .with_header("content-type", "text/json")
            .with_body(to_string(&json!({
                "id": 128,
                "title": "title",
                "released": "released",
                "released_formatted": "released_formatted",
                "resource_url": "resource_url",
                "date_added": "date_added",
                "date_changed": "date_changed",
                "uri": "uri",
                "year": 2000,
                "status": "Accepted",
                "artists": [{
                    "id": 1,
                    "name": "name",
                    "resource_url": "resource_url"
                }]
            })).unwrap().as_str())
            .create_for(|| {
                let release = Discogs::new("USER_AGENT")
                    .release(128)
                    .get()
                    .ok()
                    .unwrap();

                assert_eq!(release.id, 128);
                assert_eq!(release.title, "title".to_string());
                assert_eq!(release.released, Some("released".to_string()));
                assert_eq!(release.released_formatted, Some("released_formatted".to_string()));
                assert_eq!(release.resource_url, "resource_url".to_string());
                assert_eq!(release.date_added, Some("date_added".to_string()));
                assert_eq!(release.date_changed, Some("date_changed".to_string()));
                assert_eq!(release.uri, Some("uri".to_string()));
                assert_eq!(release.year, 2000);
                assert_eq!(release.artists, Some(vec![
                           Artist::new(1,
                                       "name".to_string(),
                                       "resource_url".to_string())
                ]));
                assert_eq!(release.status, Status::Accepted);
            });
    }
}
