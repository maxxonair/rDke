use ndarray::{Array1, Array2};

/*
 * @brief: Create Direction-Cosine Matrix from Euler 3-2-1 sequence
 * 
 * @description: Euler 3-2-1 -> yaw, pitch, roll
 * 
 */
pub fn dcm_from_e321(z_rotation_deg: f64,
                     y_rotation_deg: f64,
                     x_rotation_deg: f64)
-> Array2<f64>
{
  let mut dcm_3: Array2<f64> = Array2::eye(3);
  let mut dcm_2: Array2<f64> = Array2::eye(3);
  let mut dcm_1: Array2<f64> = Array2::eye(3);

  /* Compose DCM around z axis (yaw) */
  dcm_3[[0, 0]] = z_rotation_deg.to_radians().cos();
  dcm_3[[1, 0]] = z_rotation_deg.to_radians().sin();
  dcm_3[[0, 1]] = - z_rotation_deg.to_radians().sin();
  dcm_3[[1, 1]] = z_rotation_deg.to_radians().cos();
  /* Compose DCM around y axis (pitch) */
  dcm_2[[0, 0]] = y_rotation_deg.to_radians().cos();
  dcm_2[[2, 0]] = y_rotation_deg.to_radians().sin();
  dcm_2[[0, 2]] = - y_rotation_deg.to_radians().sin();
  dcm_2[[2, 2]] = y_rotation_deg.to_radians().cos();
  /* Compose DCM around x axis (roll) */
  dcm_1[[1, 1]] = y_rotation_deg.to_radians().cos();
  dcm_1[[1, 2]] = y_rotation_deg.to_radians().sin();
  dcm_1[[2, 1]] = - y_rotation_deg.to_radians().sin();
  dcm_1[[2, 2]] = y_rotation_deg.to_radians().cos();

  dcm_1 * dcm_2 * dcm_3
}