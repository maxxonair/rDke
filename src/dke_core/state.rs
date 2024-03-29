/*
 * @brief: This class is to maintain the full state vector in an easy to access 
 *         format
 * 
 * @description: To run the simulation the full state vector is stored and 
 *               updated in a 1D Array1. 
 *               This class splits the state into time, position, velocity,
 *               attitude quaternion etc.
 * 
 */
/* Import local crates */
use crate::math::vec3::Vec3;
use crate::math::vec9::Vec9;
use crate::math::quat::Quat;

/* Import externals crates */
use ndarray::Array1;
use chrono::*;

/* Import constants */
use crate::constants::state::*;
use crate::constants::time::*;

#[derive(Clone)]

pub struct State {
  /* [state time] 
   * @description : Time associated with state
   * @unit        : seconds
   * 
   * */
  time_s: f64,
  /* [position vector] 
   * @frame_type  : cartesian / body fixed
   * @unit        : meters
   * */
  position_xyz_m: Vec3,
  /* [attitude quaternion] 
   * @frame_type  : inertial to body fixed
   * @unit        : N/A
   */
  attitude_quat: Quat,
   /* [velocity vector]
   * @frame_type  : cartesian / body fixed
   * @unit        : meter/second 
   */
  velocity_xyz_ms: Vec3,
  /* [angular rate]
   * @frame_type  : cartesian / body fixed
   * @unit        : rad/second 
   */
  angular_rate_xyz_rads: Vec3,
  /* [acceleration]
   * @frame_type  : cartesian / body fixed
   * @unit         : m/(second*second) 
   */
  acceleration_xyz_mss: Vec3,
  /* [angular acceleration]
   * @frame_type : cartesian / body fixed
   * @unit       : rad/(second*second) 
   */
  angular_acc_xyz_radss: Vec3,
  /* [inertia matrix]
   * @frame_type : cartesian / body fixed
   * @unit       : kg/(meter*meter)
   */
  inertia_matrix_kgmm: Vec9,
  /* [state S/C mass] 
   * @description : S/C mass
   * @unit        : kg
   * 
   * */
  mass_kg: f64,
  /* [state epoch ] 
   * @description : State epoch in seconds since J2000 epoch
   * @unit        : seconds
   * 
   * */
  state_since_epoch_j2000_s: f64,
  /* [state gast ] 
   * @description : Greenwich aparent sidrereal time
   * @unit        : degree
   * 
   * */
   state_gast_deg: f64, 
  /* [state velocity magnitude inertial frame ] 
   * @description : Maginutde of the S/C velocity vector in inertial frame
   * @unit        : m/s
   * 
   * */
   vel_magn_pci_ms: f64,
  /* [aerodynamic drag coefficient] 
   * @description : Aerodynamic drag coefficient
   * @unit        : None
   * 
   * */
   aero_drag_coeff: f64,
  /* [ballistic coefficient] 
   * @description : Ballistic coefficient
   * @unit        : kg / (m * m)
   * 
   * */
   ballistic_coeff_kgmm: f64,
  /* [mach number] 
   * @description : Mach number
   * @unit        : -
   * 
   * */
   mach_number: f64,
   Knudsen_number: f64
}


impl State {
  pub fn new() -> State {
    State {
      time_s: 0.0,
      position_xyz_m: Vec3::new(),
      attitude_quat: Quat::new(),
      velocity_xyz_ms: Vec3::new(),
      angular_rate_xyz_rads: Vec3::new(),
      acceleration_xyz_mss: Vec3::new(),
      angular_acc_xyz_radss: Vec3::new(),
      inertia_matrix_kgmm: Vec9::new(),
      mass_kg: 1.0,
      state_since_epoch_j2000_s: 0.0,
      state_gast_deg: 0.0,
      vel_magn_pci_ms: 0.0,
      aero_drag_coeff: 0.0,
      ballistic_coeff_kgmm: 0.0,
      mach_number: 0.0,
      Knudsen_number: 0.0
    }
  }
}

/*
 * ----------------------------------------------------------------------
 *                    [getters]
 * ----------------------------------------------------------------------
 */
impl State {
  pub fn get_time(&mut self) -> f64 {
    self.time_s
  }
}

impl State {
  pub fn get_pos(&mut self) -> Vec3 {
    self.position_xyz_m
  }
}

impl State {
  pub fn get_vel(&mut self) -> Vec3 {
    self.velocity_xyz_ms
  }
}

impl State {
  pub fn get_acc(&mut self) -> Vec3 {
    self.acceleration_xyz_mss
  }
}

impl State {
  pub fn get_att(&mut self) -> Quat {
    self.attitude_quat
  }
}

impl State {
  pub fn get_ang_rate(&mut self) -> Vec3 {
    self.angular_rate_xyz_rads
  }
}

impl State {
  pub fn get_ang_acc(&mut self) -> Vec3 {
    self.angular_acc_xyz_radss
  }
}

impl State {
  pub fn get_mass(&mut self) -> f64 {
    self.mass_kg
  }
}

impl State {
  pub fn get_altitude(&mut self, state_in: &Array1<f64>) -> f64 {
    (  (state_in[STATE_VEC_INDX_POS_X]).powf(2.0) 
     + (state_in[STATE_VEC_INDX_POS_Y]).powf(2.0)
     + (state_in[STATE_VEC_INDX_POS_Z]).powf(2.0) ).sqrt()
  }
}

impl State {
  pub fn get_state_epoch(&mut self) -> f64 {
    self.state_since_epoch_j2000_s
  }
}

impl State {
  pub fn get_vector(&mut self) -> Array1<f64> {
    let mut vec_out = Array1::<f64>::zeros(STATE_VEC_NUM_ELEMENTS);
    /* Compose state vector from elements */ 

    /* Simulation time associated to state */
    vec_out[STATE_VEC_INDX_SIM_TIME] = self.time_s;

    /* [Position] */
    vec_out[STATE_VEC_INDX_POS_X] = self.position_xyz_m.get_x();
    vec_out[STATE_VEC_INDX_POS_Y] = self.position_xyz_m.get_y();
    vec_out[STATE_VEC_INDX_POS_Z] = self.position_xyz_m.get_z();

    /* [Velocity] */
    vec_out[STATE_VEC_INDX_VEL_X] = self.velocity_xyz_ms.get_x();
    vec_out[STATE_VEC_INDX_VEL_Y] = self.velocity_xyz_ms.get_y();
    vec_out[STATE_VEC_INDX_VEL_Z] = self.velocity_xyz_ms.get_z();

    /* [Acceleration] */
    vec_out[STATE_VEC_INDX_ACC_X] = self.acceleration_xyz_mss.get_x();
    vec_out[STATE_VEC_INDX_ACC_Y] = self.acceleration_xyz_mss.get_y();
    vec_out[STATE_VEC_INDX_ACC_Z] = self.acceleration_xyz_mss.get_z();

    /* [Attitude Quaternion] */
    vec_out[STATE_VEC_INDX_ATTQ_X] = self.attitude_quat.get_x();
    vec_out[STATE_VEC_INDX_ATTQ_Y] = self.attitude_quat.get_y();
    vec_out[STATE_VEC_INDX_ATTQ_Z] = self.attitude_quat.get_z();
    vec_out[STATE_VEC_INDX_ATTQ_W] = self.attitude_quat.get_w();

    /* [Angular Rate] */
    vec_out[STATE_VEC_INDX_ATTRATE_X] = self.angular_rate_xyz_rads.get_x();
    vec_out[STATE_VEC_INDX_ATTRATE_Y] = self.angular_rate_xyz_rads.get_y();
    vec_out[STATE_VEC_INDX_ATTRATE_Z] = self.angular_rate_xyz_rads.get_z();

    /* [Angular Acceleration] */
    vec_out[STATE_VEC_INDX_ATTACC_X] = self.angular_acc_xyz_radss.get_x();
    vec_out[STATE_VEC_INDX_ATTACC_Y] = self.angular_acc_xyz_radss.get_y();
    vec_out[STATE_VEC_INDX_ATTACC_Z] = self.angular_acc_xyz_radss.get_z();

    /* [Vehicle mass] */
    vec_out[STATE_VEC_INDX_MASS] = self.mass_kg;

    /* [State absolute time as] */
    vec_out[STATE_VEC_INDX_J2000_S] = self.state_since_epoch_j2000_s + self.time_s;

    /* [State greenwich aparent sidereal time] */
    vec_out[STATE_VEC_INDX_GAST_DEG] = self.state_since_epoch_j2000_s + self.time_s;

    /* [State velocity mangitude in inertial frame] */
    vec_out[STATE_VEC_INDX_VEL_MAGN_PCI_MS] = self.vel_magn_pci_ms;

    vec_out[STATE_VEC_INDX_DRAG_COEFF] = self.aero_drag_coeff;
    vec_out[STATE_VEC_INDX_BALLISTIC_COEFF] = self.ballistic_coeff_kgmm;
    vec_out[STATE_VEC_INDX_MACH_NUMBER] = self.mach_number;
    vec_out[STATE_VEC_INDX_KNUDSEN_NUMBER] = self.Knudsen_number;

    vec_out
  }
}
/*
 * ----------------------------------------------------------------------
 *                    [setters]
 * ----------------------------------------------------------------------
 */
impl State {
  pub fn set_time(&mut self, new_time: &f64) {self.time_s = *new_time;}
}

impl State {
  pub fn set_pos(&mut self, new_pos: &Vec3) {self.position_xyz_m = *new_pos;}
}

impl State {
  pub fn set_pos_x(&mut self, new_pos_x: &f64) 
  {self.position_xyz_m.set_x(new_pos_x);}
}

impl State {
  pub fn set_pos_y(&mut self, new_pos_y: &f64) 
  {self.position_xyz_m.set_y(new_pos_y);}
}

impl State {
  pub fn set_pos_z(&mut self, new_pos_z: &f64) 
  {self.position_xyz_m.set_z(new_pos_z);}
}

impl State {
  pub fn set_vel_x(&mut self, new_vel_x: &f64) 
  {self.velocity_xyz_ms.set_x(new_vel_x);}
}

impl State {
  pub fn set_vel_y(&mut self, new_vel_y: &f64) 
  {self.velocity_xyz_ms.set_y(new_vel_y);}
}

impl State {
  pub fn set_vel_z(&mut self, new_vel_z: &f64) 
  {self.velocity_xyz_ms.set_z(new_vel_z);}
}

impl State {
  pub fn set_mass_kg(&mut self, new_mass_kg_in: &f64) 
  {self.mass_kg = *new_mass_kg_in;}
}

impl State {
  pub fn set_date_time(&mut self, date_time_in: &str) 
  {
    let date_time_utc: DateTime<Utc> = DateTime::parse_from_str(date_time_in, 
                                                              DATETIME_FORMAT)
        .unwrap()
        .with_timezone(&Utc);

    /* Set difference as seconds since epoch */
    self.state_since_epoch_j2000_s = (date_time_utc.timestamp() 
                                      - UNIX_SECONDS_AT_J2000_EPOCH) as f64;
    println!("[x] Initialise J2000 time [s]: {:?}",  self.state_since_epoch_j2000_s)
  }
}

impl State 
{
  pub fn set_vector(&mut self, state_vec_in: &Array1<f64>) 
  {
    /* De-compose state vector into elements */
    
    /* Simulation time associated to state */
    self.time_s = state_vec_in[STATE_VEC_INDX_SIM_TIME];

    /* [Position] */
    self.position_xyz_m.set_x(&state_vec_in[STATE_VEC_INDX_POS_X]) ;
    self.position_xyz_m.set_y(&state_vec_in[STATE_VEC_INDX_POS_Y]) ;
    self.position_xyz_m.set_z(&state_vec_in[STATE_VEC_INDX_POS_Z]) ;

    /* [Velocity] */
    self.velocity_xyz_ms.set_x(&state_vec_in[STATE_VEC_INDX_VEL_X]) ;
    self.velocity_xyz_ms.set_y(&state_vec_in[STATE_VEC_INDX_VEL_Y]) ;
    self.velocity_xyz_ms.set_z(&state_vec_in[STATE_VEC_INDX_VEL_Z]) ;

    /* [Acceleration] */
    self.acceleration_xyz_mss.set_x(&state_vec_in[STATE_VEC_INDX_ACC_X]) ;
    self.acceleration_xyz_mss.set_y(&state_vec_in[STATE_VEC_INDX_ACC_Y]) ;
    self.acceleration_xyz_mss.set_z(&state_vec_in[STATE_VEC_INDX_ACC_Z]) ;

    /* [Attitude Quaternion] */
    self.attitude_quat.set_x(&state_vec_in[STATE_VEC_INDX_ATTQ_X]) ;
    self.attitude_quat.set_y(&state_vec_in[STATE_VEC_INDX_ATTQ_Y]) ;
    self.attitude_quat.set_z(&state_vec_in[STATE_VEC_INDX_ATTQ_Z]) ;
    self.attitude_quat.set_w(&state_vec_in[STATE_VEC_INDX_ATTQ_W]) ;

    /* [Angular Rate] */
    self.angular_rate_xyz_rads.set_x(&state_vec_in[STATE_VEC_INDX_ATTRATE_X]) ;
    self.angular_rate_xyz_rads.set_y(&state_vec_in[STATE_VEC_INDX_ATTRATE_Y]) ;
    self.angular_rate_xyz_rads.set_z(&state_vec_in[STATE_VEC_INDX_ATTRATE_Z]) ;

    /* [Angular Acceleration] */
    self.angular_acc_xyz_radss.set_x(&state_vec_in[STATE_VEC_INDX_ATTACC_X]) ;
    self.angular_acc_xyz_radss.set_y(&state_vec_in[STATE_VEC_INDX_ATTACC_Y]) ;
    self.angular_acc_xyz_radss.set_z(&state_vec_in[STATE_VEC_INDX_ATTACC_Z]) ;

    self.mass_kg = state_vec_in[STATE_VEC_INDX_MASS] ;

    self.state_since_epoch_j2000_s = state_vec_in[STATE_VEC_INDX_J2000_S] ;

    self.state_gast_deg = state_vec_in[STATE_VEC_INDX_GAST_DEG] ;

    self.vel_magn_pci_ms = state_vec_in[STATE_VEC_INDX_VEL_MAGN_PCI_MS] ;

    self.aero_drag_coeff = state_vec_in[STATE_VEC_INDX_DRAG_COEFF] ;
    self.ballistic_coeff_kgmm = state_vec_in[STATE_VEC_INDX_BALLISTIC_COEFF] ;
    self.mach_number = state_vec_in[STATE_VEC_INDX_MACH_NUMBER];
    self.Knudsen_number = state_vec_in[STATE_VEC_INDX_KNUDSEN_NUMBER];

  }
}
/*
 * ----------------------------------------------------------------------
 *                    [function]
 * ----------------------------------------------------------------------
 */

