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
use query::query_auth::*;
use hyper;
use hyper::header::*;


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

    fn perform_request(&self) -> Result<String, QueryError> {
        let client = hyper::Client::new();
        let response = client.get(self.get_query_url().as_str())
                             .header(UserAgent(self.get_user_agent()))
                             .header(Authorization(DiscogsKSAuth {
                                         key: self.get_key(),
                                         secret: self.get_secret()
                             }))
                             .send();

//    pub fn query_url(&self, url: String) -> Option<String> {
//        // let final_url = format!("{}&key={}&secret={}", url, self.key, self.secret);
//        let response = self.client
//            .get(&url[..])
//            .header(UserAgent(self.user_agent.clone()))
//            .send()
//            .ok();
//
//        if let Some(mut json) = response {
//
//            if json.status != StatusCode::Ok {
//                return None;
//            }
//
//            let mut s: String = "".to_owned();
//            if let Ok(sz) = json.read_to_string(&mut s) {
//                if sz <= 0 {
//                    return None;
//                }
//                return Some(s);
//            }
//        }
//        return None;
//    }
//
//    pub fn query(&self, qs: QuerySource) -> Option<String> {
//        self.query_url(qs.get_address())
//    }
        Ok("ads".to_string())
    }
}
