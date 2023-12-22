/* Include external crates */
use ndarray::{Array1, ArrayView1, s};

/* Include local crates */
use crate::environment::environment::Environment;
use crate::math::vec_math::{l2_norm_array1, normalize_array1};

/* Include constants */
use crate::constants::state::*;

pub fn get_force_in_iframe(state_in: &Array1<f64>, environment: &Environment) -> Array1<f64>
{ 
  /* Initialize Array1 to store gravity force 
   * @unit: m/ss
   * */
  let grav_acceleration_mss: f64 = get_grav_acc(state_in, &environment);

  let mut position_vec_xyz_m: Array1<f64> = Array1::zeros(3);
  position_vec_xyz_m.assign(&state_in.slice(s![STATE_VEC_INDX_POS_X..(STATE_VEC_INDX_POS_Z+1)]));
  let local_radius_m: f64 = l2_norm_array1(position_vec_xyz_m.view());
  
  let sc_mass_kg: f64 = state_in[STATE_VEC_INDX_MASS];
  
  /* Initialise output vector to store the gravitational force  
  * @unit: Newton
  * @frame: PCI
  */
  let mut grav_force_pci_n_out: Array1<f64> = Array1::zeros(3);
  
  /* Check position is at least 10 cm away from the center of the inertial 
  * planet centered frame in which case the gravitational force cannot be 
  * computed
  * */
  if local_radius_m > 0.1
  {
    /* Comppute normalized direction vector of the gravitational acceleration */
    let grav_acc_dir_norm: Array1<f64> = -1.0 * normalize_array1(position_vec_xyz_m);
    /* Compute vector of the gravitational force on the S/C  */
    grav_force_pci_n_out = grav_acceleration_mss * grav_acc_dir_norm * sc_mass_kg; 
  }
  else 
  {
    println!("[ERR] Computing gravitation force failed. PCI Position is [0,0,0]");
  }
  grav_force_pci_n_out
}

/*
 * @brief: Function to compute the local gravitational acceleration magnitude.
 * @unit: m/second_squared
 */
pub fn get_grav_acc(state_in: &Array1<f64>, environment: &Environment)
-> f64
{
  /* Get S/C position in PCI */
  let mut position_vec_xyz_m: Array1<f64> = Array1::zeros(3);
  position_vec_xyz_m.assign(&state_in.slice(s![STATE_VEC_INDX_POS_X..(STATE_VEC_INDX_POS_Z+1)]));

  /* Estimate local radius of the planet at the longitude and latitude of the 
     current S/C's position */
  let mean_radius_m: f64 = ( environment.get_planet().get_semi_major_axis() 
      + environment.get_planet().get_semi_minor_axis()) * 0.5;
  /*
   * Note: We can compare the radii in PCI vs PCPF because we only look at the 
   *       magnitude of both vectors and both coordinate systems share the same 
   *       origin.  
   */
  let local_altitude_m: f64 = l2_norm_array1(position_vec_xyz_m.view())
                              - mean_radius_m;

  /* Initialize Array1 to store gravity force 
  * @unit: m/ss
  * */
  let grav_acceleration_mss: f64 = 9.80665 * (mean_radius_m 
    * (1.0 / (mean_radius_m + local_altitude_m))).powf(2.0); 

  grav_acceleration_mss
}