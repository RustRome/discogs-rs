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

use query::QueryError;

pub trait QueryBuilder {
    fn get_key(&self) -> Option<String> {
        None
    }

    fn get_secret(&self) -> Option<String> {
        None
    }

    // returns the  url to perform the query
    fn get_query_url(&self) -> String;

    fn get_user_agent(&self) -> String;

    fn perform_request(&self) -> Result<String, QueryError> {
        Ok("ads".to_string())
    }
}
