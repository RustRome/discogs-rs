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

    pub fn label(&self) -> LabelQuery {
        LabelQuery::new(self)
    }

    pub fn master(&self) -> MasterQuery {
        MasterQuery::new(self)
    }

    pub fn artist(&self) -> ArtistQuery {
        ArtistQuery::new(self)
    }

    pub fn release(&self) -> ReleaseQuery {
        ReleaseQuery::new(self)
    }
}

#[cfg(test)]
mod tests {
    use Discogs;
    use release::ReleaseQuery;
    #[test]
    fn discogs_inst() {
        let l: Discogs = Discogs::new("useragent".to_owned());

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
