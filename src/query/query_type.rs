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

#[cfg(test)]
mod tests {
    use query::QueryType;

    #[test]
    fn query_type_to_string() {
        assert_eq!(QueryType::Release.to_string(), "releases");
        assert_eq!(QueryType::Master.to_string(), "masters");
        assert_eq!(QueryType::Artist.to_string(), "artists");
        assert_eq!(QueryType::Label.to_string(), "labels");
        assert_eq!(QueryType::Search.to_string(), "database");
    }

}


