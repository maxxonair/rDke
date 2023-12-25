
/* 
 * This file contains all hard coded, not to be touched constans
 * 
 */
/*----------------------------------------------------------------------------*/
/*
 *                  [State constants]
 * 
 */
/* 
 * @brief: Number of elements in full state vector 
 *  
 * @description: See state class for full state description
 * 
 * @unit:  N/A
 * @frame: N/A
 */
pub const STATE_VEC_NUM_ELEMENTS: usize = 34;

/* 
 * @brief: State vector indices
 *  
 * @description: See state class for full state description
 * 
 * @unit:  N/A
 * @frame: N/A
 */
pub const STATE_VEC_INDX_SIM_TIME: usize           =  0;
pub const STATE_VEC_INDX_POS_X: usize              =  1;
pub const STATE_VEC_INDX_POS_Y: usize              =  2;
pub const STATE_VEC_INDX_POS_Z: usize              =  3;
pub const STATE_VEC_INDX_VEL_X: usize              =  4;
pub const STATE_VEC_INDX_VEL_Y: usize              =  5;
pub const STATE_VEC_INDX_VEL_Z: usize              =  6;
pub const STATE_VEC_INDX_ACC_X: usize              =  7;
pub const STATE_VEC_INDX_ACC_Y: usize              =  8;
pub const STATE_VEC_INDX_ACC_Z: usize              =  9;
pub const STATE_VEC_INDX_ATTQ_X: usize             = 10;
pub const STATE_VEC_INDX_ATTQ_Y: usize             = 11;
pub const STATE_VEC_INDX_ATTQ_Z: usize             = 12;
pub const STATE_VEC_INDX_ATTQ_W: usize             = 13;
pub const STATE_VEC_INDX_ATTRATE_X: usize          = 14;
pub const STATE_VEC_INDX_ATTRATE_Y: usize          = 15;
pub const STATE_VEC_INDX_ATTRATE_Z: usize          = 16;
pub const STATE_VEC_INDX_ATTACC_X: usize           = 17;
pub const STATE_VEC_INDX_ATTACC_Y: usize           = 18;
pub const STATE_VEC_INDX_ATTACC_Z: usize           = 19;
pub const STATE_VEC_INDX_MASS: usize               = 20;
pub const STATE_VEC_INDX_J2000_S: usize            = 21;
pub const STATE_VEC_INDX_ALTITUDE_PCPF_M: usize    = 22;
pub const STATE_VEC_INDX_POS_PCPF_LAT_DEG: usize   = 23;
pub const STATE_VEC_INDX_POS_PCPF_LONG_DEG: usize  = 24;
pub const STATE_VEC_INDX_GRAV_ACC_MSS: usize       = 25;
pub const STATE_VEC_INDX_GAST_DEG: usize           = 26;
pub const STATE_VEC_INDX_VEL_MAGN_PCI_MS: usize    = 27;
pub const STATE_VEC_INDX_ATMOS_DENSITY: usize      = 28;
pub const STATE_VEC_INDX_AERO_FORCE_X: usize       = 29;
pub const STATE_VEC_INDX_AERO_FORCE_Y: usize       = 30;
pub const STATE_VEC_INDX_AERO_FORCE_Z: usize       = 31;
pub const STATE_VEC_INDX_DRAG_COEFF: usize         = 32;
pub const STATE_VEC_INDX_BALLISTIC_COEFF: usize    = 33;