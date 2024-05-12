use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn parse_json<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn Error>> {
    let json_data = fs::read_to_string(path).map_err(|e| Box::new(e) as Box<dyn Error>)?;

    serde_json::from_str(&json_data).map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub fn save_json<T: Serialize>(data: &T, path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::create(path).map_err(|e| Box::new(e) as Box<dyn Error>)?;

    let formatter = PrettyFormatter::with_indent(b"  ");
    let mut serializer = serde_json::Serializer::with_formatter(file, formatter);

    data.serialize(&mut serializer)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    Ok(())
}
