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
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

extern crate hyper;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod query_builder;
pub mod pagination;
pub mod data_structures;

use hyper::Client;
use hyper::header::UserAgent;
use std::io::Read;
use hyper::status::StatusCode;
use serde::Serialize;
use serde::Deserialize;
pub use query_builder::{QueryBuilder, QueryType};

pub struct Discogs {
    api_endpoint: String,
    user_agent: String,

    key: Option<String>,
    secret: Option<String>,

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
    /// #[allow(unused_variables)]
    /// use discogs::Discogs;
    ///
    /// let client = Discogs::new(env!("DISCOGS_USER_AGENT"));
    /// ```
    pub fn new(user_agent: &str) -> Self {
        Discogs {
            api_endpoint: "https://api.discogs.com".to_owned(),
            key: None,
            secret: None,
            user_agent: user_agent.to_owned(),
            rate_limit: 240,
            client: Client::new(),
        }
    }

    /// Sets the discogs api client key
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let mut client = Discogs::new(env!("DISCOGS_USER_AGENT"));
    /// client.key(env!("DISCOGS_CLIENT_KEY"));
    /// ```
    //TODO: Come back and make a better example
    pub fn key(&mut self, key: &str) -> &mut Self {
        self.key = Some(key.to_owned());
        self
    }
    /// Sets the discogs api client secret
    ///
    /// # Examples
    ///
    /// ```
    /// use discogs::Discogs;
    ///
    /// let mut client = Discogs::new(env!("DISCOGS_USER_AGENT"));
    /// client.secret(env!("DISCOGS_CLIENT_SECRET"));
    /// ```
    //TODO: Come back and make a better example
    pub fn secret(&mut self, secret: &str) -> &mut Self {
        self.secret = Some(secret.to_owned());
        self
    }
}

#[cfg(test)]
mod tests {
    use Discogs;

    #[test]
    fn user_agent_test() {
        let client = Discogs::new(env!("DISCOGS_USER_AGENT"));

        assert_eq!(client.user_agent, env!("DISCOGS_USER_AGENT"));
    }

    #[test]
    fn key_test() {
        let mut client = Discogs::new(env!("DISCOGS_USER_AGENT"));
        client.key(env!("DISCOGS_CLIENT_KEY"));

        assert_eq!(client.user_agent, env!("DISCOGS_USER_AGENT"));
        assert_eq!(client.key, Some(env!("DISCOGS_CLIENT_KEY").to_owned()));
    }


    #[test]
    fn secret_test() {
        let mut client = Discogs::new(env!("DISCOGS_USER_AGENT"));
        client.secret(env!("DISCOGS_CLIENT_SECRET"));

        assert_eq!(client.user_agent, env!("DISCOGS_USER_AGENT"));
        assert_eq!(client.secret, Some(env!("DISCOGS_CLIENT_SECRET").to_owned()));

    }
}

//    /// Constructs a new `QueryBuilder` from the client
//    ///
//    /// # Examples
//    ///
//    /// ```
//    /// use discogs::Discogs;
//    ///
//    /// let query = Discogs::new(env!("DISCOGS_USER_AGENT")).query();
//    /// ```
//    pub fn query(&self) -> QueryBuilder {
//        QueryBuilder {
//
//        }
//    }

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

//#[cfg(test)]
//mod tests {
//    use Discogs;
//    //    use release::ReleaseQuery;
//    use data_structures::master::Master;
//    use data_structures::artist::Artist;
//    #[test]
//    fn discogs_inst() {
//        let l: Discogs = Discogs::new("useragent".to_owned());
//        let mut at = Master::new(1016, &l);
//        let mut i = Artist::new(1020, &l);
//
//        println!("{:?}", i);
//        at.update(&l);
//        println!("{:?}", at.main_release);
//        //        for i in 950..1020 {
//        //            print!("{}: ", i);
//        //            if let Some(r) = at.id(i).call() {
//        //                println!("OK");
//        //            } else {
//        //                println!("ERR");
//        //            }
//        //        }
//    }
//}
