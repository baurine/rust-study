use super::*;

pub fn replace_column(data: String, column: &str, replacement: &str) -> Result<String, Error> {
  let mut lines = data.lines();
  let header = lines.next().unwrap();
  let columns: Vec<&str> = header.split(',').collect();
  let column_index = columns.iter().position(|&e| e == column); // finds ??
  let column_index = match column_index {
    Some(x) => x,
    None => Err("column name doesn't exist in the input file")?,
  };
  let mut result = String::with_capacity(data.capacity());
  result.push_str(&columns.join(","));
  result.push('\n');
  for line in lines {
    let mut records: Vec<&str> = line.split(',').collect();
    records[column_index] = replacement;
    result.push_str(&records.join(","));
    result.push('\n');
  }
  Ok(result)
}
