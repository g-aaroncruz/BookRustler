use clap::Parser;
use reqwest;
use std::error::Error;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="")]
    isbn: String,
}

#[derive(Deserialize, Debug)]
struct Book {
    title: Option<String>,
    authors: Option<Vec<Author>>,
    publish_date: Option<String>,
    publishers: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct Author {
    key: Option<String>,
    name: Option<String>,
}

fn request_isbn(isbn: String) -> Result<Book, Box<dyn Error>> {
    let body = reqwest::blocking::get(&format!("https://openlibrary.org/isbn/{isbn}.json"))?
        .text()?;
    let json: Book = serde_json::from_str(&body)?;
    Ok(json)
}
fn main() {
    let args = Args::parse();
    assert_eq!(args.isbn.is_empty(), false);
    match request_isbn(args.isbn) {
        Ok(json) => {
            println!("JSON: {:?}", json);
        }
        Err(e) => println!("Error: {}", e),
    }

}
