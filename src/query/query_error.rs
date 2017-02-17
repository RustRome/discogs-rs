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

use serde_json;
use hyper;
use std;

/// `QueryError` is a structure of all the errors
/// that are possible during a query
// TODO: Document when these errors would occur
#[derive(Debug)]
pub enum QueryError {
    JsonDecodeError {
        serde_err: Option<serde_json::Error>
    },
    HyperSendError {
        hyper_err: hyper::Error
    },
    HyperStatusError {
        response: hyper::client::response::Response
    },
    EmptyResponseError,
    TextReadError {
        error: std::io::Error
    }
}
