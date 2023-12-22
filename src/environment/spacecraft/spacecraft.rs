
/* Include external crates */
use ini::Ini;
use ndarray::{Array1, ArrayView1, s};

/* Import (local) structs */
/* None */

/* Include local crates */
use crate::dke_core::dke_core::DKE;

/* Import constants */
use crate::constants::filepaths::*;


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
  sc_altitude_m: f64
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
      sc_altitude_m: 0.0

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
}