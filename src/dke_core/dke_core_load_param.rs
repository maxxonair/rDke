
/* Include external crates */
use ini::Ini;

/* Import (local) structs */
/* None */

/* Include local crates */
use crate::dke_core::dke_core::DKE;

/* Import constants */
use crate::constants::filepaths::*;

/*
 * @brief: This function is to load all required parameters from configuration 
 *         files before initialising and starting the simulation. This includes 
 *         all parameters for sub-structs for all simulation modules.
 * 
 * @details: TODO
 * 
 */
pub fn load_dke_core_parameters(dke: &mut DKE)
{
  /* Create parameter file instance > sim.ini < */
  let sim_conf = Ini::load_from_file(SIM_PARAMETER_FILE_PATH)
    .expect(&"! [ERROR] ! > sim.ini not found! <".to_string());
  /* -------------------------------------------------------------------------
  * WRITE/PRINT Intervals
  * 
  * -----------------------------------------------------------------------*/
  dke.set_param_sim_print_interval_s(&((sim_conf
      .section(Some("print_setting")).unwrap()
      .get("sim_print_interval_s").unwrap())
      .parse::<f64>().unwrap()));

  dke.set_param_sim_archive_interval_s(&((sim_conf
    .section(Some("write_setting")).unwrap()
    .get("sim_archive_interval_s").unwrap())
    .parse::<f64>().unwrap()));

  dke.set_param_sim_archive_flush_interval_s(&((sim_conf
    .section(Some("write_setting")).unwrap()
    .get("sim_archive_flush_interval_s").unwrap())
    .parse::<f64>().unwrap()));
  /* -------------------------------------------------------------------------
  *      [PLANET]
  * 
  * -----------------------------------------------------------------------*/
  let planet_conf = Ini::load_from_file(PLANET_PARAMETER_FILE_PATH)
    .expect(&"! [ERROR] ! > planet.ini not found! <".to_string());

  dke.get_mut_environment().get_mut_planet().set_semi_major_axis(&(planet_conf
    .section(Some("general")).unwrap()
    .get("planet_semi_major_axis_m").unwrap())
    .parse::<f64>().unwrap() );

  dke.get_mut_environment().get_mut_planet().set_semi_minor_axis(&(planet_conf
    .section(Some("general")).unwrap()
    .get("planet_semi_minor_axis_m").unwrap())
    .parse::<f64>().unwrap() );

  dke.get_mut_environment().get_mut_planet().set_gravitational_constant(&(planet_conf
    .section(Some("general")).unwrap()
    .get("planet_gravitational_constant").unwrap())
    .parse::<f64>().unwrap() );

  dke.get_mut_environment().get_mut_planet().set_flattening_factor(&(planet_conf
    .section(Some("general")).unwrap()
    .get("planet_flattening_factor").unwrap())
    .parse::<f64>().unwrap() );

  dke.get_mut_environment().get_mut_planet().set_omega(&(planet_conf
    .section(Some("general")).unwrap()
    .get("planet_omega_rads").unwrap())
    .parse::<f64>().unwrap() );

 /* -------------------------------------------------------------------------
  *      [PLANET / ATMOSPHERE]
  * 
  * -----------------------------------------------------------------------*/
  let atmosphere_conf: Ini = Ini::load_from_file(ATMOSPHERE_PARAMETER_FILE_PATH)
    .expect(&"! [ERROR] ! > atmosphere.ini not found! <".to_string());

  dke.get_mut_environment().get_mut_planet().get_mut_atmosphere().set_enable_atmophere_modelling(&(atmosphere_conf
      .section(Some("general")).unwrap()
      .get("flag_enable_atmosphere_modelling").unwrap())
      .parse::<bool>().unwrap() );

  dke.get_mut_environment().get_mut_planet().get_mut_atmosphere().set_radio_10_cm_flux(&(atmosphere_conf
      .section(Some("general")).unwrap()
      .get("radio_10_cm_flux").unwrap())
      .parse::<f64>().unwrap() );

  dke.get_mut_environment().get_mut_planet().get_mut_atmosphere().set_geomagnetic_ap_index(&(atmosphere_conf
        .section(Some("general")).unwrap()
        .get("geomagnetic_ap_index").unwrap())
        .parse::<f64>().unwrap() );

  /*
   * @brief: After all parameters have been loaded -> initialize planet and sub-structs
   * 
   */
  dke.get_mut_environment().get_mut_planet().init();
  /* -------------------------------------------------------------------------
  *      [SPACECRAFT]
  * 
  * -----------------------------------------------------------------------*/
  let sim_conf: Ini = Ini::load_from_file(SIM_PARAMETER_FILE_PATH)
  .expect(&"! [ERROR] ! > sim.ini not found! <".to_string());

  dke.get_mut_environment().get_mut_spacecraft().set_sc_aero_eff_area_mm(&(sim_conf
    .section(Some("start_state")).unwrap()
    .get("sc_eff_aero_area_mm").unwrap())
    .parse::<f64>().unwrap() );

  dke.get_mut_environment().get_mut_spacecraft().set_sc_mass_kg(&(sim_conf
    .section(Some("start_state")).unwrap()
    .get("sc_mass_start_kg").unwrap())
    .parse::<f64>().unwrap() );

  dke.get_mut_environment().get_mut_spacecraft().set_sc_charact_length_m(&(sim_conf
      .section(Some("start_state")).unwrap()
      .get("sc_charact_length_m").unwrap())
      .parse::<f64>().unwrap() );

}
