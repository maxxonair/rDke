

use csv::Writer; 
use std::error::Error;
use std::fs::OpenOptions;
use ndarray::Array1;

use crate::constants::constants::*;


pub fn create_csv(file_path_in: String) 
{
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .append(false)
    .open(file_path_in)
    .unwrap();
  csv::Writer::from_writer(file);
  // TODO: Write header 
  // writer.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
}

pub fn append_to_csv(file_path_in: String, data_row_in: &Array1<f64>, simtime_s: f64) 
-> Result<(), Box<dyn Error>>
{
  let mut file = OpenOptions::new()
  .write(true)
  .create(false)
  .append(true)
  .open(file_path_in)
  .unwrap();
  let mut writer = csv::Writer::from_writer(file);

  let mut data_out: [String; STATE_VEC_NUM_ELEMENTS+1] = Default::default();
  data_out[0] = simtime_s.to_string();

  for n in 1..STATE_VEC_NUM_ELEMENTS
  {
    data_out[n] = data_row_in[n].to_string();
  }

  writer.write_record(data_out)?;

  /* TODO move this to have more control when the buffer is flushed */
  writer.flush()?;

  Ok(())
}