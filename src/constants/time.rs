
/* 
 * @brief: (Chrono) DateTime format string  
 *  
 * @description: Format %Y-%m-%d %H:%M:%S %z
 * 
 * @unit:  N/A
 * @frame: N/A
 */
pub const DATETIME_FORMAT: &str = " %Y-%m-%d %H:%M:%S %z" ;

/* 
 * @brief: J2000 Epoch in UTC and DateTime format 
 *  
 * @description: Format DATETIME_FORMAT
 * 
 * @unit:  N/A
 * @frame: N/A
 */
pub const EPOCH_J2000_DATETIME_STR: &str = "2000-01-01 12:00:00 +00:00" ;

/* 
 * @brief: J2000 Epoch in Julian days
 *  
 * @description: Julian days at J2000 epoch at January 1st, 2000, 12:00:00 UTC
 * 
 * @unit:  days
 * @frame: N/A
 */
pub const JULIAN_DAYS_AT_J2000_EPOCH: f64 = 2451545.0;

/* 
 * @brief: Unix Epoch in Julian days
 *  
 * @description: Julian days at Unix epoch at January 1st, 1970, 00:00:00 UTC
 *               This allows to compute the Unix time (UT) from Julian Time (JD)
 *               UT = (JD - JULIAN_DAYS_AT_UNIX_EPOCH) * 86400
 * 
 * @unit:  days
 * @frame: N/A
 */
pub const JULIAN_DAYS_AT_UNIX_EPOCH: f64 = 2440587.5;

/* 
 * @brief: J2000 Epoch in Unix seconds
 *  
 * @description: Unix seconds at J2000 epoch at January 1st, 2000, 12:00:00 UTC
 * 
 * @unit:  seconds
 * @frame: N/A
 */
pub const UNIX_SECONDS_AT_J2000_EPOCH: i64 = 946728000;

/* 
 * @brief: Days between Julian Day (JD) and Modified Julian Day (MJD).
 *  
 * @description: This constant is used to convert between JD and MJD:
 *               MJD = JD - DIFF_MJD_TO_JD
 * 
 * @unit:  days
 * @frame: N/A
 */
pub const DIFF_MJD_TO_JD: f64 = 2400000.5;

/* 
 * @brief: Number of seconds within a solar day on Earth
 *  
 * 
 * @unit:  days
 * @frame: N/A
 */
pub const SECONDS_OF_EARTH_DAY: f64 = 86400.0;

/* 
 * @brief: Mean number of days in a year on Earth
 *  
 * 
 * @unit:  days
 * @frame: N/A
 */
pub const MEAN_DAYS_IN_EARTH_YEAR: f64 = 365.25;