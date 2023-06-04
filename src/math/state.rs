
use super::vec3::Vec3;
use super::quat::Quat;

#[derive(Debug)]

pub struct State {
  /* [position vector] 
   * @frame_type  : cartesian
   * @unit        : meters
   * */
  position_xyz_m: Vec3,
  /* [attitude quaternion] 
   * @frame_type  : N/A 
   * @unit        : N/A
   */
  attitude_quat: Quat,
  /* velocity vector in cartesian coordinates [m/s] */
  velocity_xyz_ms: Vec3,
  /* [angular rate]
   * @frame_type  : cartesian
   * @unit        : rad/second 
   */
  angular_rate_xyz_rads: Vec3,
  /* [acceleration]
   * @frame_type  : cartesian
   * @unit         : m/(second*second) 
   */
  acceleration_xyz_mss: Vec3,
  /* [angular acceleration]
   * @frame_type : cartesian
   * @unit       : rad/(second*second) 
   */
  angular_acc_xyz_radss: Vec3,
}


impl State {
  pub fn new() -> State {
    State {
      position_xyz_m: Vec3::new(),
      attitude_quat: Quat::new(),
      velocity_xyz_ms: Vec3::new(),
      angular_rate_xyz_rads: Vec3::new(),
      acceleration_xyz_mss: Vec3::new(),
      angular_acc_xyz_radss: Vec3::new()
    }
  }

}

/*
 * ----------------------------------------------------------------------
 *                    [getters and setters]
 * ----------------------------------------------------------------------
 */
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
