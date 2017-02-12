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

use Queryable;
use QuerySource;

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
    pub catno: String,
    pub entity_type: String,
    pub entity_type_name: String,
    pub id: u32,
    pub name: String,
    pub resource_url: String,
}

impl Queryable for Company {
    // TODO: there is probably a better way to do this without the clone
    fn query_source(&self) -> QuerySource {
        QuerySource::Url { url: self.resource_url.clone() }
    }
}
