use clap::Parser;
use reqwest;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="")]
    isbn: String,
}

fn request_isbn(isbn: String) -> Result<(), Box<dyn Error>> {
    println!("{}", isbn);
    let body = reqwest::blocking::get(&format!("https://openlibrary.org/isbn/{isbn}.json"))?
        .text()?;
    println!("body = {body:?}");
    Ok(())
}
fn main() {
    let args = Args::parse();
    assert_eq!(args.isbn.is_empty(), false);
    if let Err(e) = request_isbn(args.isbn) {
        eprintln!("Error: {}", e);
    }
}
