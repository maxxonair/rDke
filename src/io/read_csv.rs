

/*
 * @brief: I/O function to read a single column from a csv into a f64 vector
 * 
 * @returns: Vec<f64> 
 */
pub fn read_csv_column_f64(filepath: &str, has_headers: bool, column_id: usize) 
-> Vec<f64>
{
  let file = std::fs::File::open(filepath).unwrap();
  let mut rdr = csv::ReaderBuilder::new()
     .has_headers(has_headers)
     .from_reader(file);


  let mut vector_out: Vec<f64> = Vec::new();
  // push all the records
  for result in rdr.records().into_iter() {
     let record = result.unwrap();
     let value: f64 = (record[column_id]).parse().unwrap();
     vector_out.push(value);
  }
  vector_out
}