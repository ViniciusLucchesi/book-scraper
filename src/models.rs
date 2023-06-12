use scraper::Selector;

pub struct ModelSelector {
    pub book: Selector,
    pub book_name: Selector,
    pub book_price: Selector,
    pub book_link: Selector
}

pub struct ModelResponse<'a> {
    pub body: String,
    pub url: &'a str
}

impl ModelResponse<'_> {
    pub fn new(body: String, url: &str) -> ModelResponse {
        ModelResponse {body, url}
    }
}