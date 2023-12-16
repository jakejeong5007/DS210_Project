use std::error::Error;
use std::fs::File;
use std::io::Read;
use csv::ReaderBuilder;

// Function to load data from a CSV file
pub fn load_data(file_path: &str) -> Result<Vec<(u32, u32)>, Box<dyn Error>> {
    let mut edges = Vec::new();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(File::open(file_path)?);

    for result in rdr.deserialize() {
        let record: (u32, u32) = result?;
        edges.push(record);
    }

    Ok(edges)
}
