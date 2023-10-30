/* Include external crates */
use ndarray::{Array1, s};

/* Include local crates */
use crate::dke_core::dke_core::DKE;

/* Include constants */
use crate::constants::state::*;

pub fn get_force_in_iframe(state_in: &Array1<f64>, dke: &DKE) -> Array1<f64>
{ 
  /* Initialize Array1 to store gravity force 
   * @unit: m/ss
   * */
  let grav_acceleration_mss: f64 = get_grav_acc(&state_in, &dke);

  /* Initialise output vector to store the gravitational force  
   * @unit: Newton
   * @frame: PCI
   */
  let mut grav_Force_PCI_N_out: Array1<f64> = Array1::zeros(3);
  /* Create vector to compute gravity direction in PCI */
  let mut grav_acc_dir_norm: Array1<f64> = Array1::zeros(3);
  /* Temporarily assign current state position vector to grav_acc_dir_norm */
  grav_acc_dir_norm.assign(&state_in.slice(s![STATE_VEC_INDX_POS_X..(STATE_VEC_INDX_POS_Z+1)]));
  /* Calculate length of position vector in PCI frame */
  let position_norm: f64 = ((grav_acc_dir_norm[0]).powf(2.0) 
                          + (grav_acc_dir_norm[1]).powf(2.0) 
                          + (grav_acc_dir_norm[2]).powf(2.0)).sqrt();
  /* Check position is at least 10 cm away from the center of the internal 
   * planet centered frame in which case the gravitational force cannot be 
   * computed
   * */
  if position_norm > 0.1
  {
    grav_acc_dir_norm = -1.0 * grav_acc_dir_norm * (1.0 / position_norm);
    grav_Force_PCI_N_out = grav_acceleration_mss * grav_acc_dir_norm * state_in[STATE_VEC_INDX_MASS]; 
  }
  else 
  {
    println!("[ERR] Computing gravitation force failed. PCI Position is [0,0,0]");
  }
  grav_Force_PCI_N_out
}

/*
 * @brief: Function to compute the local gravitational acceleration magnitude.
 * @unit: m/second_squared
 */
pub fn get_grav_acc(state_in: &Array1<f64>, dke: &DKE)
-> f64
{
  let mean_radius_m: f64 = ( dke.get_planet().get_semi_major_axis() 
      + dke.get_planet().get_semi_minor_axis()) * 0.5;
  let local_altitude_m: f64 =   (
        state_in[STATE_VEC_INDX_POS_X].powf(2.0)
      + state_in[STATE_VEC_INDX_POS_Y].powf(2.0)
      + state_in[STATE_VEC_INDX_POS_Z].powf(2.0)).sqrt() 
      - mean_radius_m;

  /* Initialize Array1 to store gravity force 
  * @unit: m/ss
  * */
  let grav_acceleration_mss: f64 = 9.80665 * (mean_radius_m 
    * (1.0/ (mean_radius_m + local_altitude_m))).powf(2.0); 

  grav_acceleration_mss
}