use serde_json;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub per_page: i16,
    pub page: i16,
    pub items: i64,
    pub pages: i16,
    pub urls: PaginationUrls,
}

#[derive(Deserialize, Debug)]
pub struct PaginationUrls {
    pub next: Option<String>,
    pub last: Option<String>,
}
