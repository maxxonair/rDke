
/* Include external crates */
/* None */

/* Import (local) structs */
/* None */

/* Include local crates */
use crate::io::read_csv::*;
use crate::math::lin_math::*;

/* Import constants */
use crate::constants::spacecraft::*;


#[derive(Clone)]

pub struct Spacecraft {
 /*
  * @brief: Acceleration in x direction of the spacecraft in inertial (PCI) frame
  * 
  * @unit: m/ss
  * @frame: PCI
  */
  acc_pci_mss_x: f64,
  acc_pci_mss_y: f64,
  acc_pci_mss_z: f64,
 /*
  * @brief: Gravitational force acting on the spacecraft in x direction in inertial 
  *         (PCI) frame
  * 
  * @unit: Newton
  * @frame: PCI
  */
  grav_force_pci_n_x: f64,
  grav_force_pci_n_y: f64,
  grav_force_pci_n_z: f64,
 /*
  * @brief: Aerodynamic force acting on the spacecraft in x direction in inertial 
  *         (PCI) frame
  * 
  * @unit: Newton
  * @frame: PCI
  */
  aero_force_pci_n_x: f64,
  aero_force_pci_n_y: f64,
  aero_force_pci_n_z: f64,
 /*
  * @brief: Total mass of the spacecraft.
  * 
  * @unit: kg
  * @frame: N/A
  */
  sc_mass_kg: f64,
 /*
  * @brief: Spacecraft altitude above sea level (wrt the central body)
  * 
  * @unit: m
  * @frame: PCI
  */
  sc_altitude_m: f64,
 /*
  * @brief: Effective aerodynamic surface area
  * 
  * @unit: m * m 
  * @frame: N/A
  */
  sc_aero_eff_area_mm: f64,
 /*
  * @brief: Characteristic length of the spacecraft. 
  *
  * Note: This length parameter is used to compute a series of aerodynamic
  *       number like Reynolds, Knudsen and others
  * 
  * @unit: m 
  * @frame: N/A
  */
  sc_charact_length_m: f64,
 /*
  * @brief: Drag coefficient of the spacecraft.
  * 
  * Note: While in continuous flow this value will be computed from a Mach number
  *       dependent LUT. 
  * 
  * @unit: N/A
  * @frame: N/A
  */
  sc_drag_contin_coefficient: f64,
 /*
  * @brief: Mach number of the spacecraft.
  * 
  * @unit: N/A
  * @frame: N/A
  */
  sc_mach_number: f64,
  /*
   * @brief: Tuple vector to store LUT for Mach - Cd 
   */
  drag_coeff_lut_vec: Vec<(f64, f64)>
}



/*
 * ----------------------------------------------------------------------
 *                    [constructor]
 * ----------------------------------------------------------------------
 */
impl Spacecraft {
  pub fn new() -> Spacecraft {
    Spacecraft {
      acc_pci_mss_x: 0.0,
      acc_pci_mss_y: 0.0,
      acc_pci_mss_z: 0.0,
      grav_force_pci_n_x: 0.0,
      grav_force_pci_n_y: 0.0,
      grav_force_pci_n_z: 0.0,
      aero_force_pci_n_x: 0.0,
      aero_force_pci_n_y: 0.0,
      aero_force_pci_n_z: 0.0,
      sc_mass_kg: 0.0,
      sc_altitude_m: 0.0,
      sc_aero_eff_area_mm: 0.0,
      sc_charact_length_m: 0.0,
      sc_drag_contin_coefficient: 0.0,
      sc_mach_number: 0.0,
      drag_coeff_lut_vec: Vec::new()

    }
  }

  /* 
   * @brief: Function to complete initializing the class after the struct has 
   *         been created. This function usually contains file loaders.
   */
  pub fn init(&mut self) 
  {
    self.load_drag_coeff_lut();
  }
}
/*
 * ----------------------------------------------------------------------
 *                    [setters]
 * ----------------------------------------------------------------------
 */
impl Spacecraft {
  pub fn set_acc_pci_mss_x(&mut self, val_in: &f64) {self.acc_pci_mss_x = *val_in;}
  pub fn set_acc_pci_mss_y(&mut self, val_in: &f64) {self.acc_pci_mss_y = *val_in;}
  pub fn set_acc_pci_mss_z(&mut self, val_in: &f64) {self.acc_pci_mss_z = *val_in;}
  pub fn set_grav_force_pci_n_x(&mut self, val_in: &f64) {self.grav_force_pci_n_x = *val_in;}
  pub fn set_grav_force_pci_n_y(&mut self, val_in: &f64) {self.grav_force_pci_n_y = *val_in;}
  pub fn set_grav_force_pci_n_z(&mut self, val_in: &f64) {self.grav_force_pci_n_z = *val_in;}
  pub fn set_aero_force_pci_n_x(&mut self, val_in: &f64) {self.aero_force_pci_n_x = *val_in;}
  pub fn set_aero_force_pci_n_y(&mut self, val_in: &f64) {self.aero_force_pci_n_y = *val_in;}
  pub fn set_aero_force_pci_n_z(&mut self, val_in: &f64) {self.aero_force_pci_n_z = *val_in;}
  pub fn set_sc_mass_kg(&mut self, val_in: &f64) {self.sc_mass_kg = *val_in;}
  pub fn set_sc_altitude_m(&mut self, val_in: &f64) {self.sc_altitude_m = *val_in;}
  pub fn set_sc_aero_eff_area_mm(&mut self, val_in: &f64) {self.sc_aero_eff_area_mm = *val_in;}
  pub fn set_sc_charact_length_m(&mut self, val_in: &f64) {self.sc_charact_length_m = *val_in;}
  pub fn set_sc_drag_contin_coefficient(&mut self, val_in: &f64) {self.sc_drag_contin_coefficient = *val_in;}
  pub fn set_sc_mach_number(&mut self, val_in: &f64) {self.sc_mach_number = *val_in;}
}
/*
 * ----------------------------------------------------------------------
 *                    [getters]
 * ----------------------------------------------------------------------
 * Note: All getters here allow immutable access only by design!
 */
impl Spacecraft {
  pub fn get_acc_pci_mss_x(&self) -> &f64 {&self.acc_pci_mss_x}
  pub fn get_acc_pci_mss_y(&self) -> &f64 {&self.acc_pci_mss_y}
  pub fn get_acc_pci_mss_z(&self) -> &f64 {&self.acc_pci_mss_z}
  pub fn get_grav_force_pci_n_x(&self) -> &f64 {&self.grav_force_pci_n_x}
  pub fn get_grav_force_pci_n_y(&self) -> &f64 {&self.grav_force_pci_n_y}
  pub fn get_grav_force_pci_n_z(&self) -> &f64 {&self.grav_force_pci_n_z}
  pub fn get_aero_force_pci_n_x(&self) -> &f64 {&self.aero_force_pci_n_x}
  pub fn get_aero_force_pci_n_y(&self) -> &f64 {&self.aero_force_pci_n_y}
  pub fn get_aero_force_pci_n_z(&self) -> &f64 {&self.aero_force_pci_n_z}
  pub fn get_sc_mass_kg(&self) -> &f64 {&self.sc_mass_kg}
  pub fn get_sc_altitude_m(&self) -> &f64 {&self.sc_altitude_m}
  pub fn get_sc_aero_eff_area_mm(&self) -> &f64 {&self.sc_aero_eff_area_mm}
  pub fn get_sc_charact_length_m(&self) -> &f64 {&self.sc_charact_length_m}
  pub fn get_sc_drag_contin_coefficient(&self) -> &f64 {&self.sc_drag_contin_coefficient}
  pub fn get_sc_mach_number(&self) -> &f64 {&self.sc_mach_number}
}

/*
 * ----------------------------------------------------------------------
 *                    [public functions]
 * ----------------------------------------------------------------------
 * */
impl Spacecraft {
  /*
  * @brief: Read Mach dependent drag coefficient data (for continous flow) from file. 
  *         This function shall be called before running the simulation!
  * 
  *
  * @returns: 
  */
  pub fn load_drag_coeff_lut(&mut self) 
  {
    let vec_altitude_km: Vec<f64> = read_csv_column_f64(&SC_DRAG_COEFF_TABLE_PATH, true, SC_DRAG_COEFF_INDX_ALTITUDE);
    let vec_drag_coeff: Vec<f64> = read_csv_column_f64(&SC_DRAG_COEFF_TABLE_PATH, true, SC_DRAG_COEFF_INDX_DRAG_COEFF);

    for (i, x) in vec_drag_coeff.iter().enumerate() 
    {
      self.drag_coeff_lut_vec.push((vec_altitude_km[i], vec_drag_coeff[i]));
    }
  }

 /*
  * @brief: Function to update the atmospheric speed of sound for a given geometric 
  *         altitude
  *
  * @description: Speed of sound value is interpolated from a lookup table
  * 
  */
  pub fn update_mach_number(&mut self, sc_speed_pci_ms: f64, speed_of_sound_ms: f64)
  {
    if speed_of_sound_ms == 0.0
    {
      println!("[spacecraft.update_mach_number()] [WRN] Speed of sound is found to be zero. Set Mach = 0");
      self.sc_mach_number = 0.0;
    }
    /* Update speed of sound value for given altitude */
    self.sc_mach_number = sc_speed_pci_ms / speed_of_sound_ms;

  }

 /*
  * @brief: Function to update the drag coefficient for a given Mach number 
  *         
  * Note: This assumes that the Mach number has been updated in a previous step!
  * 
  */
  pub fn update_drag_coeff(&mut self)
  {
    /* Update drag coefficient for a given Mach number */
    self.sc_drag_contin_coefficient = find_value_from_lut(&self.drag_coeff_lut_vec, self.sc_mach_number);

  }

}