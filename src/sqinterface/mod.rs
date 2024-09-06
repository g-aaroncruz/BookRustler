use sqlite::{State, Connection} ;

pub fn open_dbs() {
    let connection = sqlite::open(":memory:");
    let query = r"
        CREATE TABLE books (id INTEGER PRIMARY KEY, title TEXT, created DATE, publish_year DATE, updated DATE);
        CREATE TABLE authors (id INTEGER PRIMARY KEY, name TEXT UNIQUE);
        CREATE TABLE authorship (id INTEGER PRIMARY KEY, author_id INTEGER, book_id INTEGER);
    ";
    connection.execute(query).unwrap();
    // let q = "SELECT * FROM books;";
    // let mut statement = connection.prepare(q).unwrap();
    // while let Ok(State::Row) = statement.next() {
    //     println!("id = {}", statement.read::<i64, _>("id").unwrap());
    //     println!("name = {}", statement.read::<String, _>("name").unwrap());
    //     println!("created = {}", statement.read::<String, _>("created").unwrap());
    //     println!("publish_year = {}", statement.read::<String, _>("publish_year").unwrap());
    //     println!("updated = {}", statement.read::<String, _>("updated").unwrap());
    // }
}
