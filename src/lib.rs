#![feature(proc_macro)]

extern crate hyper;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod label;
mod master;
mod artist;
mod release;
mod pagination;
mod data_structures;

use label::LabelQuery;
use master::MasterQuery;
use artist::ArtistQuery;
use release::ReleaseQuery;
use hyper::Client;
use hyper::client::Response;
use hyper::header::{Headers, UserAgent};
use hyper::client::IntoUrl;
use std::io::Result;

pub struct Discogs {
    api_endpoint: String,
    key: String,
    secret: String,
    user_agent: String,

    // Maximum number of API Queries per minute
    rate_limit: u32,

    // hyper client
    client: Client,
}

impl Discogs {
    pub fn new(key: String, secret: String, user_agent: String) -> Self {
        Discogs {
            api_endpoint: "https://api.discogs.com".to_string(),
            key: key,
            secret: secret,
            user_agent: user_agent,
            rate_limit: 240,
            client: Client::new(),
        }
    }

    pub fn query(&self, url: String) -> Option<Response> {
        // let final_url = format!("{}&key={}&secret={}", url, self.key, self.secret);
        self.client
            .get(&url[..])
            .header(UserAgent(self.user_agent.clone()))
            .send()
            .ok()
    }

    // pub fn call(&self, e: &mut Endpoint) -> Self {
    // let url: String = self.get_api_url(*e);
    // let res = self.client.get(&url[..]).send().unwrap();
    //
    // e.data() = serde_json::from_str(&res).unwrap();
    // }
    //
    // pub fn get_api_url(&self, e: Endpoint) -> String {
    // format!("{}/{}", self.api_endpoint, e.to_string())
    // }
    //
    // pub fn set_useragent(mut self, useragent: String) -> Self {
    //     // self.curl.useragent(&useragent[..]).unwrap();
    //     self
    // }
}

#[cfg(test)]
mod tests {
    use Discogs;
    use release::ReleaseQuery;
    #[test]
    fn discogs_inst() {
        let l: Discogs = Discogs::new("key".to_owned(),
                                      "secret".to_owned(),
                                      "useragent".to_owned());

        let mut at: ReleaseQuery = ReleaseQuery::new(&l);
        for i in 950..1020 {
            print!("{}: ", i);
            if let Some(r) = at.id(i).call() {
                println!("OK");
            } else {
                println!("ERR");
            }
        }
    }
}
