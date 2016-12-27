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

#![allow(dead_code)]
/// / Data structures
#[derive(Serialize, Deserialize)]
pub struct Community {
    pub contributors: Vec<Contributor>,
    pub data_quality: String,
    pub have: u32,
    pub rating: Rating,
    pub status: Status,
    pub submitter: Contributor,
    pub want: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Rating {
    pub average: f32,
    pub count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Contributor {
    pub resource_url: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReleaseFormat {
    pub descriptions: Vec<String>,
    pub name: String,
    pub qty: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
    pub id: u32,
    pub resource_url: String,
    pub name: String,
    pub profile: Option<String>,
    pub releases_url: Option<String>,
    pub contact_info: Option<String>,
    pub uri: Option<String>,
    pub urls: Option<Vec<String>>,
    pub data_quality: Option<DataQuality>,
    pub sublabels: Option<Vec<Label>>,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub duration: String,
    pub position: String,
    pub title: String,
    pub type_: String,
    pub extraartists: Option<Vec<Artist>>,
}

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub description: String,
    pub duration: u32,
    pub embed: bool,
    pub title: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct Company {
    pub catno: String,
    pub entity_type: String,
    pub entity_type_name: String,
    pub id: u32,
    pub name: String,
    pub resource_url: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
    pub extraartists: Option<Vec<Artist>>,
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

#[derive(Serialize, Deserialize)]
pub enum Status {
    Accepted,
}

#[derive(Serialize, Deserialize)]
pub enum DataQuality {
    Correct,
    #[serde(rename="Needs Vote")]
    NeedsVote,
    #[serde(rename="Complete and Correct")]
    CompleteAndCorrect,
}

#[derive(Serialize, Deserialize)]
pub struct Master {
    pub id: u32,
    pub resource_url: String,
    pub main_release: u32,
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
    pub main_release_url: Option<String>,
    pub lowest_price: Option<f64>,
}
