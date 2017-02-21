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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Community {
    pub contributors: Vec<Contributor>,
    pub data_quality: String,
    pub have: u32,
    pub rating: Rating,
    pub status: Status,
    pub submitter: Contributor,
    pub want: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rating {
    pub average: f32,
    pub count: u32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseFormat {
    pub descriptions: Vec<String>,
    pub name: String,
    pub qty: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identifier {
    // TODO: can we do an enum for this?
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub duration: String,
    pub position: String,
    pub title: String,
    pub type_: String,
    pub extra_artists: Option<Vec<Artist>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub description: String,
    pub duration: u32,
    pub embed: bool,
    pub title: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Status {
    Accepted,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DataQuality {
    Correct,
    #[serde(rename="Needs Vote")]
    NeedsVote,
    #[serde(rename="Complete and Correct")]
    CompleteAndCorrect,
}

