
/* Include external crates */
use ini::Ini;
use ndarray::{Array1, ArrayView1, s};

/* Import (local) structs */
use crate::environment::planet::planet::Planet;
use crate::environment::spacecraft::spacecraft::Spacecraft;

/* Include local crates */
use crate::dke_core::dke_core::DKE;

/* Import constants */
use crate::constants::filepaths::*;


#[derive(Clone)]

pub struct Environment {
  sim_current_time_s: f64,
  dt_s: f64,
  /* [Planet struct] 
   * @description : Data struct containing all planet relevant parameters
   * @unit        : N/A
   * 
   * */
   planet: Planet,
  /* [Spacecraft struct] 
   * @description : Data struct containing all spacecraft relevant parameters
   * @unit        : N/A
   * 
   * */
   spacecraft: Spacecraft
}



/*
 * ----------------------------------------------------------------------
 *                    [constructor]
 * ----------------------------------------------------------------------
 */
impl Environment {
  pub fn new() -> Environment {
    Environment {
      sim_current_time_s: 0.0,
      dt_s: 0.0,
      planet: Planet::new(),
      spacecraft: Spacecraft::new(),

    }
  }
}
/*
 * ----------------------------------------------------------------------
 *                    [setters]
 * ----------------------------------------------------------------------
 */
impl Environment {
  pub fn set_planet(&mut self, planet_in: Planet) {self.planet = planet_in.clone();}

  pub fn set_simtimes(&mut self, dt_s_in: &f64, sim_current_time_s_in: &f64) 
  {
    self.dt_s = *dt_s_in;
    self.sim_current_time_s = *sim_current_time_s_in;
  }
}

/*
 * ----------------------------------------------------------------------
 *                    [getters]
 * ----------------------------------------------------------------------
 * Note: All getters here allow immutable access only by design!
 */
impl Environment {

  pub fn get_planet(&self) -> &Planet {&self.planet}
  
  /* The following function allows mutable access to the planet struct.
   * This power should be used with care and ONLY to set planet data when 
   * initializing from parameters 
   */
  pub fn get_mut_planet(&mut self) -> &mut Planet {&mut self.planet}
  
  /* 
   * Allow immutable access only.
   */
  pub fn get_spacecraft(&self) -> &Spacecraft {&self.spacecraft}
  pub fn get_mut_spacecraft(&mut self) -> &mut Spacecraft {&mut self.spacecraft}

  pub fn get_dt_s(&self) -> f64 {self.dt_s}
  pub fn get_sim_time_s(&self) -> f64 {self.sim_current_time_s}

}