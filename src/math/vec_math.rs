
use super::vec3::Vec3;

pub fn add_vec3(vec1: Vec3, vec2: Vec3) -> Vec3 {
  let mut res_vec = Vec3::new();

  let xx: f64 = vec1.get_x() + vec2.get_x();
  let yy: f64 = vec1.get_y() + vec2.get_y();
  let zz: f64 = vec1.get_z() + vec2.get_z();
  res_vec.set_x(&xx);
  res_vec.set_y(&yy);
  res_vec.set_z(&yy);

  res_vec
}