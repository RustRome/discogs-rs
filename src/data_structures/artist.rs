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
const ARTIST_ENDPOINT: &'static str = "artists";


#[derive(Serialize, Deserialize, Debug)]
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
    pub fn new(id: u32,name: String, resource_url: String) -> Artist {
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
    ///                                   env!("DISCOGS_USER_AGENT").to_string());
    /// ```
    pub fn new(id: u32, api_endpoint: String, user_agent: String) -> ArtistQueryBuilder {
        ArtistQueryBuilder {
            id: id,
            api_endpoint: api_endpoint,
            user_agent: user_agent,
            key: None,
            secret: None
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
    ///                                  env!("DISCOGS_USER_AGENT").to_string())
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
}

impl QueryBuilder for ArtistQueryBuilder {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn get_secret(&self) -> Option<String> {
        self.secret.clone()
    }

    fn get_query_url(&self) -> String {
        format!("{}{}/{}", self.api_endpoint, ARTIST_ENDPOINT, self.id)
    }

    fn get_user_agent(&self) -> String {
        self.user_agent.clone()
    }
}

#[cfg(test)]
mod tests {
    use discogs::*;
    use data_structures::*;

    fn aqb(id: u32) -> ArtistQueryBuilder {
        Discogs::new(env!("DISCOGS_USER_AGENT")).artist(id)
    }

    #[test]
    fn test_request_builder() {
        let qb = aqb(789);

        assert_eq!(qb.id, 789);
        assert_eq!(qb.api_endpoint, API_URL.to_owned());
        assert_eq!(qb.user_agent, env!("DISCOGS_USER_AGENT"));
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
}
