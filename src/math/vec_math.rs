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

/*
 * @brief: Scale Vec<f64> with scale factor 
 * 
 */
pub fn scale_vec_f64(mut vec: Vec<f64>, scale_factor: f64) -> Vec<f64> {
  for element in vec.iter_mut() {
    *element *= scale_factor;
  }
  vec
}


/*
 * @brief: Find maximum value in Vec<f64> 
 *         Note: NaN entries are ignored!
 * 
 */
pub fn find_max_vec_f64(vec: &Vec<f64>) -> f64 {
  let max_value: f64 = vec.iter().cloned().fold(-1./0. /* -inf */, f64::max);
  max_value
}

/*
 * @brief: Find minimum value in Vec<f64>
 *         Note: NaN entries are ignored!
 * 
 */
pub fn find_min_vec_f64(vec: &Vec<f64>) -> f64 {
  let min_value: f64 = vec.iter().cloned().fold(99999./0. /* -inf */, f64::min);
  min_value
}

/*
 * @brief: Pointwise square a Array1 vector 
 * 
 */
pub fn pointwise_square(arr: Array1<f64>) -> Array1<f64> {
  arr.mapv(|x| x * x)
}