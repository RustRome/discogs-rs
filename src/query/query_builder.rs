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

use query::QueryError;
use query::*;
use hyper;
use hyper::header::*;
use hyper::status::StatusCode;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

pub trait QueryBuilder {
    fn get_key(&self) -> Option<String> {
        None
    }

    fn get_secret(&self) -> Option<String> {
        None
    }

    // returns the  url to perform the query
    fn get_query_url(&self) -> String;

    fn get_user_agent(&self) -> String;

    //TODO: Fix the unwrap()
    fn perform_request(&self) -> Result<String, QueryError> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);
        let response = client.get(self.get_query_url().as_str())
                             .header(UserAgent(self.get_user_agent()))
                             .header(Authorization(DiscogsKSAuth {
                                         key: self.get_key(),
                                         secret: self.get_secret()
                             }))
                             .send();

        match response {
            Ok(mut text) => {
                let mut json: String = "".to_owned();

                if text.status != StatusCode::Ok {
                    return Err(QueryError::HyperStatusError {
                        response: text
                    });
                }
                let text_read_result = text.read_to_string(&mut json);

                if let Ok(sz) = text_read_result {
                    if sz <= 0 {
                        return Err(QueryError::EmptyResponseError);
                    }
                    return Ok(json);
                } else {
                    return Err(QueryError::TextReadError {
                        error: text_read_result.err().unwrap()
                    });
                }


            },
            Err(error) => return Err(QueryError::HyperSendError {
                hyper_err: error
            })
        }
    }
}
