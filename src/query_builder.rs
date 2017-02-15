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

use Discogs;

#[derive(Debug)]
pub enum QueryType {
    Release,
    Master,
    Artist,
    Label,
    Search
}

impl QueryType {
    /// Transforms the `QueryType` into the corresponding string
    /// for the url
    pub fn to_string(&self) -> String {
        match *self {
            QueryType::Release => "releases".to_owned(),
            QueryType::Master => "masters".to_owned(),
            QueryType::Artist => "artists".to_owned(),
            QueryType::Label => "labels".to_owned(),
            QueryType::Search => "database".to_owned()
        }
    }
}

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
    /// Uses the `Discogs` instance information to create a `QueryBuilder`
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::{Discogs, QueryBuilder};
    ///
    /// let client = Discogs::new(env!("DISCOGS_USER_AGENT"));
    /// let query = QueryBuilder::new(&client);
    /// ```
    pub fn new(d: &Discogs) -> QueryBuilder {
        QueryBuilder {
            api_endpoint: d.api_endpoint.clone(),
            user_agent: d.user_agent.clone(),
            key: d.key.clone(),
            secret: d.key.clone(),
            query_type: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use Discogs;
    use QueryBuilder;
    use QueryType;

    #[test]
    fn query_builder_new() {

        let client = Discogs::new(env!("DISCOGS_USER_AGENT"));
        let query = QueryBuilder::new(&client);

        assert_eq!(query.api_endpoint, client.api_endpoint);
        assert_eq!(query.user_agent, client.user_agent);
        assert_eq!(query.key, client.key);
        assert_eq!(query.secret, client.secret);
        assert!(query.query_type.is_none());
    }

    #[test]
    fn query_type_to_string() {
        assert_eq!(QueryType::Release.to_string(), "releases");
        assert_eq!(QueryType::Master.to_string(), "masters");
        assert_eq!(QueryType::Artist.to_string(), "artists");
        assert_eq!(QueryType::Label.to_string(), "labels");
        assert_eq!(QueryType::Search.to_string(), "database");
    }

}

