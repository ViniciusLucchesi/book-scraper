use reqwest::blocking::Response;
use scraper::{Html, Selector, ElementRef};

mod models;
use crate::models::{ModelResponse, ModelSelector};


fn get_html_page(url: &str) -> String {
    let response: Response = reqwest::blocking::get(url).expect("Could not load url.");
    response.text().unwrap()
}

fn create_selectors() -> ModelSelector {
    let book_selector: Selector = Selector::parse("article.product_pod").unwrap();
    let book_name_selector: Selector = Selector::parse("h3 a").unwrap();
    let book_price_selector: Selector = Selector::parse(".price_color").unwrap();
    let book_link_selector: Selector = Selector::parse(".image_container a").unwrap();
    
    ModelSelector {
        book: book_selector,
        book_name: book_name_selector,
        book_price: book_price_selector,
        book_link: book_link_selector
    }    
}

fn generate_csv_file(document: Html, response: ModelResponse, selector: ModelSelector) {    
    let mut wtr = csv::Writer::from_path("books.csv").unwrap();
    wtr.write_record(&["Book name", "Price", "Link"]).unwrap();

    // Extracting products description
    for element in document.select(&selector.book) {
        let book_name_element: ElementRef = element.select(&selector.book_name).next().expect("Could not select book name.");
        let book_name: &str = book_name_element.value().attr("title").expect("Could not find title attribute.");
        let price_element: ElementRef = element.select(&selector.book_price).next().expect("Could not find price");
        let price: String = price_element.text().collect::<String>();
        let book_link_element: ElementRef = element.select(&selector.book_link).next().expect("Could not find book link element.");
        let book_link: &str = book_link_element.value().attr("href").expect("Cound not find href attribute.");
        let link: String = format!("{}/{}", response.url, book_link);
        wtr.write_record([book_name, &price, &link]).unwrap();
    }
    wtr.flush().expect("Could not close file.");
    
}


fn main() {
    let url: &str = "https://books.toscrape.com";
    let body: String = get_html_page(url);
    let response: ModelResponse = ModelResponse::new(body, url);

    let selector: ModelSelector = create_selectors();
    let document: Html = Html::parse_document(&response.body);

    generate_csv_file(document, response, selector);
}
