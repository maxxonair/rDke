
/* Include external crates */
/* None */

/* Import (local) structs */
/* None */

/* Include local crates */
/* None */

/* Import constants */
/* None */


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
  * @brief: Atmospheric density at the position of the spacecraft.
  *
  * Note: This will be maintained here rather than on the planet/environment
  *       side for know, because the density value is closely tied to the 
  *       spacecrafts current position.
  * 
  * @unit: kg/mmm
  * @frame: N/A
  */
  atmos_density_kgmmm: f64,
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
  sc_aero_eff_area_mm: f64
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
      atmos_density_kgmmm: 0.0,
      sc_mass_kg: 0.0,
      sc_altitude_m: 0.0,
      sc_aero_eff_area_mm: 0.0

    }
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
  pub fn set_atmos_density_kgmmm(&mut self, val_in: &f64) {self.atmos_density_kgmmm = *val_in;}
  pub fn set_sc_mass_kg(&mut self, val_in: &f64) {self.sc_mass_kg = *val_in;}
  pub fn set_sc_altitude_m(&mut self, val_in: &f64) {self.sc_altitude_m = *val_in;}
  pub fn set_sc_aero_eff_area_mm(&mut self, val_in: &f64) {self.sc_aero_eff_area_mm = *val_in;}
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
  pub fn get_atmos_density_kgmmm(&self) -> &f64 {&self.atmos_density_kgmmm}
  pub fn get_sc_mass_kg(&self) -> &f64 {&self.sc_mass_kg}
  pub fn get_sc_altitude_m(&self) -> &f64 {&self.sc_altitude_m}
  pub fn get_sc_aero_eff_area_mm(&self) -> &f64 {&self.sc_aero_eff_area_mm}
}