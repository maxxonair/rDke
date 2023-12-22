use ndarray::{Array1, ArrayView1, s};

/* Import (local) structs */
use crate::environment::planet::*;

/* Include local crates */
use crate::environment::environment::Environment;
use crate::math::vec_math::pointwise_square;

/* Import constants */
use crate::constants::state::*;
use crate::constants::general::*;

/*
 * @brief: Function to compute the force vector of all aerodynamic forces acting on 
 *         the spacecraft.
 * 
 * 
 * @unit: Newton
 * @frame: PCI
 * 
 */
pub fn get_force_vec_iframe(state_in: ArrayView1<f64>, environment: &mut Environment)
-> Array1<f64>
{
  let mut sum_of_forces_vec_pci_n: Array1<f64> = Array1::zeros(3);
  // TODO: fill with content to correctly determine aerodynamic forces in PCI frame
  sum_of_forces_vec_pci_n = get_newtonian_flow_force_vec(state_in, environment);

  environment.get_mut_spacecraft().set_aero_force_pci_n_x(&sum_of_forces_vec_pci_n[VEC_X]);
  environment.get_mut_spacecraft().set_aero_force_pci_n_x(&sum_of_forces_vec_pci_n[VEC_Y]);
  environment.get_mut_spacecraft().set_aero_force_pci_n_x(&sum_of_forces_vec_pci_n[VEC_Z]);

  sum_of_forces_vec_pci_n
}


/*
 * @brief: Function to compute the force vector of aerodynamic forces from newtonian
 *         flow aerodynamic on the spacecraft. 
 * 
 * Note: It requires an additional step in another function to determine if free molecular
 *       flow can be assumed or if continuous flow has to be modelled instead. 
 * 
 * @unit: Newton
 */
fn get_newtonian_flow_force_vec(state_in: ArrayView1<f64>, environment: &mut Environment)
-> Array1<f64>
{
  /* initialize force vector in PCI frame */
  let mut sum_of_forces_vec_pci_n: Array1<f64> = Array1::zeros(3);

  /* Get local atmospheric density from atmosphere model */
  let density: f64 = atmosphere_model::calculate_density(state_in[STATE_VEC_INDX_ALTITUDE_PCPF_M], environment);
  
  /* Get velocity vector in PCI frame from state vector */
  let mut V_infinity: Array1<f64> = Array1::zeros(3);
  V_infinity.assign(&state_in.slice(s![STATE_VEC_INDX_VEL_X..(STATE_VEC_INDX_VEL_Z+1)]));

  // TODO: parameterize this
  let a_effective_mm: f64 = 1.5;

  sum_of_forces_vec_pci_n = -1.0 * (density * a_effective_mm) * pointwise_square(V_infinity) ;

  sum_of_forces_vec_pci_n
}