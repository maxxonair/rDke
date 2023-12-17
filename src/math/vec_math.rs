use ndarray::{Array1, ArrayView1, s};

/*
 * @brief: Calculate l2 norm of a Array1 vector
 *
 */
pub fn l2_norm_array1(vec: ArrayView1<f64>) -> f64 {
  vec.dot(&vec).sqrt()
}

/*
 * @brief: Normalize Array1 vector
 * 
 */
pub fn normalize_array1(mut vec: Array1<f64>) -> Array1<f64> {
  let norm: f64 = l2_norm_array1(vec.view());
  vec.mapv_inplace(|e: f64| e/norm);
  vec
}