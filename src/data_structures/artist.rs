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
use serde_json::from_str;
use std::io::Read;
use hyper::client::Response;
use QuerySource;
use data_structures::others::*;
use data_structures::label::Label;
use data_structures::image::Image;
use data_structures::company::Company;
use data_structures::contributor::Contributor;

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
    pub fn new(id: u32, d: &Discogs) -> Option<Self> {
        // TODO: Do we need clone here?
        let qs = QuerySource::Id {
            api_endpoint: d.api_endpoint.clone(),
            endpoint: "artists".to_owned(),
            id: id,
        };
        serde_json::from_str(&d.query(qs).unwrap()[..]).unwrap()
    }
}

impl Queryable for Artist {
    // TODO: there is probably a better way to do this without the clone
    fn query_source(&self) -> QuerySource {
        QuerySource::Url { url: self.resource_url.clone() }
    }
}
