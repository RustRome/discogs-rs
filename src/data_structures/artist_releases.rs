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


#[derive(Deserialize, Debug)]
pub struct ArtistRelease {
    pub title: String,
    pub id: i64,
    pub resource_url: String,
    pub year: Option<u32>,
    pub artist: String,
    pub label: String,
    pub format: String,
    pub role: String,
    pub status: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct ArtistReleases {
    pub pagination: Pagination,
    pub releases: Vec<ArtistRelease>,
}

pub struct ArtistReleasesQueryBuilder {
    //artist id
    id: u32,

    api_endpoint: String,
    user_agent: String,

    // Optional key and secret if necessary
    key: Option<String>,
    secret: Option<String>,

    page: i16,
    per_page: i16,
}


impl ArtistReleasesQueryBuilder {
    /// Creates a new instance of `ArtistReleasesQueryBuilder`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::ArtistReleasesQueryBuilder;
    ///
    /// let arqb = ArtistReleasesQueryBuilder::new(4567,
    ///                                   discogs::API_URL.to_string(),
    ///                                   env!("DISCOGS_USER_AGENT").to_string());
    /// ```
    pub fn new(id: u32, api_endpoint: String, user_agent: String) -> ArtistReleasesQueryBuilder {
        ArtistReleasesQueryBuilder {
            id: id,
            api_endpoint: api_endpoint,
            user_agent: user_agent,
            key: None,
            secret: None,
            page: 1,
            per_page: 50
        }
    }

    /// Perform request
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::data_structures::ArtistReleasesQueryBuilder;
    ///
    /// let artist = ArtistReleasesQueryBuilder::new(4567,
    ///                                  discogs::API_URL.to_string(),
    ///                                  env!("DISCOGS_USER_AGENT").to_string())
    ///                                  .get();
    /// ```
    pub fn get(&self) -> Result<ArtistReleases, QueryError> {
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

    pub fn pagination(&mut self, page: i16, per_page: i16) -> &mut ArtistReleasesQueryBuilder {
        self.page = page;
        self.per_page = per_page;
        self
    }
}

impl QueryBuilder for ArtistReleasesQueryBuilder {
    fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    fn get_secret(&self) -> Option<String> {
        self.secret.clone()
    }

    fn get_query_url(&self) -> String {
        format!("{}{}/{}/releases?page={}&per_page={}", self.api_endpoint, ARTIST_ENDPOINT, self.id, self.page, self.per_page)
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
    use serde_json::to_string;

    fn arqb(id: u32) -> ArtistReleasesQueryBuilder {
        Discogs::new(env!("DISCOGS_USER_AGENT")).artist(id).releases()
    }

    #[test]
    fn test_request_builder() {
        let qb = arqb(789);

        assert_eq!(qb.id, 789);
        assert_eq!(qb.api_endpoint, API_URL.to_owned());
        assert_eq!(qb.user_agent, env!("DISCOGS_USER_AGENT"));
        assert!(qb.key.is_none());
        assert!(qb.secret.is_none());
    }

    #[test]
    fn test_perform_artist_request() {
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
                let releases = Discogs::new(env!("DISCOGS_USER_AGENT"))
                    .artist(4567)
                    .releases()
                    .pagination(1, 2)
                    .get()
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

            });
    }
}
