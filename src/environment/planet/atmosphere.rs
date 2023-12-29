
/* Include external crates */
use libm::exp;

/* Include local crates */
use crate::io::read_csv::*;
use crate::math::lin_math::*;

/* constants */
use crate::constants::atmosphere::*;

#[derive(Clone)]

pub struct Atmosphere {
  /* [semi-major axis] 
   * @description : Planets semi-major axis (for elliptical model)
   * @unit        : m
   * 
   * */
  density_kgmmm: f64,
  ambient_pressure_pa: f64,
  temperature_k: f64,
  mean_free_path_m: f64,
  knudsen_number: f64,
  radio_10_cm_flux: f64,
  geomagnetic_ap_index: f64,
  enable_atmosphere_modelling: bool,
  mean_free_path_lut_vec: Vec<(f64, f64)>
}

/*
 * ----------------------------------------------------------------------
 *                    [constructor]
 * ----------------------------------------------------------------------
 */
impl Atmosphere {
  pub fn new() 
  -> Atmosphere 
  {
    Atmosphere {
      density_kgmmm: 0.0,
      ambient_pressure_pa: 0.0,
      temperature_k: 0.0,
      mean_free_path_m: 0.0,
      knudsen_number: 0.0,
      radio_10_cm_flux: 0.0,
      geomagnetic_ap_index: 0.0,
      enable_atmosphere_modelling: false,
      mean_free_path_lut_vec: Vec::new()
    }
  }

  /* 
   * @brief: Function to complete initializing the class after the struct has 
   *         been created. This function usually contains file loaders.
   */
  pub fn init(&mut self) 
  {
    self.load_mean_free_path_lut();
  }
}

/*
 * ----------------------------------------------------------------------
 *                    [setters]
 * ----------------------------------------------------------------------
 */
impl Atmosphere
{
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
impl Atmosphere
{
  pub fn get_density_kgmmm(&self) -> &f64 {&self.density_kgmmm}
  pub fn get_radio_10_cm_flux(&self) -> &f64 {&self.radio_10_cm_flux}
  pub fn get_geomagnetic_ap_index(&self) -> &f64 {&self.geomagnetic_ap_index}
  pub fn get_knudsen_number(&self) -> &f64 {&self.knudsen_number}

  pub fn is_atmoshpere_modelled(&self) -> &bool {&self.enable_atmosphere_modelling}
}

/*
 * ----------------------------------------------------------------------
 *                    [amosphere class functions -> API]
 * ----------------------------------------------------------------------
 * 
 * Note: Everything inside this implementation defines the API for this 
 *       model. Everthing outside should be private functions building 
 *       its individual implementation.
 *
 */
impl Atmosphere {
  /*
  * @brief: Read mean free path data from file. This function shall be called before 
  *         running the simulation!
  * 
  *
  * @returns: 
  */
  pub fn load_mean_free_path_lut(&mut self) 
  {
    let vec_altitude_km: Vec<f64> = read_csv_column_f64(&ATMOS_MEAN_FREE_PATH_TABLE_PATH, true, ATMOS_MEAN_FREE_PATH_TABLE_INDX_ALTITUDE);
    let vec_mean_free_path_m: Vec<f64> = read_csv_column_f64(&ATMOS_MEAN_FREE_PATH_TABLE_PATH, true, ATMOS_MEAN_FREE_PATH_TABLE_INDX_MEAN_FREE_PATH);

    for (i, x) in vec_mean_free_path_m.iter().enumerate() 
    {
      self.mean_free_path_lut_vec.push((vec_altitude_km[i], vec_mean_free_path_m[1]));
    }
  }

 /*
  * @brief: Wrapper to update the atmospheric density value
  * 
  */
  pub fn update_density(&mut self, altitude_m: f64)
  {
    self.density_kgmmm = self.calculate_density(altitude_m);
  }

 /*
  * @brief: Function to update the atmospheric, molecular mean free path
  *
  * @description: The mean free path is interpolated from a lookup table
  * 
  */
  pub fn update_mean_free_path_and_kn(&mut self, altitude_m: f64, characteristic_length_m: f64)
  {
    // TODO implementation
    let mut index: usize = 0;
    let max_length: usize = self.mean_free_path_lut_vec.len()-1;
    let altitude_km: f64 = altitude_m / 1000.0;

    /*  (1) find lut index */
    if self.mean_free_path_lut_vec[max_length].0 < altitude_km
    {
      index = max_length;
    }
    else 
    {
      for (i, x) in self.mean_free_path_lut_vec.iter().enumerate() 
      {
        if altitude_km < x.0 && i == 0
        {
          break;
        }
        else if altitude_km < x.0
        {
          index = i - 1;
          break;
        }
      }   
    }

    /* (2) interpolate value */ 
    if index == 0 
    {
      self.mean_free_path_m = self.mean_free_path_lut_vec[0].1;
    }
    else if index == max_length
    {
      self.mean_free_path_m = self.mean_free_path_lut_vec[max_length].1;
    }
    else 
    {
      self.mean_free_path_m = linear_interpolate(altitude_km, 
                                                        self.mean_free_path_lut_vec[index].0, 
                                                        self.mean_free_path_lut_vec[index+1].0, 
                                                        self.mean_free_path_lut_vec[index].1, 
                                                        self.mean_free_path_lut_vec[index+1].1)
    }

    /* (3) Compute Kn number */ 
    self.knudsen_number = self.mean_free_path_m / characteristic_length_m;

  }
}


/*
 * ----------------------------------------------------------------------
 *                    [atmosphere functions]
 * ----------------------------------------------------------------------
 * - - - - - - - - - - - - - - - - - - - - - - - - - -- - - - - -- - -- -
 *
 *                        [PRIVATE FUNCTIONS]
 * 
 *  - - - - - - - - - - - - - - - - - - - - - - - - - -- - - - - -- - - - 
 * Note: All the following functions are outside the API for this model
 */
impl Atmosphere
{
  /*
   * @brief: Main function to return the atmospheric density for a given altitude.
   * 
   *
   * @returns: Density at the given altitude in kg/m3
   */
  fn calculate_density(&mut self, altitude_m: f64)
  -> f64
  {
    /* Initialize density variable */
    let mut density: f64 = 0.0;
  
    if altitude_m < 180000.0 
    {
      density = self.calculate_density_earth_cira_model_180(altitude_m);
    }
    else if altitude_m < 500000.0
    {
      let res: (bool, f64) = self.calculate_density_earth_model_500(altitude_m);
      if res.0 == true
      {
        density = res.1;
      }
    }
    else
    {
      /* DO NOTHING FOR NOW */
    }
  
    density
  }
  
  /*
   * @brief: Function to compute the density for a given altitude on Earth using a 
   *         seventh order polynomial fit and the CIRA model. The CIRA model is the 
   *         COSPAR International Reference Atmosphere. COSPAR is the United Nations 
   *         Committee for Space Research.
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
  fn calculate_density_earth_cira_model_180(&mut self, altitude_m: f64)
  -> f64
  {
    /* Compute altitude in meters */
    let altitude_km: f64 = altitude_m / 1000.0;
    
    /* Compute polyfn(h) */
    let rho: f64 = ((((((ATMOS_CIRA_DENSITY_MODEL_CONST_C7 
      * altitude_km + ATMOS_CIRA_DENSITY_MODEL_CONST_C6)
      * altitude_km + ATMOS_CIRA_DENSITY_MODEL_CONST_C5)
      * altitude_km + ATMOS_CIRA_DENSITY_MODEL_CONST_C4) 
      * altitude_km + ATMOS_CIRA_DENSITY_MODEL_CONST_C3) 
      * altitude_km + ATMOS_CIRA_DENSITY_MODEL_CONST_C2) 
      * altitude_km + ATMOS_CIRA_DENSITY_MODEL_CONST_C1) 
      * altitude_km + ATMOS_CIRA_DENSITY_MODEL_CONST_C0 ;
      
    /* Compute desnity value corresponding to given altitude */
    let density: f64 = 10.0_f64.powf(rho);
  
    density
  }
  
  /*
   * @brief: Function to compute the density for a given altitude on Earth using a 
   *         simplified, isothermal model
   * 
   * @description: Despite the changing scale height, a simple isothermal model with 
   *               a fixed scale height can represent the atmosphere up to 100 km with 
   *               an error less than 50%, and up to 130 km with an error no larger than 
   *               a factor of 2. These errors are quite comparable to the variation that 
   *               the real atmosphere shows from day to day from the mean 'standard' model. 
   *               This model is quite suitable for demonstrating the physics of meteor 
   *               flight and space debris reentries. High velocity meteoroids (~60 km/s) 
   *               start ablating around 130 km altitude, but space debris reentries, with 
   *               the much lower velocity of ~ 8 km/s only start to show significant 
   *               atmospheric interaction below 90 km. 
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
  fn calculate_density_earth_model_180_isothermal(&mut self, altitude_m: f64)
  -> f64
  {
    let mut density: f64 = 0.0;
  
    /* Compute density using  a simplified isothermal model */
    density = 1.3 * exp( -1.0 * altitude_m / 7000.0 );
  
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
  fn calculate_density_earth_model_500(&mut self, altitude_m: f64)
  -> (bool, f64)
  {
    let mut valid: bool = false;
    let mut density: f64 = 0.0;
  
    /* Compute altitude in kilometers */
    let altitude_km = altitude_m / 1000.0 ;
    /*  Compute intermediate variables */
    let temp_kelvin: f64 =  900.0 + 2.5 * (self.get_radio_10_cm_flux() - 70.0) 
                            + 1.5 * self.get_geomagnetic_ap_index() ;
    
    if temp_kelvin != 0.0 && altitude_km >= 180.0 && altitude_km < 500.0
    {
      let mu: f64 = 27.0 - 0.012 * (altitude_km - 200.0) ;
      let base: f64 = 10.0;
      density = 6.0 * base.powf(-10.0) * exp( -1.0 * (altitude_km - 175.0) * mu / temp_kelvin ) ;
      valid = true;
    }
    (valid, density)
  }
}