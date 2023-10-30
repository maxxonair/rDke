/* Include external crates */
use ndarray::Array1;

/* Include local carates */
use crate::dke_core::dke_core::DKE;
use crate::environment::gravity::*;

/* Include constants */
use crate::constants::state::*;

pub fn dxdt(
    x_in: &Array1<f64>,
    t_in: f64,
    dke: &DKE) 
-> Array1<f64>
{
  let mut dxdt_out = Array1
                                                    ::<f64>
                                                    ::zeros(STATE_VEC_NUM_ELEMENTS);
  /* Get sum of all forces acting on the S/C */
  let sum_of_forces_iframe: Array1<f64> = get_sum_of_forces(x_in, dke) ;

  /* Dummy policy to test the integrator -> Only gravity is pulling us down */
  
  /* [POSITION] */

  /* dx/dt = f(x_in) */
  dxdt_out[STATE_VEC_INDX_POS_X] = x_in[STATE_VEC_INDX_VEL_X];
  dxdt_out[STATE_VEC_INDX_POS_Y] = x_in[STATE_VEC_INDX_VEL_Y];
  dxdt_out[STATE_VEC_INDX_POS_Z] = x_in[STATE_VEC_INDX_VEL_Z];

  /* dv/dt = f(x_in,t) */
  dxdt_out[STATE_VEC_INDX_VEL_X] = sum_of_forces_iframe[0] * ( 1.0 / x_in[STATE_VEC_INDX_MASS]) ;
  dxdt_out[STATE_VEC_INDX_VEL_Y] = sum_of_forces_iframe[1] * ( 1.0 / x_in[STATE_VEC_INDX_MASS]) ;
  dxdt_out[STATE_VEC_INDX_VEL_Z] = sum_of_forces_iframe[2] * ( 1.0 / x_in[STATE_VEC_INDX_MASS]) ;

  /* [ATTITUDE] */
  // TODO


  dxdt_out
}

/*
 * @brief: Function to gather the sum of all forces on the vehicle 
 */
pub fn get_sum_of_forces(x_n1: &Array1<f64>, dke: &DKE) -> Array1<f64>
{
  // TODO: All all forces acting on the spacecraft here
  gravity::get_force_in_iframe(x_n1, &dke)
}
