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
    ///                                   "https://api.discogs.com".to_string(),
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
}

#[cfg(test)]
mod tests {
    use discogs::Discogs;
    use data_structures::*;

    fn aqb(id: u32) -> ArtistQueryBuilder {
        Discogs::new(env!("DISCOGS_USER_AGENT")).artist(id)
    }

    #[test]
    fn test_request_builder() {
        let qb = aqb(789);

        assert_eq!(qb.id, 789);
        assert_eq!(qb.api_endpoint, "https://api.discogs.com".to_owned());
        assert_eq!(qb.user_agent, env!("DISCOGS_USER_AGENT"));
        assert!(qb.key.is_none());
        assert!(qb.secret.is_none());
    }
}


