use std::error::Error;
use std::fmt;
use std::process;

use csv::{Reader, StringRecord, Writer};

/// A simple error handler structure
#[derive(Debug)]
struct IndexError(String);

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index Error: {}", self.0)
    }
}

impl Error for IndexError {}

/// Internal data set to make aggregation simpler
#[derive(Debug)]
struct DataSet {
    /// Header row of CSV file
    headers: StringRecord,

    /// Records from CSV file
    records: Vec<StringRecord>,
}

impl DataSet {
    /// Creates a new data set
    fn new(headers: StringRecord, records: Vec<StringRecord>) -> Self {
        DataSet { headers, records }
    }

    /// Finds the index of a column given the column name
    ///
    /// # Arguments
    ///
    /// * `key` -> The column name
    ///
    /// # Errors
    ///
    /// An error occurs if column name does not exist.
    fn key_index(&self, key: &str) -> Result<usize, Box<dyn Error>> {
        match self.headers.iter().position(|column| column == key) {
            Some(index) => Ok(index),
            None => Err(Box::new(IndexError(format!(
                "Column '{}' does not exist.",
                key
            )))),
        }
    }

    /// Sort data records by the given index.
    ///
    /// # Errors
    ///
    /// An error occurs if the index is out of bounds
    fn sort_by_index(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        if index >= self.headers.len() {
            Err(Box::new(IndexError(format!(
                "Index '{}' out of bounds",
                index
            ))))
        } else {
            self.records.sort_by(|a, b| a[index].cmp(&b[index]));
            Ok(())
        }
    }
}

/// This trait defines aggregation methods for the internal data set
trait Aggregate {
    fn inner_join(&mut self, right: &mut Self, key: &str) -> Result<DataSet, Box<dyn Error>>;
}

impl Aggregate for DataSet {
    /// Performs an inner join on two data sets, where `self` is the left table.
    ///
    /// # Arguments
    ///
    /// * `right` -> The right data set for the join
    /// * `key` -> The column name to join on
    fn inner_join(&mut self, right: &mut Self, key: &str) -> Result<DataSet, Box<dyn Error>> {
        // Get column index
        let left_index = self.key_index(key)?;
        let right_index = right.key_index(key)?;

        // Merge headers
        let headers = StringRecord::from(
            self.headers
                .iter()
                .chain(right.headers.iter())
                .collect::<Vec<&str>>(),
        );

        let mut records = vec![];

        if self.records.is_empty() || right.records.is_empty() {
            return Ok(DataSet::new(headers, records));
        }

        // Sort data sets by the column index
        // Required to for this sort algorithm
        self.sort_by_index(left_index)?;
        right.sort_by_index(right_index)?;

        let mut left_cursor = 0;
        let mut right_cursor = 0;

        while left_cursor < self.records.len() && right_cursor < right.records.len() {
            // If two fields match, merge fields into a single record
            // and add to records vector
            // If they don't match and the left value is less then right value advance the left cursor
            // else advance the right cursor
            if self.records[left_cursor][left_index] == right.records[right_cursor][right_index] {
                let record = StringRecord::from(
                    self.records[left_cursor]
                        .iter()
                        .chain(right.records[right_cursor].iter())
                        .collect::<Vec<&str>>(),
                );

                records.push(record);

                // Since data sets are sorted
                // Advance cursor through right data set to
                // see if there are matches
                let mut k = right_cursor + 1;
                while k < right.records.len()
                    && self.records[left_cursor][left_index] == right.records[k][right_index]
                {
                    let record = StringRecord::from(
                        self.records[left_cursor]
                            .iter()
                            .chain(right.records[k].iter())
                            .collect::<Vec<&str>>(),
                    );

                    records.push(record);

                    k += 1;
                }

                left_cursor += 1;
            } else if self.records[left_cursor][left_index]
                < right.records[right_cursor][right_index]
            {
                left_cursor += 1;
            } else {
                right_cursor += 1;
            }
        }

        Ok(DataSet::new(headers, records))
    }
}

/// Reads csv data from a file and returns a DataSet
fn read_from_file(path: &str) -> Result<DataSet, Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;

    let headers = reader.headers()?.clone();

    let records = reader
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    Ok(DataSet { headers, records })
}

/// Converts given DataSet to CSV and writes to file
fn write_to_file(data: DataSet, path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(path)?;

    writer.write_record(data.headers.iter())?;

    for record in data.records {
        writer.write_record(record.iter())?;
    }

    Ok(())
}

fn main() {
    // Read customers
    let mut customers = match read_from_file("./data/Customers.csv") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Read orders
    let mut orders = match read_from_file("./data/Orders.csv") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Join records
    let result = match customers.inner_join(&mut orders, "customer_guid") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Write results to file
    if let Err(e) = write_to_file(result, "./data/JoinedRecords.csv") {
        eprintln!("{}", e);
        process::exit(1);
    } else {
        println!("Inner Join Complete");
    }
}
