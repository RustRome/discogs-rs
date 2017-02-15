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

use query::{QueryType, QueryError};

pub struct QueryBuilder {
    api_endpoint: String,
    user_agent: String,

    // Optional key and secret if necessary
    key: Option<String>,
    secret: Option<String>,

    /// `query_type` refers to the specific endpoint being acessed
    /// for example `artists` or `labels`
    query_type: Option<QueryType>,

}

impl QueryBuilder {
    /// Create an instance of a `QueryBuilder`
    ///
    /// You will almost always want to use the method `query` provided
    /// by a discogs instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::query::QueryBuilder;
    ///
    /// let query = QueryBuilder::new("https://api.discogs.com".to_string(),
    ///                               env!("DISCOGS_USER_AGENT").to_string());
    /// ```
    pub fn new(api_endpoint: String, user_agent: String) -> QueryBuilder {
        QueryBuilder {
            api_endpoint: api_endpoint,
            user_agent: user_agent,
            key: None,
            secret: None,
            query_type: None,
        }
    }

    /// Assign a key to a query
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::query::QueryBuilder;
    ///
    /// let mut query = QueryBuilder::new("https://api.discogs.com".to_string(),
    ///                                   env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// query.key(Some(env!("DISCOGS_CLIENT_KEY").to_string()));
    /// ```
    pub fn key(&mut self, key: Option<String>) -> &mut Self {
        self.key = key;
        self
    }

    /// Assign a secret to a query
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::query::QueryBuilder;
    ///
    /// let mut query = QueryBuilder::new("https://api.discogs.com".to_string(),
    ///                                   env!("DISCOGS_USER_AGENT").to_string());
    ///
    /// query.secret(Some(env!("DISCOGS_CLIENT_SECRET").to_string()));
    /// ```
    pub fn secret(&mut self, secret: Option<String>) -> &mut Self {
        self.secret = secret;
        self
    }




    //pub fn get(&self) -> Result<String, QueryError> {
    //}
}

#[cfg(test)]
mod tests {
    use query::{QueryBuilder, QueryType};

    #[test]
    fn query_builder_test() {

        let mut query = QueryBuilder::new("https://api.discogs.com".to_string(),
                                          env!("DISCOGS_USER_AGENT").to_string());

        assert_eq!(query.api_endpoint, "https://api.discogs.com".to_string());
        assert_eq!(query.user_agent, env!("DISCOGS_USER_AGENT"));
        assert!(query.key.is_none());
        assert!(query.secret.is_none());
        assert!(query.query_type.is_none());

        query.key(Some(env!("DISCOGS_CLIENT_KEY").to_string()))
            .secret(Some(env!("DISCOGS_CLIENT_SECRET").to_string()));

        assert_eq!(query.key, Some(env!("DISCOGS_CLIENT_KEY").to_string()));
        assert_eq!(query.secret, Some(env!("DISCOGS_CLIENT_SECRET").to_string()));
    }
}

