

use crate::constants::state::*;

use ndarray::{Array1, s};

pub fn get_force_in_iframe(state_in: &Array1<f64>) -> Array1<f64>
{
  /* Initialize Array1 to store gravity force 
   * @unit: m/ss
   * */
  let grav_acceleration_mss: f64 = 9.80665; // TODO call as function of current position

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