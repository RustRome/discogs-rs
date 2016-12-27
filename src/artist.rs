// TODO: Implement versions api for artists

use pagination::Pagination;
use data_structures::Artist;
use hyper::status::StatusCode;
use Discogs;
use serde_json;
use std::io::Read;

const API_ENDPOINT: &'static str = "/artists";

pub struct ArtistQuery<'a> {
    discogs: &'a Discogs,
    id: u32,
    pagination: Option<Pagination>,
}

impl<'a> ArtistQuery<'a> {
    pub fn new(d: &'a Discogs) -> Self {
        ArtistQuery {
            discogs: d,
            id: 0,
            pagination: None,
        }
    }

    pub fn call(&self) -> Option<Artist> {
        let r = self.discogs
            .query(format!("{}{}/{}", self.discogs.api_endpoint, API_ENDPOINT, self.id).to_owned());

        if r.is_none() {
            return None;
        }

        if let Some(mut json) = r {
            if json.status != StatusCode::Ok {
                return None;
            }

            let mut s: String = "".to_owned();

            if let Ok(sz) = json.read_to_string(&mut s) {
                if sz > 0 {
                    return serde_json::from_str(&s[..]).ok();
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        None
    }

    pub fn id(&mut self, id: u32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn pagination(&mut self, pagination: Pagination) -> &mut Self {
        self.pagination = Some(pagination);
        self
    }
}
