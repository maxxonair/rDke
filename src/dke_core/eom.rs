/* Include external crates */
use ndarray::Array1;

/* Include local carates */
use crate::environment::environment::Environment;
use crate::environment::gravity::*;
use crate::environment::aerodynamic::*;

/* Include constants */
use crate::constants::state::*;
use crate::constants::general::*;

pub fn dxdt(
    x_in: &Array1<f64>,
    environment: &mut Environment) 
-> Array1<f64>
{
  let mut dxdt_out = Array1
                                                    ::<f64>
                                                    ::zeros(STATE_VEC_NUM_ELEMENTS);
  /* Get sum of all forces acting on the S/C */
  let sum_of_forces_pci: Array1<f64> = get_sum_of_force_vecs_pci(x_in, environment) ;

  let fx: f64 = sum_of_forces_pci[VEC_X];
  let fy: f64 = sum_of_forces_pci[VEC_Y];
  let fz: f64 = sum_of_forces_pci[VEC_Z];

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
 * 
 * NOTE: All all forces acting on the spacecraft here
 * 
 * @unit: TODO
 * @frame: TODO
 */
pub fn get_sum_of_force_vecs_pci(x_n1: &Array1<f64>, environment: &mut Environment) -> Array1<f64>
{
  let mut sum_of_forces_vec_pci_n: Array1<f64> = Array1::zeros(3);

  /* [GRAVITATIONAL FORCES] */
  sum_of_forces_vec_pci_n += &gravity::get_force_vec_pci(x_n1, environment);
  
  /* [AERODYNAMIC FORCES] */
  if *environment.get_mut_planet().get_mut_atmosphere().is_atmoshpere_modelled() == true
  {
    sum_of_forces_vec_pci_n += &aerodynamic::get_force_vec_pci(x_n1.view(), environment);
  }

  sum_of_forces_vec_pci_n
}
