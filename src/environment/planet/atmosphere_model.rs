

/* Include external crates */
use libm::exp;

/* Include local crates */
use crate::environment::environment::Environment;

/* Include constants */
/* None */

/*
 * @brief: Main function to return the atmospheric density for a given altitude.
 * 
 *
 * @returns: Density at the given altitude in kg/m3
 */
pub fn calculate_density(altitude_m: f64, environment: &mut Environment)
-> f64
{
  /* Initialize density variable */
  let mut density: f64 = 0.0;

  if altitude_m < 180000.0 
  {
    density = calculate_density_earth_cira_model_180();
  }
  else if altitude_m < 500000.0
  {
    let res: (bool, f64) = calculate_density_earth_model_500(altitude_m, environment);
    if res.0 == true
    {
      density = res.1;
    }
  }
  else
  {
    // DO NOTHING FOR NOW
  }

  /* Update density value in spacecraft struct */
  environment.get_mut_spacecraft().set_atmos_density_kgmmm(&density);
  density
}

/* - - - - - - - - - - - - - - - - - - - - - - - - - -- - - - - -- - -- - - -- - - - - -
 *
 *                                    [PRIVATE FUNCTIONS]
 * 
 *  - - - - - - - - - - - - - - - - - - - - - - - - - -- - - - - -- - -- - - -- - - - - -
 */

/*
 * @brief: Function to compute the desnity for a given altitude on Earth using a 
 *         seventh order polynomial fit and the CIRA model. 
 * 
 * polyfn(h) = a0 + a1*h + a2*h2 + a3*h3 + a4*h4 + a5*h5 + a6*h6 + a7*h7
 *           = ((((((a7*h + a6)*h + a5)*h + a4)*h + a3)*h + a2)*h + a1)*h + a0 
 * 
 *         ! NOTE: The output of this function is only valid for an altitude between
 *                 0 and 180 km 
 * 
 *         See: https://www.spaceacademy.net.au/watch/debris/atmosmod.htm
 * 
 *         Ref: Adolph S Jursa (Ed.), Handbook of Geophysics and the Space Envvironment, 
 *              US Airforce Geophysics Laboratory, 1985 {This text can be obtained from 
 *              the National Technical Information Service, 5285 Port Royal Road, 
 *              Springfield, VA, 22161 - Document Accession Number: ADA 167000} 
 *
 * @returns: Density at the given altitude in kg/m3
 */
fn calculate_density_earth_cira_model_180()
-> f64
{
  let mut density: f64 = 0.0;

  // TODO: fill with content
  density
}

/*
 * @brief: Function to compute the desnity of the upper atmosphere for a given altitude 
 *         on Earth using a model that varies according to the space weather conditions 
 *         at the time.
 * 
 * @description: 
 *
 *         * We use the solar radio ten centimetre flux (F10) as a proxy for the solar EUV output,
 *         * The geomagnetic Ap index as a proxy for the geomagnetic activity
 *         * Both modelling parameters are momentuous values that should be averaged to 
 *           achieve a statistical representation of the space weather in the simulated
 *           time frame. 
 * 
 *         ! NOTE: The output of this function is only valid for an altitude between
 *                 180 and 500 km 
 * 
 *         See: https://www.spaceacademy.net.au/watch/debris/atmosmod.htm
 *
 * @returns: Density at the given altitude in kg/m3
 */
fn calculate_density_earth_model_500(altitude_m: f64, environment: &Environment)
-> (bool, f64)
{
  let mut valid: bool = false;
  let mut density: f64 = 0.0;

  /* Compute altitude in kilometers */
  let altitude_km = altitude_m / 1000.0 ;
  /*  Compute intermediate variables */
  let temp_kelvin: f64 =  900.0 + 2.5 * (environment.get_planet().get_radio_10_cm_flux() - 70.0) 
                          + 1.5 * environment.get_planet().get_geomagnetic_ap_index() ;
  
  if temp_kelvin != 0.0 && altitude_km >= 180.0 && altitude_km < 500.0
  {
    let mu: f64 = 27.0 - 0.012 * (altitude_km - 200.0) ;
    let base: f64 = 10.0;
    density = 6.0 * base.powf(-10.0) * exp( -1.0 * (altitude_km - 175.0) * mu / temp_kelvin ) ;
    valid = true;
  }
  (valid, density)
}