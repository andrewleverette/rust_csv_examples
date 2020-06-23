use std::error::Error;
use std::io;

use csv;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Customer {
    customer_guid: String,
    first_name: String,
    last_name: String,
    email: String,
    address: String,
}

/// Inserts a custom type into writer, serializes it and prints to `stdout`
///
/// # Error
///
/// If an error occurs, the error is returned to `main`
fn write_to_stdout() -> Result<(), Box<dyn Error>> {
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_writer(io::stdout());

    // We don't explicitly write the header record
    writer.serialize(Customer {
        customer_guid: "6e49f2fc-00fd-4502-aed7-812da4aacbb8".to_string(),
        first_name: "Ailey".to_string(),
        last_name: "Benstead".to_string(),
        email: "abenstead0@state.gov".to_string(),
        address: "554 Mcguire Center".to_string(),
    })?;

    writer.serialize(Customer {
        customer_guid: "24349324-7e89-412e-b4bd-2a3c6d8e6d96".to_string(),
        first_name: "Ninnette".to_string(),
        last_name: "Wasmuth".to_string(),
        email: "nwasmuth1@washington.edu".to_string(),
        address: "10 Haas Circle".to_string(),
    })?;

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;

    Ok(())
}

fn main() {
    if let Err(e) = write_to_stdout() {
        eprintln!("{}", e)
    }
}
