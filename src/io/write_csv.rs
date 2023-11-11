/*
 * @brief: This file contains a series of functions to handle writing the 
 *         simulation results to csv.
 *
 *
 */

use std::error::Error;
use std::fs::OpenOptions;
use ndarray::Array1;
use std::path::Path;
use std::fs;
use std::fs::File;

/* Include constants */
use crate::constants::state::*;

/*
 * @brief: Function to Create a csv file writer and add the file header 
 *         description.
 */
pub fn create_csv(file_path_in: String) 
-> csv::Writer<File>
{
  /* Check if output file already exists -> if so remove it */
  delete_file_if_exists(&file_path_in).unwrap();

  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open(file_path_in)
    .unwrap();
  let mut writer_out = csv::Writer::from_writer(file);

  /* Write csv header */
  write_header_to_csv(&mut writer_out).unwrap();

  writer_out
}

/*
 * @brief: Function to append a row to csv file writer (writer_in)
 */
pub fn append_to_csv(writer_in: &mut csv::Writer<File>,
                    data_row_in: &Array1<f64>,
                    simtime_s: f64) 
-> Result<(), Box<dyn Error>>
{
  let mut data_out: [String; STATE_VEC_NUM_ELEMENTS] = Default::default();

  for n in 0..STATE_VEC_NUM_ELEMENTS
  {
    data_out[n] = data_row_in[n].to_string();
  }

  writer_in.write_record(data_out)?;

  Ok(())
}

/*
 * @brief: Function to write csv header  
 */
pub fn write_header_to_csv(writer_in: &mut csv::Writer<File>)
-> Result<(), Box<dyn Error>>
{
  /* Write csv header */
  writer_in.write_record(&["sim_time_s", 
                            "pos_x_iframe",
                            "pos_y_iframe", 
                            "pos_z_iframe", 
                            "vel_x_iframe",
                            "vel_y_iframe",
                            "vel_z_iframe",
                            "acc_x_iframe",
                            "acc_y_iframe",
                            "acc_z_iframe",
                            "att_q_x_iframe2bframe",
                            "att_q_y_iframe2bframe",
                            "att_q_z_iframe2bframe",
                            "att_q_w_iframe2bframe",
                            "att_rate_x_bframe",
                            "att_rate_y_bframe",
                            "att_rate_z_bframe",
                            "att_acc_x_bframe",
                            "att_acc_y_bframe",
                            "att_acc_z_bframe",
                            "mass_kg",
                            "j2000_times_s",
                            "altitude_pcpf_m",
                            "latitude_pcpf_deg",
                            "longitude_pcpf_deg",
                            "magn_grav_acc_mss",
                            "gast_deg"
                            ])?;

 Ok(())
}

/*
 * @brief: Function to flush file writer buffer. 
 *         Note: This should be called at SIMULATION_WRITE_FLUSH_INTERVAL_S 
 *               and NOT every time step. 
 * 
 */
pub fn flush_csv_writer(writer_in: &mut csv::Writer<File>) 
-> Result<(), Box<dyn Error>>
{
  /* TODO move this to have more control when the buffer is flushed */
  writer_in.flush()?;

  Ok(())
}

/*
 * @brief: Function to delete a file at file_path_in if that file exits and 
 *         do nothing if it doesn't exit.
 * 
 *
 */
pub fn delete_file_if_exists(file_path_in: &String)  
-> Result<(), Box<dyn Error>>
{
  if Path::new(&file_path_in).exists()
  {
    fs::remove_file(&file_path_in)?;
  }
  Ok(())
}