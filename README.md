# Working with CSV Data in Rust

## Reading CSV

### Reading from `stdin`

[*/src/bin/csv_read_stdin.rs*](https://github.com/andrewleverette/rust_csv_examples/blob/master/src/bin/csv_read_stdin.rs)
```rust
/// Reads data from `stdin` into a reader and prints all records.
///
/// # Error
///
/// If an error occurs, the error is returned to `main`.
fn read_from_stdin() -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from `stdin`
    let mut reader = csv::Reader::from_reader(io::stdin());

    let headers = reader.headers()?;

    println!("Headers: {:?}", headers);

    // `.records` return an iterator of the internal
    // record structure
    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}
```

### Reading from File

[*/src/bin/csv_read_file.rs*](https://github.com/andrewleverette/rust_csv_examples/blob/master/src/bin/csv_read_file.rs)
```rust
/// Reads data from a file into a reader and prints all records.
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

    // `.records` return an iterator of the internal
    // record structure
    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}
```

### Reading with Serde

[*/src/bin/csv_read_serde.rs*](https://github.com/andrewleverette/rust_csv_examples/blob/master/src/bin/csv_read_serde.rs)
```rust
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
```

## Writing CSV Data

### Writing to `stdout`

[*/src/bin/csv_write_stdout.rs*](https://github.com/andrewleverette/rust_csv_examples/blob/master/src/bin/csv_write_stdout.rs)
```rust
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
```

### Write to File

[*/src/bin/csv_write_file.rs*](https://github.com/andrewleverette/rust_csv_examples/blob/master/src/bin/csv_write_file.rs)
```rust
/// Inserts data into writer and writes to a file
///
/// # Error
///
/// If an error occurs, the error is returned to `main`
fn write_to_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_path(path)?;

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
```

### Writing with Serde

[*/src/bin/csv_write_serde.rs*](https://github.com/andrewleverette/rust_csv_examples/blob/master/src/bin/csv_write_serde.rs)
```rust
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
```

## CSV Aggregation

[*/src/bin/csv_aggregation.rs*](https://github.com/andrewleverette/rust_csv_examples/blob/master/src/bin/csv_aggregation.rs)
```rust
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

    /// Sort data records by the given index.Aggregate
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
                continue;
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
```