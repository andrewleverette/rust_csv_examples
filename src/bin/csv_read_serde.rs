use std::error::Error;

use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Customer {
    customer_guid: String,
    first_name: String,
    last_name: String,
    email: String,
    address: String,
}

/// Reads data from a file into a reader and deserializes each record
///
/// # Error
///
/// If an error occurs, the error is returned to `main`.
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve and print header record
    let headers = reader.headers()?;
    println!("{:?}", headers);

    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    for result in reader.deserialize() {
        let record: Customer = result?;

        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    // If an error occurs print error
    if let Err(e) = read_from_file("./data/Customers.csv") {
        eprintln!("{}", e);
    }
}
