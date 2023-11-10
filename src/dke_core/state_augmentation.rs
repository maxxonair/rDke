
/* Include external crates */
use ini::Ini;
use ndarray::Array1;

/* Import (local) structs */
/* None */

/* Include local crates */
use crate::dke_core::dke_core::DKE;
use crate::environment::gravity::gravity::get_grav_acc;

/* Import constants */
use crate::constants::state::*;

/*
 * @brief: This function is to fill fields of the full state vector that are not 
 *         filled by the solver itself.
 * 
 * @details: Note: State augmentation is the process of filling in missing fields
 *                 (field not solved by the solving step) of the full state 
 *                 vector. For computational efficiency state augmentation is 
 *                 done in two steps. 
 *                 (1) augment_state_solve() is called at solving frequency and 
 *                     only contains post-processing computations that need to 
 *                     be done at full frequency (e.g. acceleration from velocity
 *                     derivative)
 *                 (2) augment_state_write() is called at writing frequency and 
 *                     is only called when writing the results to file. This 
 *                     function contains all processing that can be done at a
 *                     lower frequency.
 * 
 *           Post solving computations include: 
 *           * Acceleration
 * 
 * @param[in] x1_in - Full state vector for current solving step n
 * 
 * @param[in] x0_in - Full state vector for previous solving step n-1
 *  
 * @returns x1_in - Full state vector for current solving step with additional 
 *                  fields filled.
 * 
 */
pub fn augment_state_solve(dke: &DKE, x1_inout: &Array1<f64>, x0_in: &Array1<f64>)
-> Array1<f64>
{
  let mut x_out = (*x1_inout).clone();
  /* Compute linear acceleration from incremental velocity change */
  x_out[STATE_VEC_INDX_ACC_X] = (x_out[STATE_VEC_INDX_VEL_X] 
    - x0_in[STATE_VEC_INDX_VEL_X]) / dke.get_dt_s();
  x_out[STATE_VEC_INDX_ACC_Y] = (x_out[STATE_VEC_INDX_VEL_Y] 
    - x0_in[STATE_VEC_INDX_VEL_Y]) / dke.get_dt_s();
  x_out[STATE_VEC_INDX_ACC_Z] = (x_out[STATE_VEC_INDX_VEL_Z] 
    - x0_in[STATE_VEC_INDX_VEL_Z]) / dke.get_dt_s();
  x_out
}


/*
 * @brief: This function is to fill fields of the full state vector that are not 
 *         filled by the solver itself.
 * 
 * @details: Note: State augmentation is the process of filling in missing fields
 *                 (field not solved by the solving step) of the full state 
 *                 vector. For computational efficiency state augmentation is 
 *                 done in two steps. 
 *                 (1) augment_state_solve() is called at solving frequency and 
 *                     only contains post-processing computations that need to 
 *                     be done at full frequency (e.g. acceleration from velocity
 *                     derivative)
 *                 (2) augment_state_write() is called at writing frequency and 
 *                     is only called when writing the results to file. This 
 *                     function contains all processing that can be done at a
 *                     lower frequency.
 * 
 *           Post solving computations include: 
 *           * TODO
 * 
 * @param[in] x1_in - Full state vector for current solving step n
 * 
 * @param[in] x0_in - Full state vector for previous solving step n-1
 *  
 * @returns x1_in - Full state vector for current solving step with additional 
 *                  fields filled.
 * 
 */
pub fn augment_state_write(dke: &DKE, x1_inout: &Array1<f64>, x0_in: &Array1<f64>)
-> Array1<f64>
{
  /* TODO */
  let mut x_out = (*x1_inout).clone();

  // TODO: This is not to proper way to compute the local altitude by any 
  //       standards and needs to be replaced by calculating the geodetic 
  //       altitude.
  let local_radius_m: f64 = ( x_out[STATE_VEC_INDX_POS_X].powf(2.0)
                            + x_out[STATE_VEC_INDX_POS_Y].powf(2.0) 
                            + x_out[STATE_VEC_INDX_POS_Z].powf(2.0)).sqrt() ;
  x_out[STATE_VEC_INDX_ALTITUDE_PCPF_M] = local_radius_m
    - (dke.get_planet().get_semi_major_axis()
       + dke.get_planet().get_semi_minor_axis()) * 0.5;

  /* Get local magnitude of the gravitational acceleration */
  x_out[STATE_VEC_INDX_GRAV_ACC_MSS] = get_grav_acc(&x1_inout, &dke);

  /* The following is only true if we assume that the Earth is not rotating
    * around it's axis.
    * TODO: Replace with proper implementation */
  if local_radius_m != 0.0
  {
    x_out[STATE_VEC_INDX_POS_PCPF_LAT_DEG] = (x_out[STATE_VEC_INDX_POS_Z] / local_radius_m).asin();

    x_out[STATE_VEC_INDX_POS_PCPF_LONG_DEG] = (x_out[STATE_VEC_INDX_POS_Y] / x_out[STATE_VEC_INDX_POS_X]).atan();
  }
  else 
  {
    println!{"[WRN] Conversion from Cartesian to cylindrical coordinates failed. Radius found to be zero!"};    
  }

  x_out
}