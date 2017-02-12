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

#[derive(Debug)]
struct SearchParameters{}

