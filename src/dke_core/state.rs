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
use crate::math::vec3::Vec3;
use crate::math::vec9::Vec9;
use crate::math::quat::Quat;

use ndarray::Array1;

use crate::constants::constants::STATE_VEC_NUM_ELEMENTS;

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
  /* [state vector]
   * @frame_type : N/A
   * @unit       : SI
   * @description: Fixed size array with vector containing all elements 
   *               of the state in a vector
   */
  state_vector: Array1<f64>,
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
      state_vector: Array1::<f64>::zeros(STATE_VEC_NUM_ELEMENTS)
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
  pub fn get_vector(&mut self) -> Array1<f64> {
    let mut vec_out = Array1::<f64>::zeros(STATE_VEC_NUM_ELEMENTS);
    // Compose state vector from elements
    vec_out[0] = self.position_xyz_m.get_x();
    vec_out[1] = self.position_xyz_m.get_y();
    vec_out[2] = self.position_xyz_m.get_z();

    vec_out[3] = self.attitude_quat.get_x();
    vec_out[4] = self.attitude_quat.get_y();
    vec_out[5] = self.attitude_quat.get_z();
    vec_out[6] = self.attitude_quat.get_w();

    vec_out[7] = self.velocity_xyz_ms.get_x();
    vec_out[8] = self.velocity_xyz_ms.get_y();
    vec_out[9] = self.velocity_xyz_ms.get_z();

    vec_out[10] = self.angular_rate_xyz_rads.get_x();
    vec_out[11] = self.angular_rate_xyz_rads.get_y();
    vec_out[12] = self.angular_rate_xyz_rads.get_z();

    vec_out[13] = self.acceleration_xyz_mss.get_x();
    vec_out[14] = self.acceleration_xyz_mss.get_y();
    vec_out[15] = self.acceleration_xyz_mss.get_z();

    vec_out[16] = self.angular_acc_xyz_radss.get_x();
    vec_out[17] = self.angular_acc_xyz_radss.get_y();
    vec_out[18] = self.angular_acc_xyz_radss.get_z();

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
/*
 * ----------------------------------------------------------------------
 *                    [function]
 * ----------------------------------------------------------------------
 */

