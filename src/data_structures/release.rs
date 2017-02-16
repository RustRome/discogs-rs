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

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub title: String,
    pub id: u32,
    pub released: String,
    pub released_formatted: String,
    pub resource_url: String,
    pub date_added: String,
    pub date_changed: String,
    pub uri: String,
    pub year: u32,
    pub artists: Vec<Artist>,
    pub status: Status,

    pub data_quality: Option<DataQuality>,
    pub thumb: Option<String>,
    pub community: Option<Community>,
    pub companies: Option<Vec<Company>>,
    pub country: Option<String>,
    pub estimated_weight: Option<u32>,
    pub extra_artists: Option<Vec<Artist>>,
    pub format_quantity: Option<u32>,
    pub formats: Option<Vec<ReleaseFormat>>,
    pub genres: Option<Vec<String>>,
    pub identifiers: Option<Vec<Identifier>>,
    pub images: Option<Vec<Image>>,
    pub labels: Option<Vec<Label>>,
    pub lowest_price: Option<f64>,
    pub master_id: Option<u32>,
    pub master_url: Option<String>,
    pub notes: Option<String>,
    pub num_for_sale: Option<u32>,
    pub series: Option<Vec<Label>>,
    pub styles: Option<Vec<String>>,
    pub tracklist: Option<Vec<Track>>,
    pub videos: Option<Vec<Video>>,
}