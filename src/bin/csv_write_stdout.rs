use std::error::Error;
use std::io;

use csv;

/// Inserts data into writer and prints to `stdout`
///
/// # Error
///
/// If an error occurs, the error is returned to `main`
fn write_to_stdout() -> Result<(), Box<dyn Error>> {
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_writer(io::stdout());

    // Write records one at a time including the header record.
    writer.write_record(&[
        "customer_guid",
        "first_name",
        "last_name",
        "email",
        "address",
    ])?;
    writer.write_record(&[
        "6e49f2fc-00fd-4502-aed7-812da4aacbb8",
        "Ailey",
        "Benstead",
        "abenstead0@state.gov",
        "554 Mcguire Center",
    ])?;
    writer.write_record(&[
        "24349324-7e89-412e-b4bd-2a3c6d8e6d96",
        "Ninnette",
        "Wasmuth",
        "nwasmuth1@washington.edu",
        "10 Haas Circle",
    ])?;

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
