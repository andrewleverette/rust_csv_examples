use std::error::Error;

use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Order {
    order_guid: String,
    customer_guid: String,
    order_date: String,

    #[serde(deserialize_with = "csv::invalid_option")]
    total: Option<f64>,
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
        let record: Order = result?;

        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    // If an error occurs print error
    if let Err(e) = read_from_file("./data/Orders.csv") {
        eprintln!("{}", e);
    }
}
