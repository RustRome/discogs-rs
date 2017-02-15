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

#![feature(proc_macro)]

extern crate hyper;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod query_builder;
mod pagination;
pub mod data_structures {
    pub mod master;
    pub mod artist;
    pub mod company;
    pub mod contributor;
    pub mod image;
    pub mod label;
    pub mod others;
    pub mod release;
}

use hyper::Client;
use hyper::header::UserAgent;
use std::io::Read;
use hyper::status::StatusCode;
use serde::Serialize;
use serde::Deserialize;

pub struct Discogs {
    api_endpoint: String,
    key: Option<String>,
    secret: Option<String>,
    user_agent: String,

    // Maximum number of API Queries per minute
    rate_limit: u32,

    // hyper client
    client: Client,
}

impl Discogs {
    /// Constructs a new `Client` with the provided `user_agent`.
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let client = Discogs::new(env!("DISCOGS_USER_AGENT"));
    /// ```
    pub fn new(user_agent: String) -> Self {
        Discogs {
            api_endpoint: "https://api.discogs.com".to_string(),
            key: None,
            secret: None,
            user_agent: user_agent,
            rate_limit: 240,
            client: Client::new(),
        }
    }

    /// Constructs a new `QueryBuilder` from the client
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let query = Discogs::new(env!("DISCOGS_USER_AGENT")).query();
    /// ```
    pub fn query(&self) -> QueryBuilder {
        QueryBuilder {
            
        }
    }

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

}

#[cfg(test)]
mod tests {
    use Discogs;
    //    use release::ReleaseQuery;
    use data_structures::master::Master;
    use data_structures::artist::Artist;
    use Queryable;
    #[test]
    fn discogs_inst() {
        let l: Discogs = Discogs::new("useragent".to_owned());
        let mut at = Master::new(1016, &l);
        let mut i = Artist::new(1020, &l);

        println!("{:?}", i);
        at.update(&l);
        println!("{:?}", at.main_release);
        //        for i in 950..1020 {
        //            print!("{}: ", i);
        //            if let Some(r) = at.id(i).call() {
        //                println!("OK");
        //            } else {
        //                println!("ERR");
        //            }
        //        }
    }
}
