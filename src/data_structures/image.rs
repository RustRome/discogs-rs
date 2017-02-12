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
use Queryable;
use serde_json;
use QuerySource;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub resource_url: String,
    // TODO: do a enum type
    #[serde(rename = "type")]
    pub image_type: String,
    pub uri: String,
    pub uri150: String,
    pub height: u32,
    pub width: u32,
}


impl Image {
    pub fn new(resource_url: String, d: &Discogs) -> Self {
        // TODO: Do we need clone here?
        let qs = QuerySource::Url { url: resource_url };
        serde_json::from_str(&d.query(qs).unwrap()[..]).unwrap()
    }
}

impl Queryable for Image {
    // TODO: there is probably a better way to do this without the clone
    fn query_source(&self) -> QuerySource {
        QuerySource::Url { url: self.resource_url.clone() }
    }
}
