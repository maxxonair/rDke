use ndarray::{Array1, ArrayView1, s};

/* Import (local) structs */
use crate::environment::planet::*;

/* Include local crates */
use crate::environment::environment::Environment;
use crate::math::vec_math::{l2_norm_array1, normalize_array1};

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
  // TODO: fill with content to correctly determine aerodynamic forces in PCI frame
  let sum_of_forces_vec_pci_n: Array1<f64> = get_newtonian_flow_force_vec(state_in, environment);

  environment.get_mut_spacecraft().set_aero_force_pci_n_x(&sum_of_forces_vec_pci_n[VEC_X]);
  environment.get_mut_spacecraft().set_aero_force_pci_n_y(&sum_of_forces_vec_pci_n[VEC_Y]);
  environment.get_mut_spacecraft().set_aero_force_pci_n_z(&sum_of_forces_vec_pci_n[VEC_Z]);

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

  /* Get local atmospheric density from atmosphere model */
  let density: f64 = atmosphere_model::calculate_density(state_in[STATE_VEC_INDX_ALTITUDE_PCPF_M], environment);
  
  /* Get velocity vector in PCI frame from state vector */
  let mut velocity_vec: Array1<f64> = Array1::zeros(3);
  velocity_vec.assign(&state_in.slice(s![STATE_VEC_INDX_VEL_X..(STATE_VEC_INDX_VEL_Z+1)]));
 
  /* Compute Vinfinity as the length of the velocity vector in PCI frame */
  let v_infinity: f64 = l2_norm_array1(velocity_vec.view());
  /* Compute squared velocity */
  let v_squared: f64 = v_infinity * v_infinity;

  /* Compute drag direction vector in PCI frame as opposite to the current normalized 
   * velocity vector */
  let drag_direction_vec: Array1<f64> = -1.0 * normalize_array1(velocity_vec);

  let sum_of_forces_vec_pci_n: Array1<f64> = density * environment.get_spacecraft().get_sc_aero_eff_area_mm()
                                            * v_squared * drag_direction_vec;

  sum_of_forces_vec_pci_n
}