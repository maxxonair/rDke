
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
pub const STATE_VEC_NUM_ELEMENTS: usize = 21;

/* 
 * @brief: State vector indices
 *  
 * @description: See state class for full state description
 * 
 * @unit:  N/A
 * @frame: N/A
 */
pub const STATE_VEC_INDX_SIM_TIME: usize   =  0;
pub const STATE_VEC_INDX_POS_X: usize      =  1;
pub const STATE_VEC_INDX_POS_Y: usize      =  2;
pub const STATE_VEC_INDX_POS_Z: usize      =  3;
pub const STATE_VEC_INDX_VEL_X: usize      =  4;
pub const STATE_VEC_INDX_VEL_Y: usize      =  5;
pub const STATE_VEC_INDX_VEL_Z: usize      =  6;
pub const STATE_VEC_INDX_ACC_X: usize      =  7;
pub const STATE_VEC_INDX_ACC_Y: usize      =  8;
pub const STATE_VEC_INDX_ACC_Z: usize      =  9;
pub const STATE_VEC_INDX_ATTQ_X: usize     = 10;
pub const STATE_VEC_INDX_ATTQ_Y: usize     = 11;
pub const STATE_VEC_INDX_ATTQ_Z: usize     = 12;
pub const STATE_VEC_INDX_ATTQ_W: usize     = 13;
pub const STATE_VEC_INDX_ATTRATE_X: usize  = 14;
pub const STATE_VEC_INDX_ATTRATE_Y: usize  = 15;
pub const STATE_VEC_INDX_ATTRATE_Z: usize  = 16;
pub const STATE_VEC_INDX_ATTACC_X: usize   = 17;
pub const STATE_VEC_INDX_ATTACC_Y: usize   = 18;
pub const STATE_VEC_INDX_ATTACC_Z: usize   = 19;
pub const STATE_VEC_INDX_MASS: usize       = 20;
/*----------------------------------------------------------------------------*/
/*
 *                  [Simulation timer constants]
 * 
 */
/* 
 * @brief: Time interval between status print outs while the simulator is 
 *         running in simulated time
 * 
 * @unit:  seconds (of simulated time)
 * @frame: N/A
 */
pub const SIMULATION_PRINT_INTERVAL_S: f64 = 5.0;

/* 
 * @brief: Time interval between state write to csv while the simulator is 
 *         running in simulated time
 * 
 * @unit:  seconds (of simulated time)
 * @frame: N/A
 */
pub const SIMULATION_WRITE_INTERVAL_S: f64 = 0.5;

/* 
 * @brief: Time interval when the simulation result writer is flushed
 * 
 * @unit:  seconds (of simulated time)
 * @frame: N/A
 */
pub const SIMULATION_WRITE_FLUSH_INTERVAL_S: f64 = 10.0;

/*----------------------------------------------------------------------------*/
/* 
 * @brief: This constant defines 1/6 and is used to avoid an unnecessary 
 *         division in the rk4 stepper function. 
 * 
 * @unit:  N/A
 * @frame: N/A
 */
pub const ONE_DEVIDED_BY_SIX: f64       = 1.0 / 6.0;
