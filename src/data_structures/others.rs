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
use data_structures::*;
use serde::Serialize;
use serde::Deserialize;
use data_structures::label::Label;
use data_structures::image::Image;
use data_structures::company::Company;
use data_structures::contributor::Contributor;
use data_structures::artist::Artist;

#[derive(Serialize, Deserialize, Debug)]
pub struct Community {
    pub contributors: Vec<Contributor>,
    pub data_quality: String,
    pub have: u32,
    pub rating: Rating,
    pub status: Status,
    pub submitter: Contributor,
    pub want: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rating {
    pub average: f32,
    pub count: u32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseFormat {
    pub descriptions: Vec<String>,
    pub name: String,
    pub qty: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Identifier {
    // TODO: can we do an enum for this?
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub duration: String,
    pub position: String,
    pub title: String,
    pub type_: String,
    pub extraartists: Option<Vec<Artist>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub description: String,
    pub duration: u32,
    pub embed: bool,
    pub title: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Accepted,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataQuality {
    Correct,
    #[serde(rename="Needs Vote")]
    NeedsVote,
    #[serde(rename="Complete and Correct")]
    CompleteAndCorrect,
}
