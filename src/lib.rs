#![feature(proc_macro)]

extern crate curl;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub struct Discogs {
    api_endpoint: String,
    useragent: String,
    token: String,
}

impl Discogs {

    pub fn new(token: String, useragent: String) -> Self {
        Discogs {
            api_endpoint: "https://api.discogs.com/".to_string(),
            token: token.to_string(),
            useragent: useragent.to_string(),
        }
    }

}


//// Data structures

#[derive(Serialize, Deserialize)]
pub struct Community {
    pub contributors: Vec<Contributor>,
    pub data_quality: String,
    pub have: u32,
    pub rating: Rating,
    pub status: String,
    pub submitter: Contributor,
    pub want: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Rating {
    pub average: f32,
    pub count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Contributor {
    pub resource_url: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReleaseFormat {
    pub descriptions: Vec<String>,
    pub name: String,
    pub qty: String,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub height: u32,
    pub resource_url: String,
    #[serde(rename = "type")]
    pub image_type: String,
    pub uri: String,
    pub uri150: String,
    pub width: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
    pub catno: String,
    pub entity_type: String,
    pub id: u32,
    pub name: String,
    pub resource_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub duration: String,
    pub position: String,
    pub title: String,
    pub type_: String,
}

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub description: String,
    pub duration: u32,
    pub embed: bool,
    pub title: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct Company {
    pub catno: String,
    pub entity_type: String,
    pub entity_type_name: String,
    pub id: u32,
    pub name: String,
    pub resource_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
    pub anv: String,
    pub id: u32,
    pub join: String,
    pub name: String,
    pub resource_url: String,
    pub role: String,
    pub tracks: String,
}

#[derive(Serialize, Deserialize)]
pub struct Release {
    pub title: String,
    pub id: u32,
    pub artists: Vec<Artist>,
    pub data_quality: String,
    pub thumb: String,
    pub community: Community,
    pub companies: Vec<Company>,
    pub country: String,
    pub date_added: String,
    pub date_changed: String,
    pub estimated_weight: u32,
    pub extraartists: Vec<Artist>,
    pub format_quantity: u32,
    pub formats: Vec<ReleaseFormat>,
    pub genres: Vec<String>,
    pub identifiers: Vec<Identifier>,
    pub images: Vec<Image>,
    pub labels: Vec<Label>,
    pub lowest_price: f64,
    pub master_id: u32,
    pub master_url: String,
    pub notes: String,
    pub num_for_sale: u32,
    pub released: String,
    pub released_formatted: String,
    pub resource_url: String,

    // series: None,//[], ///////////////////////////////////////////////////////////
    pub series: Vec<String>,

    pub status: String,
    pub styles: Vec<String>,
    pub tracklist: Vec<Track>,
    pub uri: String,
    pub videos: Vec<Video>,
    pub year: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
