use ndarray::Array1;

/*
 * @brief: Function to convert position coordinates from PCI 
 *         (planet centered inertial) to latitude, longitude, Radius
 * 
 * @param[in] pos_PCI_m_in - Position vector in PCI frame
 * 
 * @param[in] state_epoch_in - Time epoch since J2000 in seconds to position 
 *                             vector
 * 
 *  @returns Position vector in PCPF frame in Latitude, Longitude, Radius format
 * 
 */
fn convert_pci_to_llr(pos_PCI_m_in: &Array1<f64>, state_epoch_in: f64)
-> Array1<f64>
{
  let mut vec_out_LLR: Array1<f64> = Array1::zeros(3);

  vec_out_LLR[2] = ((pos_PCI_m_in[0]).powf(2.0) 
                  + (pos_PCI_m_in[1]).powf(2.0) 
                  + (pos_PCI_m_in[2]).powf(2.0)).sqrt();

  if vec_out_LLR[2] != 0.0
  {
    vec_out_LLR[0] = (pos_PCI_m_in[2] / vec_out_LLR[2]).asin();

    vec_out_LLR[1] = (pos_PCI_m_in[1] / pos_PCI_m_in[0]).atan();
  }
  else 
  {
    println!{"[WRN] Conversion from Cartesian to cylindrical coordinates failed. Radius found to be zero!"};    
  }
  vec_out_LLR
}

/*
 * @brief: Function to convert position coordinates from PCI 
 *         (planet centered inertial) to latitude, longitude, Radius
 * 
 * @param[in] pos_PCI_m_in - Position vector in PCI frame
 * 
 * @param[in] state_epoch_in - Time epoch since J2000 in seconds to position 
 *                             vector
 * 
 *  @returns Position vector in PCPF frame in cartesian coordinates
 * 
 */
fn convert_pci_to_pcpf(pos_PCI_m_in: &Array1<f64>, state_epoch_in: f64)
-> Array1<f64>
{
  let mut vec_out_PCPF_m: Array1<f64> = Array1::zeros(3);

  // TODO: implement method

  vec_out_PCPF_m
}