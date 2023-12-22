



#[derive(Clone)]

pub struct Planet {
  /* [semi-major axis] 
   * @description : Planets semi-major axis (for elliptical model)
   * @unit        : m
   * 
   * */
  semi_major_axis_m: f64,
  semi_minor_axis_m: f64,
  gravitational_constant: f64,
  flattening_factor: f64,
  omega_rads: f64,
  radio_10_cm_flux: f64,
  geomagnetic_ap_index: f64,
  enable_atmosphere_modelling: bool
}

/*
 * ----------------------------------------------------------------------
 *                    [constructor]
 * ----------------------------------------------------------------------
 */
impl Planet {
  pub fn new() -> Planet {
    Planet {
      semi_major_axis_m: 0.0,
      semi_minor_axis_m: 0.0,
      gravitational_constant: 0.0,
      flattening_factor: 0.0,
      omega_rads: 0.0,
      radio_10_cm_flux: 0.0,
      geomagnetic_ap_index: 0.0,
      enable_atmosphere_modelling: false
    }
  }
}
/*
 * ----------------------------------------------------------------------
 *                    [setters]
 * ----------------------------------------------------------------------
 */
impl Planet {
  pub fn set_semi_major_axis(&mut self, val_in: &f64) {self.semi_major_axis_m = *val_in;}
  pub fn set_semi_minor_axis(&mut self, val_in: &f64) {self.semi_minor_axis_m = *val_in;}
  pub fn set_gravitational_constant(&mut self, val_in: &f64) {self.gravitational_constant = *val_in;}
  pub fn set_flattening_factor(&mut self, val_in: &f64) {self.flattening_factor = *val_in;}
  pub fn set_omega(&mut self, val_in: &f64) {self.omega_rads = *val_in;}
  pub fn set_radio_10_cm_flux(&mut self, val_in: &f64) {self.radio_10_cm_flux = *val_in;}
  pub fn set_geomagnetic_ap_index(&mut self, val_in: &f64) {self.geomagnetic_ap_index = *val_in;}
  pub fn set_enable_atmophere_modelling(&mut self, val_in: &bool) {self.enable_atmosphere_modelling = *val_in}
}
/*
 * ----------------------------------------------------------------------
 *                    [getters]
 * ----------------------------------------------------------------------
 * Note: All getters here allow immutable access only by design!
 */
impl Planet {
  pub fn get_semi_major_axis(&self) -> &f64 {&self.semi_major_axis_m}
  pub fn get_semi_minor_axis(&self) -> &f64 {&self.semi_minor_axis_m}
  pub fn get_gravitational_constant(&self) -> &f64 {&self.gravitational_constant}
  pub fn get_flattening_factor(&self) -> &f64 {&self.flattening_factor}
  pub fn get_omega(&self) -> &f64 {&self.omega_rads}
  pub fn get_radio_10_cm_flux(&self) -> &f64 {&self.radio_10_cm_flux}
  pub fn get_geomagnetic_ap_index(&self) -> &f64 {&self.geomagnetic_ap_index}

  pub fn is_atmoshpere_modelled(&self) -> &bool {&self.enable_atmosphere_modelling}
}