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
use data_structures::others::*;
use data_structures::image::Image;
use data_structures::artist::Artist;

#[derive(Serialize, Deserialize, Debug)]
pub struct Master {
    // Mandatory fields
    pub id: u32,
    pub resource_url: String,
    pub main_release: u32,
    pub main_release_url: String,

    // Optional fields
    pub title: Option<String>,
    pub year: Option<u32>,
    pub images: Option<Vec<Image>>,
    pub tracklist: Option<Vec<Track>>,
    pub uri: Option<String>,
    pub genres: Option<Vec<String>>,
    pub artists: Option<Vec<Artist>>,
    pub notes: Option<String>,
    pub videos: Option<Vec<Video>>,
    pub data_quality: Option<DataQuality>,
    pub num_for_sale: Option<u32>,
    pub styles: Option<Vec<String>>,
    pub versions_url: Option<String>,
    pub lowest_price: Option<f64>,
}

impl Master {
    pub fn new(id: u32, d: &Discogs) -> Self {
        // TODO: Do we need clone here?
        let qs = QuerySource::Id {
            api_endpoint: d.api_endpoint.clone(),
            endpoint: "masters".to_owned(),
            id: id,
        };
        serde_json::from_str(&d.query(qs).unwrap()[..]).unwrap()
    }
}

impl Queryable for Master {
    // TODO: there is probably a better way to do this without the clone
    fn query_source(&self) -> QuerySource {
        QuerySource::Url { url: self.resource_url.clone() }
    }
}
