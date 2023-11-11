use ndarray::{Array1, Array2};
use chrono::{DateTime, Utc};


use crate::math::rotation_math::dcm_from_zrot;

/*
 * @brief: Function to convert position coordinates from ECEF 
 *         (planet centered inertial) to latitude, longitude, Radius
 * 
 * @param[in] pos_ecef_m_in - Position vector in ECEF frame
 * 
 *  @returns Position vector in PCPF frame in Latitude, Longitude, Radius format
 * 
 */
pub fn convert_ecef_to_llr(pos_PCI_m_in: &Array1<f64>)
-> Array1<f64>
{
  let mut vec_out_LLR: Array1<f64> = Array1::zeros(3);

  /* Radius [m] */
  vec_out_LLR[2] = ((pos_PCI_m_in[0]).powf(2.0) 
                  + (pos_PCI_m_in[1]).powf(2.0) 
                  + (pos_PCI_m_in[2]).powf(2.0)).sqrt();

  if vec_out_LLR[2] != 0.0
  {
    /* Latitude [rad] */
    vec_out_LLR[0] = (pos_PCI_m_in[2] / vec_out_LLR[2]).asin();
    /* Longitude [rad] */
    vec_out_LLR[1] =  (pos_PCI_m_in[1]).atan2(pos_PCI_m_in[0]) ;
  }
  else 
  {
    println!{"[WRN] Conversion from Cartesian to cylindrical coordinates failed. Radius found to be zero!"};    
  }
  vec_out_LLR
}

pub fn convert_eci_to_ecef(pos_eci_in: &Array1<f64>, gast_deg: f64)
-> Array1<f64>
{

  // TODO: Remove this ugliness
  let mut pos_vec_in: Array2<f64> = Array2::zeros((3,1));
  pos_vec_in[[0,0]] = pos_eci_in[0];
  pos_vec_in[[1,0]] = pos_eci_in[1];
  pos_vec_in[[2,0]] = pos_eci_in[2];

  /* Create direction cosine matrix for the z rotation from ECI to ECEF */
  let dcm_eci_2_ecef: Array2<f64> = dcm_from_zrot(gast_deg);

  /* Create vector rotation from ECI to ECEF */
  let vec_out_ecef_2: Array2<f64> = dcm_eci_2_ecef.dot(&pos_vec_in);
  
  /* Convert Array2 back to Array1 */
  let mut vec_out_ecef: Array1<f64> = Array1::zeros(3);
  vec_out_ecef[0] = vec_out_ecef_2[[0,0]];
  vec_out_ecef[1] = vec_out_ecef_2[[1,0]];
  vec_out_ecef[2] = vec_out_ecef_2[[2,0]];
  vec_out_ecef
}