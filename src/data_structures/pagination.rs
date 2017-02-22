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

use serde_json;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub per_page: i16,
    pub page: i16,
    pub items: i64,
    pub pages: i16,
    pub urls: PaginationUrls,
}

#[derive(Deserialize, Debug)]
pub struct PaginationUrls {
    pub next: Option<String>,
    pub last: Option<String>,
}
