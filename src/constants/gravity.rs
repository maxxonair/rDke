

/*----------------------------------------------------------------------------*/
/*
 *                  [Gravity model constants]
 * 
 */
/*----------------------------------------------------------------------------*/

/* 
 * @brief: Gravitational constant of EARTH as defined by the WGS model 
 *         (including the mass of the atmosphere). 
 * 
 * Source: https://apps.dtic.mil/sti/pdfs/ADA280358.pdf
 * 
 * @unit:  m3/s2
 * @frame: N/A
 */
pub const GM_EARTH_WGS84: f64 = 3986005.0e+8_f64 ;

/* 
 * @brief: Semi-major axis of EARTH as defined by the WGS model 
 * 
 * Source: https://apps.dtic.mil/sti/pdfs/ADA280358.pdf
 * 
 * @unit:  m
 * @frame: N/A
 */
pub const SEMIMAJOR_AXIS_EARTH_WGS84_M: f64 = 6378137.0;


/* 
 * @brief: Semi-major axis of EARTH as defined by the WGS model 
 * 
 * Source: https://apps.dtic.mil/sti/pdfs/ADA280358.pdf
 * 
 * @unit:  N/A (Normalized coefficient)
 * @frame: N/A
 */
pub const C20_NORM_ZONAL_GRAV_COEFF: f64 = -484.16685e-6_f64;
 /*----------------------------------------------------------------------------*/