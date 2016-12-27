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

// TODO: Implement versions api for masters

use pagination::Pagination;
use data_structures::Master;
use hyper::status::StatusCode;
use Discogs;
use serde_json;
use std::io::Read;

const API_ENDPOINT: &'static str = "/masters";

pub struct MasterQuery<'a> {
    discogs: &'a Discogs,
    id: u32,
    pagination: Option<Pagination>,
}

impl<'a> MasterQuery<'a> {
    pub fn new(d: &'a Discogs) -> Self {
        MasterQuery {
            discogs: d,
            id: 0,
            pagination: None,
        }
    }

    pub fn call(&self) -> Option<Master> {
        let r = self.discogs
            .query(format!("{}{}/{}", self.discogs.api_endpoint, API_ENDPOINT, self.id).to_owned());

        if r.is_none() {
            return None;
        }

        if let Some(mut json) = r {
            if json.status != StatusCode::Ok {
                return None;
            }

            let mut s: String = "".to_owned();

            if let Ok(sz) = json.read_to_string(&mut s) {
                if sz > 0 {
                    return serde_json::from_str(&s[..]).ok();
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        None
    }

    pub fn id(&mut self, id: u32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn pagination(&mut self, pagination: Pagination) -> &mut Self {
        self.pagination = Some(pagination);
        self
    }
}
