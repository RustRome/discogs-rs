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
use query::query_type::QueryType;
use query::query_error::QueryError;

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

    //pub fn get(&self) -> Result<String, QueryError> {
    //}
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
}

