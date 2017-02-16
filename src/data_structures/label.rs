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

impl Label {
}
