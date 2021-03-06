// Library that eases the use of discogs API
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

#[cfg(test)]
use mockito::SERVER_URL;
#[cfg(test)]
use super::*;

#[cfg(test)]
pub const API_URL: &'static str = mockito::SERVER_URL;

/// The default host address for the API.
#[cfg(not(test))]
pub const API_URL: &'static str = "https://api.discogs.com";

/// The default rate limit for discogs
pub const API_RATE_LIMIT: u32 = 240;


pub struct Discogs {
    api_endpoint: String,
    user_agent: String,

    key: Option<String>,
    secret: Option<String>,

    // Maximum number of API Queries per minute
    rate_limit: u32,
}

impl Discogs {
    /// Constructs a new `Client` with the provided `user_agent`.
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let client = Discogs::new("USER_AGENT");
    /// ```
    pub fn new(user_agent: &str) -> Self {
        Discogs {
            api_endpoint: API_URL.to_owned(),
            key: None,
            secret: None,
            user_agent: user_agent.to_owned(),
            rate_limit: API_RATE_LIMIT,
        }
    }

    /// Sets the discogs api client key
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let mut client = Discogs::new("USER_AGENT");
    /// client.key("CLIENT_KEY");
    /// ```
    //TODO: Come back and make a better example
    pub fn key(&mut self, key: &str) -> &mut Self {
        self.key = Some(key.to_owned());
        self
    }

    /// Sets the discogs api client secret
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let mut client = Discogs::new("USER_AGENT");
    /// client.secret("CLIENT_STRING");
    /// ```
    //TODO: Come back and make a better example
    pub fn secret(&mut self, secret: &str) -> &mut Self {
        self.secret = Some(secret.to_owned());
        self
    }

    /// Returns an instance of the `ArtistQueryBuilder` structure for the specified id
    /// This allows you to pass parameters to build a request.
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let artist = Discogs::new("USER_AGENT")
    ///                       .artist(4567);
    /// ```
    pub fn artist(&mut self, id: u32) -> ArtistQueryBuilder {
        ArtistQueryBuilder::new(id,
                                self.api_endpoint.clone(),
                                self.user_agent.clone(),
                                self.key.clone(),
                                self.secret.clone())
    }

    /// Returns an instance of the `LabelQueryBuilder` structure for the specified id
    /// This allows you to pass parameters to build a request.
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let label = Discogs::new("USER_AGENT")
    ///                       .label(1234);
    /// ```
    pub fn label(&mut self, id: u32) -> LabelQueryBuilder {
        LabelQueryBuilder::new(id,
                               self.api_endpoint.clone(),
                               self.user_agent.clone(),
                               self.key.clone(),
                               self.secret.clone())
    }

    /// Returns an instance of the `ReleaseQueryBuilder` structure for the specified id
    /// This allows you to pass parameters to build a request.
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let release = Discogs::new("USER_AGENT")
    ///                       .label(1234);
    /// ```
    pub fn release(&mut self, id: u32) -> ReleaseQueryBuilder {
        ReleaseQueryBuilder::new(id,
                                 self.api_endpoint.clone(),
                                 self.user_agent.clone(),
                                 self.key.clone(),
                                 self.secret.clone())
    }

    /// Returns an instance of the `MasterQueryBuilder` structure for the specified id
    /// This allows you to pass parameters to build a request.
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let master = Discogs::new("USER_AGENT")
    ///                       .master(1234);
    /// ```
    pub fn master(&mut self, id: u32) -> MasterQueryBuilder {
        MasterQueryBuilder::new(id,
                                 self.api_endpoint.clone(),
                                 self.user_agent.clone(),
                                 self.key.clone(),
                                 self.secret.clone())
    }

    /// Returns an instance of the `SearchQueryBuilder` structure.
    /// This allows you to pass parameters to build a request.
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let search = Discogs::new("USER_AGENT").search();
    /// ```
    pub fn search(&mut self) -> SearchQueryBuilder {
        SearchQueryBuilder::new(self.api_endpoint.clone(),
                                 self.user_agent.clone(),
                                 self.key.clone(),
                                 self.secret.clone())
    }
}

#[cfg(test)]
mod tests {
    use discogs::Discogs;

    #[test]
    fn user_agent_test() {
        let client = Discogs::new("USER_AGENT");

        assert_eq!(client.user_agent, "USER_AGENT".to_string());
    }

    #[test]
    fn key_test() {
        let mut client = Discogs::new("USER_AGENT");
        client.key("CLIENT_KEY");

        assert_eq!(client.user_agent, "USER_AGENT".to_string());
        assert_eq!(client.key, Some("CLIENT_KEY".to_string()));
    }

    #[test]
    fn secret_test() {
        let mut client = Discogs::new("USER_AGENT");
        client.secret("CLIENT_STRING");

        assert_eq!(client.user_agent, "USER_AGENT".to_string());
        assert_eq!(client.secret, Some("CLIENT_STRING".to_string()));
    }
}
