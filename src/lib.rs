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

#[macro_use]
extern crate serde_derive;

mod label;
// mod master;
// mod artist;
// mod release;
mod pagination;
mod data_structures;

use label::LabelQuery;
// use master::MasterQuery;
// use artist::ArtistQuery;
// use release::ReleaseQuery;
use hyper::Client;
use hyper::client::Response;
use hyper::header::UserAgent;

trait Queryable {
    fn update(&mut self, d: &Discogs);
    fn get_address(&self) -> String;
}

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

    pub fn key(&mut self, key: String) -> &mut Self {
        self.key = Some(key);
        self
    }

    pub fn secret(&mut self, secret: String) -> &mut Self {
        self.secret = Some(secret);
        self
    }

    pub fn rate_limit(&mut self, rate_limit: u32) -> &mut Self {
        self.rate_limit = rate_limit;
        self
    }

    pub fn api_endpoint(&mut self, api_endpoint: String) -> &mut Self {
        self.api_endpoint = api_endpoint;
        self
    }

    pub fn query(&self, url: String) -> Option<Response> {
        // let final_url = format!("{}&key={}&secret={}", url, self.key, self.secret);
        self.client
            .get(&url[..])
            .header(UserAgent(self.user_agent.clone()))
            .send()
            .ok()
    }

    //    pub fn label(&self) -> LabelQuery {
    //        LabelQuery::new(self)
    //    }
    //
    //    pub fn master(&self) -> MasterQuery {
    //        MasterQuery::new(self)
    //    }
    //
    //    pub fn artist(&self) -> ArtistQuery {
    //        ArtistQuery::new(self)
    //    }
    //
    //    pub fn release(&self) -> ReleaseQuery {
    //        ReleaseQuery::new(self)
    //    }
}

#[cfg(test)]
mod tests {
    use Discogs;
    //    use release::ReleaseQuery;
    use data_structures::Master;
    use Queryable;
    #[test]
    fn discogs_inst() {
        let l: Discogs = Discogs::new("useragent".to_owned());

        let mut at = Master {
            id: 1016,
            resource_url: "https://api.discogs.com/masters/1016".to_owned(),
            main_release: 36287310,
            title: None,
            year: None,
            images: None,
            tracklist: None,
            uri: None,
            genres: None,
            artists: None,
            notes: None,
            videos: None,
            data_quality: None,
            num_for_sale: None,
            styles: None,
            versions_url: None,
            main_release_url: None,
            lowest_price: None,
        };
        println!("{:?}", at.main_release);
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
