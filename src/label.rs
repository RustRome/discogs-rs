use pagination::Pagination;
use data_structures::Label;
use hyper::status::StatusCode;
use Discogs;
use serde_json;
use std::io::Read;

const API_ENDPOINT: &'static str = "/labels";

pub struct LabelQuery<'a> {
    discogs: &'a Discogs,
    id: u32,
    pagination: Option<Pagination>,
}

impl<'a> LabelQuery<'a> {
    pub fn new(d: &'a Discogs) -> Self {
        LabelQuery {
            discogs: d,
            id: 0,
            pagination: None,
        }
    }

    pub fn call(&self) -> Option<Label> {
        let mut r = self.discogs
            .query(format!("https://api.discogs.com/labels/{}", self.id).to_owned());

        if r.status != StatusCode::Ok {
            return None;
        }

        let mut s: String = "".to_owned();

        r.read_to_string(&mut s);

        serde_json::from_str(&s[..]).unwrap()
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
