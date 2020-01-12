use super::{Error, File, PathBuf, Read, Write};

pub fn load_csv(csv_file: PathBuf) -> Result<String, Error> {
  let file = read(csv_file)?;
  Ok(file)
}

fn read(path: PathBuf) -> Result<String, Error> {
  let mut buffer = String::new();
  let mut file = open(path)?;
  file.read_to_string(&mut buffer)?;
  if buffer.is_empty() {
    return Err("input file missing")?;
  }
  Ok(buffer)
}

fn open(path: PathBuf) -> Result<File, Error> {
  let file = File::open(path)?;
  Ok(file)
}

////////

/// # Usage
/// ```ignore
/// let filename = PathBuf::from("./input/challenge.csv");
/// let csv_data = load_csv(filename).unwrap();
/// let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
/// let output_file = write_csv(&modified_data, "output/test.csv");
/// assert!(output_file.is_ok());
/// ```
pub fn write_csv(csv_data: &str, filename: &str) -> Result<(), Error> {
  write(csv_data, filename)?;
  Ok(())
}

fn write(data: &str, filename: &str) -> Result<(), Error> {
  let mut buffer = File::create(filename)?;
  buffer.write_all(data.as_bytes())?;
  Ok(())
}

//////

#[cfg(test)]
mod test {
  use super::load_csv;
  use std::path::PathBuf;

  #[test]
  fn test_valid_load_csv() {
    let filename = PathBuf::from("./input/challenge.csv");
    let csv_data = load_csv(filename);
    assert!(csv_data.is_ok());
  }
}
