// Library that eases the use of discogs API
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

#![doc(test(attr(allow(unused_variables), deny(warnings))))]

extern crate hyper;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod pagination;
pub mod data_structures;
pub mod query;
pub mod discogs;

pub use discogs::*;
