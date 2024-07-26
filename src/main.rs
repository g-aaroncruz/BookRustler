use clap::Parser;
use reqwest;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="")]
    isbn: String,
}

fn request_isbn(isbn: String) {
    println!("{}", isbn);
    let body = reqwest::blocking::get("https://openlibrary.org/isbn/{isbn}")?
        .text()?;
    println!("body = {body:?}");
}
fn main() {
    let args = Args::parse();
    assert_eq!(args.isbn.is_empty(), false);
    request_isbn(args.isbn);
}
