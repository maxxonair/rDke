/*----------------------------------------------------------------------------*/
/*
 *                  [Paramter file paths]
 * 
 */

 /*
  * @brief: CIRA atmoshphere model (EARTH) polynomial fit constant to compute the 
  *         density value for a given altitude.
  *         The CIRA model is the COSPAR International Reference Atmosphere. 
  *         COSPAR is the United Nations Committee for Space Research.
  *  
  */
pub const ATMOS_CIRA_DENSITY_MODEL_CONST_C0: f64 =  7.001985e-2;
pub const ATMOS_CIRA_DENSITY_MODEL_CONST_C1: f64 = -4.336216e-3;
pub const ATMOS_CIRA_DENSITY_MODEL_CONST_C2: f64 = -5.009831e-3;
pub const ATMOS_CIRA_DENSITY_MODEL_CONST_C3: f64 =  1.621827e-4;
pub const ATMOS_CIRA_DENSITY_MODEL_CONST_C4: f64 = -2.471283e-6;
pub const ATMOS_CIRA_DENSITY_MODEL_CONST_C5: f64 =  1.904383e-8;
pub const ATMOS_CIRA_DENSITY_MODEL_CONST_C6: f64 = -7.189421e-11;
pub const ATMOS_CIRA_DENSITY_MODEL_CONST_C7: f64 =  1.060067e-13;

 /*
  * @brief: Reference table column index for the mean free path data table.
  *         This constant determines the column index at which the geometric altitude 
  *         reference is saved.
  *  
  */
pub const ATMOS_MEAN_FREE_PATH_TABLE_INDX_ALTITUDE: usize = 1;

 /*
  * @brief: Reference table column index for the mean free path data table.
  *         This constant determines the column index at which the mean free path value
  *         is saved.
  *  
  */
pub const ATMOS_MEAN_FREE_PATH_TABLE_INDX_MEAN_FREE_PATH: usize = 0;

 /*
  * @brief: Path at which the mean free path table data can be loaded
  *  
  */
  pub const ATMOS_MEAN_FREE_PATH_TABLE_PATH: &str = "assets/atmosphere/earth/mean_free_path.csv";