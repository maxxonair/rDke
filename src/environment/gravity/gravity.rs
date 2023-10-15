

use crate::constants::constants::STATE_VEC_INDX_MASS;

use ndarray::Array1;

pub fn get_force_in_iframe(state_in: &Array1<f64>) -> Array1<f64>
{
  /* Initialize Array1 to store gravity force */
  let mut gF_N: Array1<f64> = Array1::zeros(3);

  gF_N[2] = -9.80665 * state_in[STATE_VEC_INDX_MASS]; 

  gF_N
}