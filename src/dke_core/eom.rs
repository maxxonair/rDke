/* Include external crates */
use ndarray::Array1;

/* Include local carates */
use crate::dke_core::dke_core::DKE;
use crate::environment::gravity::*;

/* Include constants */
use crate::constants::state::*;
use crate::constants::general::*;

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

  let fx: f64 = sum_of_forces_iframe[VEC_X];
  let fy: f64 = sum_of_forces_iframe[VEC_Y];
  let fz: f64 = sum_of_forces_iframe[VEC_Z];

  let mass_kg: f64 = x_in[STATE_VEC_INDX_MASS];

  /* Get velocity from the previous steps state */
  let vx: f64 = x_in[STATE_VEC_INDX_VEL_X];
  let vy: f64 = x_in[STATE_VEC_INDX_VEL_Y];
  let vz: f64 = x_in[STATE_VEC_INDX_VEL_Z];

  /* Compute acceleration from sum of forces and S/C mass (f = m * a) */
  let ax: f64 = fx / mass_kg;
  let ay: f64 = fy / mass_kg;
  let az: f64 = fz / mass_kg;

  /* [POSITION DERIVATIVE] */
  /* dx/dt = f(x_in, t) */
  dxdt_out[STATE_VEC_INDX_POS_X] = vx;
  dxdt_out[STATE_VEC_INDX_POS_Y] = vy;
  dxdt_out[STATE_VEC_INDX_POS_Z] = vz;

  /* [VELOCITY DERIVATIVE] */
  /* dv/dt = f(x_in,t) */
  dxdt_out[STATE_VEC_INDX_VEL_X] = ax;
  dxdt_out[STATE_VEC_INDX_VEL_Y] = ay;
  dxdt_out[STATE_VEC_INDX_VEL_Z] = az;

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
