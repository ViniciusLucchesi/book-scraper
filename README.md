<div align="center">

# Book Scraper

![programming_language](https://img.shields.io/badge/Rust-1.69.0-dca282)
![crate_scraper](https://img.shields.io/badge/scraper-0.16.0-informational)
![crate_reqwest](https://img.shields.io/badge/reqwest-0.11.18-informational)
![crate_csv](https://img.shields.io/badge/csv-1.2.2-informational)


This repository is a training project to develop a **web scraping** that extracts data from the page [books.toscrape.com](https://books.toscrape.com/).

Which is a website created for that purpose.<br />

[Getting started](#getting-started) •
[Scraper crate](#scraper-crate) •
[Documentation Roadmap](#documentation-roadmap)

</div>

## Getting started

Clone the repository

```bash
git clone https://github.com/ViniciusLucchesi/book_scraper.git
```

Compiles the project with all its dependencies defined in the Cargo.toml file and Run it afterwards

```rust
cargo run
```

<br />

## Scraper crate

This crate is an HTML parsing and query with CSS selectors that allow us to extract some information from the HTML passed to it as a parameter.

Scraper provides an interface to Servo’s html5ever and selectors crates, for browser-grade parsing and querying.

Its resources were used in both the main.rs and models.rs files, allowing the creation of the SELECTORS necessary for the project as well as the structuring of a new type called ModelSelector, respectively.

```rust
// src/models.rs
use scraper::Selector;

pub struct ModelSelector {
    pub book: Selector,
    pub book_name: Selector,
    pub book_price: Selector,
    pub book_link: Selector
}
```

```rust
// src/main.rs
fn create_selectors() -> ModelSelector {
    let book_selector: Selector = Selector::parse("article.product_pod").unwrap();
    let book_name_selector: Selector = Selector::parse("h3 a").unwrap();
    let book_price_selector: Selector = Selector::parse(".price_color").unwrap();
    let book_link_selector: Selector = Selector::parse("h3 a").unwrap();
    
    ModelSelector { 
        book: book_selector,
        book_name: book_name_selector,
        book_price: book_price_selector,
        book_link: book_link_selector
    }   
}
```

<br />

## Documentation Roadmap

- [ ]  Explain **reqwest** crate
- [ ]  Explain **csv** crate