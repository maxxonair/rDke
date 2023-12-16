
/* Include external crates */
use ndarray::{Array1, ArrayView1, s};
use chrono::*;

/* Import (local) structs */
/* None */

/* Include local crates */
use crate::dke_core::dke_core::DKE;
use crate::environment::gravity::gravity::get_grav_acc;
use crate::math::frame_math::{convert_eci_to_ecef,
                              convert_ecef_to_llr};
use crate::math::time_math::calc_earth_gast_deg;

/* Import constants */
use crate::constants::state::*;
use crate::constants::time::*;

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

  /* Assign simulation time to current state */
  x_out[STATE_VEC_INDX_SIM_TIME] = dke.get_sim_time_s();
  /* Update state epoch */
  x_out[STATE_VEC_INDX_J2000_S] += dke.get_dt_s();

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

  /* Get position in PCI frame from state vector */
  let mut pos_eci_m: Array1<f64> = Array1::zeros(3);
  pos_eci_m.assign(&x_out.slice(s![STATE_VEC_INDX_POS_X..(STATE_VEC_INDX_POS_Z+1)]));

  /* Get position in PCPF frame from eci position and current time */
  let unix_time_ms: i64 = ((x1_inout[STATE_VEC_INDX_J2000_S] 
    + (UNIX_SECONDS_AT_J2000_EPOCH as f64)) * 1000.0) as i64;

  /* Create DateTime from unix timestamp */
  let native_datetime: NaiveDateTime = NaiveDateTime::from_timestamp_millis(unix_time_ms).unwrap();
  let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(native_datetime, Utc);
  let gast_deg: f64 = calc_earth_gast_deg(datetime);
  
  /* Update Greenwich aparent sidreal time in degree */
  x_out[STATE_VEC_INDX_GAST_DEG] = gast_deg;

  let pos_ecef_m: Array1<f64> = convert_eci_to_ecef(&pos_eci_m, gast_deg);
  let pos_ecef_llr: Array1<f64> = convert_ecef_to_llr(pos_ecef_m.view());

  /* Update Latitude / Longitude in ECEF  */
  x_out[STATE_VEC_INDX_POS_PCPF_LAT_DEG] = (pos_ecef_llr[0]).to_degrees();
  x_out[STATE_VEC_INDX_POS_PCPF_LONG_DEG] = (pos_ecef_llr[1]).to_degrees();

  x_out[STATE_VEC_INDX_ALTITUDE_PCPF_M] = pos_ecef_llr[2]
    - (dke.get_planet().get_semi_major_axis()
       + dke.get_planet().get_semi_minor_axis()) * 0.5;

  /* Get local magnitude of the gravitational acceleration */
  x_out[STATE_VEC_INDX_GRAV_ACC_MSS] = get_grav_acc(&x1_inout, &dke);

  let mut vel_eci_ms: Array1<f64> = Array1::zeros(3);
  vel_eci_ms.assign(&x_out.slice(s![STATE_VEC_INDX_VEL_X..(STATE_VEC_INDX_VEL_Z+1)]));
  x_out[STATE_VEC_INDX_VEL_MAGN_PCI_MS] = vel_eci_ms.dot(&vel_eci_ms).sqrt();
  
  x_out
}