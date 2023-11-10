use chrono::{DateTime, Timelike, Datelike, Utc};

use crate::constants::time::*;

/*
 * @brief: Calculate the Greenwich Apparent Sideral Time (GAST) for a given 
 *         DateTime
 * 
 * @description: Greenwich Apparent Sidereal Time (GAST), Greenwich Mean Sidereal 
 *               Time (GMST) are sidereal time with respect to true equinox and 
 *               mean equinox, respectively. Equation of equinoxes is nutation 
 *               in right ascension and GAST = GMST + Equation of equinoxes.
 * 
 *               The following implementation to compute the GAST from a given 
 *               DateTime is derived from the following source: 
 *               https://aa.usno.navy.mil/faq/GAST
 * 
 * Note: This function is only valid if Earth is the central body!
 */
pub fn calc_earth_gast_deg(datetime: DateTime<Utc>) 
-> f64 
{
  /* Get GMST in hours */
  let gmst_hours: f64 = calc_earth_gmst_deg(datetime) / 360.0 * 24.0;

  /* Compute GAST in degree */
  let gast_hours: f64 = gmst_hours + calc_equ_equinoxes_hours(datetime);

  let gast_deg: f64 = gast_hours / 24.0 * 360.0;

  gast_deg
}

/*
 * @brief: Calculate the Greenwich Mean Sideral Time (GMST) for a given DateTime
 *         in degree.
 * 
 * @description: The following implementation to compute the GMST from a given 
 *               DateTime is derived from the following source: 
 *               https://aa.usno.navy.mil/faq/GAST
 * 
 * Note: This function is only valid if Earth is the central body!
 * 
 * @param[in] datetime chrono::DateTime 
 * 
 * @returns: GMST in degree
 * 
 */
pub fn calc_earth_gmst_deg(datetime: DateTime<Utc>)
-> f64
{
  let j2000_day: f64 = calc_julian_day(datetime) - JULIAN_DAYS_AT_J2000_EPOCH;
  let j2000_year: f64 = j2000_day / MEAN_DAYS_IN_EARTH_YEAR;
  let hours_since_midnight: f64 = datetime.hour() as f64;

  let gmst_hours: f64 = (18.697375 + 24.065709824279 * j2000_day ) % 24.0;
  let gmst_deg: f64 = gmst_hours / 24.0 * 360.0;

  gmst_deg
}

/*
 * @brief: Calculate Julian date in days
 * 
 * @description: Julian date is days passed from noon on Jan. 1, 4713 B.C.. 
 *               In "Julian Date" and "Julian Date -> Calendar Date",
 *               the year before 1 A.D. is year 0.
 * 
 *               From the following source: 
 *               https://aa.usno.navy.mil/faq/GAST
 * 
 * Note: This function is only valid if Earth is the central body!
 * 
 * @param[in] datetime chrono::DateTime 
 * 
 * @returns: Julian date in days
 * 
 */
pub fn calc_julian_day(datetime: DateTime<Utc>)
-> f64
{
  /* datetime as julian day */
  let julian_day: f64 = datetime.timestamp() as f64 / SECONDS_OF_EARTH_DAY 
  + JULIAN_DAYS_AT_UNIX_EPOCH;
  julian_day
}

/*
 * @brief: Calculate days since J2000 epoch
 * 
 * @description: From the following source: 
 *               https://aa.usno.navy.mil/faq/GAST
 * 
 * Note: This function is only valid if Earth is the central body!
 * 
 * @param[in] datetime chrono::DateTime 
 * 
 * @returns: days since J2000 epoch
 * 
 */
pub fn calc_j2000_day(datetime: DateTime<Utc>)
-> f64
{
  /* datetime as days since J2000 epoch */
  let j2000_day: f64 = calc_julian_day(datetime) - JULIAN_DAYS_AT_J2000_EPOCH;
  j2000_day
}

/*
 * @brief: Calculate equation of equinoxes
 * 
 * @description: From the following source: 
 *               https://aa.usno.navy.mil/faq/GAST
 * 
 * Note: This function is only valid if Earth is the central body!
 * 
 * @param[in] datetime chrono::DateTime 
 * 
 * @returns: equation of equinoxes in hours
 * 
 */
pub fn calc_equ_equinoxes_hours(datetime: DateTime<Utc>)
-> f64
{
  let j2000_day: f64 = calc_j2000_day(datetime);

  /* Mean longitude of the sun in degree */
  let mean_long_sun_deg: f64 = 280.47 + 0.98565 * j2000_day;
  /* Longitude of the ascending node of the Moon in degree*/
  let asc_long_moon_deg: f64 = 125.04 - 0.052954 * j2000_day;
  /* Obliquity in degrees */
  let obliquity_deg: f64 = 23.4393 - 0.0000004 * j2000_day;

  /* Compute nutation in longitude */
  let equat_of_equinoxes: f64 = -0.000319 * (asc_long_moon_deg.to_radians()).sin() 
                      - 0.000024 * (2.0 * mean_long_sun_deg.to_radians()).sin()
                      * (obliquity_deg.to_radians()).cos();
  equat_of_equinoxes
}