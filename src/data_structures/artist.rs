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
pub const ARTIST_ENDPOINT: &'static str = "/artists";



#[derive(Deserialize, Debug)]
pub struct ArtistReleases {
    pub pagination: Pagination,
    pub releases: Vec<Release>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artist {
    pub id: u32,
    pub name: String,
    pub resource_url: String,
    pub tracks: Option<String>,
    pub uri: Option<String>,
    pub releases_url: Option<String>,
    pub aliases: Option<Vec<Artist>>,
    pub join: Option<String>,
    pub role: Option<String>,
    // 'groups',
    pub anv: Option<String>,
    pub members: Option<Vec<Artist>>,
    pub active: Option<bool>,
    pub namevariations: Option<Vec<String>>,
    pub urls: Option<Vec<String>>,
    pub images: Option<Vec<Image>>,
    pub profile: Option<String>,
    pub data_quality: Option<DataQuality>,
    pub realname: Option<String>,
}

impl Artist {
    /// Creates a new instance of `Artist`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::Artist;
    ///
    /// let artist = Artist::new(4567,
    ///                          "name".to_string(),
    ///                          "resource_url".to_string());
    /// ```
    pub fn new(id: u32, name: String, resource_url: String) -> Artist {
         Artist {
            id: id,
            name: name,
            resource_url: resource_url,
            tracks: None,
            uri: None,
            releases_url: None,
            aliases: None,
            join: None,
            role: None,
            anv: None,
            members: None,
            active: None,
            namevariations: None,
            urls: None,
            images: None,
            profile: None,
            data_quality: None,
            realname: None,
        }
    }
}

// TODO: make a more comprehensive test
impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        other.id == self.id
    }
}


pub struct ArtistQueryBuilder {
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

impl ArtistQueryBuilder {
    /// Creates a new instance of `ArtistQueryBuilder`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::ArtistQueryBuilder;
    ///
    /// let aqb = ArtistQueryBuilder::new(4567,
    ///                                   discogs::API_URL.to_string(),
    ///                                   "USER_AGENT".to_string(),
    ///                                   "CLIENT_KEY".to_string(),
    ///                                   "CLIENT_SECRET.to_string()");
    /// ```
    pub fn new(id: u32,
               api_endpoint: String,
               user_agent: String,
               key: Option<String>,
               secret: Option<String>) -> ArtistQueryBuilder {
        ArtistQueryBuilder {
            id: id,
            api_endpoint: api_endpoint,
            user_agent: user_agent,
            key: key,
            secret: secret,
            page : 1,
            per_page : 50,
            releases : false
        }
    }

    /// Perform request
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::ArtistQueryBuilder;
    ///
    /// let artist = ArtistQueryBuilder::new(4567,
    ///                                  discogs::API_URL.to_string(),
    ///                                  "USER_AGENT".to_string())
    ///                                  .get();
    /// ```
    pub fn get(&self) -> Result<Artist, QueryError> {
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

    pub fn pagination(&mut self, page: i16, per_page: i16) -> &mut ArtistQueryBuilder {
        self.page = page;
        self.per_page = per_page;
        self
    }
    /// Returns an instance of the `ArtistReleasesQueryBuilder` structure for the specified id
    /// This allows you to pass parameters to build a request.
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let releases = Discogs::new("USER_AGENT")
    ///                       .artist(1234)
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

impl QueryBuilder for ArtistQueryBuilder {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn get_secret(&self) -> Option<String> {
        self.secret.clone()
    }

    fn get_query_url(&self) -> String {

        match self.releases {
            false => format!("{}{}/{}", self.api_endpoint, ARTIST_ENDPOINT, self.id),
            true => format!("{}{}/{}/releases?page={}&per_page={}", self.api_endpoint, ARTIST_ENDPOINT, self.id, self.page, self.per_page)
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

    fn aqb(id: u32) -> ArtistQueryBuilder {
        Discogs::new("USER_AGENT").artist(id)
    }

    #[test]
    fn test_request_builder() {
        let qb = aqb(789);

        assert_eq!(qb.id, 789);
        assert_eq!(qb.api_endpoint, API_URL.to_owned());
        assert_eq!(qb.user_agent, "USER_AGENT".to_string());
        assert!(qb.key.is_none());
        assert!(qb.secret.is_none());
    }

    #[test]
    fn test_artist_new() {
        let artist = Artist::new(4567,
                                 "name".to_string(),
                                 "resource_url".to_string());

        assert_eq!(artist.id, 4567);
        assert_eq!(artist.name, "name".to_string());
        assert_eq!(artist.resource_url, "resource_url".to_string());
    }

    #[test]
    fn test_artist_eq() {
        let artist = Artist::new(4567,
                                 "name".to_string(),
                                 "resource_url".to_string());
        let artist2 = Artist::new(4567,
                                 "adsh".to_string(),
                                 "dasdas".to_string());

        assert!(artist == artist2);
    }

    #[test]
    fn test_perform_artist_request() {
        mock("GET", "/artists/4567")
            .with_status(200)
            .with_header("content-type", "text/json")
            .with_body(to_string(&json!({
                "id": 4567,
                "resource_url": "https://api.discogs.com/artists/4567",
                "name": "Whirlpool Productions"
            })).unwrap().as_str())
            .create_for(|| {
                let artist = Discogs::new("USER_AGENT")
                    .artist(4567)
                    .get()
                    .ok()
                    .unwrap();

                assert_eq!(artist.id, 4567);
                assert_eq!(artist.resource_url, "https://api.discogs.com/artists/4567".to_string());
                assert_eq!(artist.name, "Whirlpool Productions".to_string());
            });
    }

    #[test]
    fn test_perform_artist_releases_request() {
        mock("GET", "/artists/4567/releases?page=1&per_page=2")
            .with_status(200)
            .with_header("content-type", "text/json")
            .with_body(to_string(&json!(
            {
            "pagination": {
                "per_page": 2,
                "items": 220,
                "page": 1,
                "urls": {
                    "last": "https://api.discogs.com/artists/4567/releases?per_page=2&page=110",
                    "next": "https://api.discogs.com/artists/4567/releases?per_page=2&page=2"
                },
                "pages": 110
            },
            "releases": [{
                "status": "Accepted",
                "thumb": "",
                "title": "Fly High",
                "format": "12\"",
                "label": "5th & Madison",
                "role": "Main",
                "year": 1992,
                "resource_url": "https://api.discogs.com/releases/843401",
                "artist": "Whirlpool*",
                "type": "release",
                "id": 843401
            }, {
                "status": "Accepted",
                "thumb": "",
                "title": "Dream Team E.P.",
                "format": "12\", EP",
                "label": "Intense Recordings",
                "role": "Main",
                "year": 1993,
                "resource_url": "https://api.discogs.com/releases/94983",
                "artist": "T'N'I / Whirlpool Productions",
                "type": "release",
                "id": 94983
            }]
            }
            )).unwrap().as_str())
            .create_for(|| {
                let mut releases = Discogs::new("USER_AGENT")
                    .artist(4567)
                    .pagination(1, 2)
                    .get_releases()
                    .ok()
                    .unwrap();



                assert_eq!(releases.pagination.page,1);
                assert_eq!(releases.pagination.per_page,2);
                assert_eq!(releases.pagination.items,220);
                assert_eq!(releases.pagination.pages,110);


                assert_eq!(releases.releases.len(), 2);

                assert_eq!(releases.releases[0].title, "Fly High");
                assert_eq!(releases.releases[0].id, 843401);
                assert_eq!(releases.releases[0].resource_url, "https://api.discogs.com/releases/843401");
                assert_eq!(releases.releases[0].artist, Some(String::from("Whirlpool*")));
                assert_eq!(releases.releases[0].label, Some(String::from("5th & Madison")));


            });
    }
}
