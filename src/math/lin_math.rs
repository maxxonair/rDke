
/*
 * @brief: Function to linear interpolate a value y for a given x and two data 
 *         points x0/y0 and x1/y1
 * 
 * @returns: f64 - interpolated value y
 * 
 */
pub fn linear_interpolate(x: f64, x0: f64, x1: f64, y0: f64, y1: f64)
-> f64
{
  let y: f64 = y0 + ( x - x0 ) * (y1 - y0) / (x1 - x0) ; y
}

/*
  * @brief: Function to compute a y value for a given x value from a x/y look-up table
  * 
  * @returns: y value f64 
  */
pub fn find_value_from_lut(vec_lut_xy: &Vec<(f64, f64)>, x_val: f64) 
-> f64
{
  if vec_lut_xy.len() == 0
  {
    println!("[WRN] Length of LUT is zero. Returning 0");
    return  0.0
  }

  /* Initialize LUT search index */
  let mut index: usize = 0;
  /* Get length of LUT array */
  let max_length: usize = vec_lut_xy.len() - 1;

  /* Initialize variable to store y value  */
  let mut y_val_out: f64;

  /*  (1) find lut index */
  if vec_lut_xy[max_length].0 < x_val
  {
    index = max_length;
  }
  else 
  {
    for (i, x) in vec_lut_xy.iter().enumerate() 
    {
      if x_val < x.0 && i == 0
      {
        break;
      }
      else if x_val < x.0
      {
        index = i - 1;
        break;
      }
    }   
  }

  /* (2) interpolate value */ 
  if index == 0 
  {
    /* Value outside LUT -> x < x_min */
    y_val_out = vec_lut_xy[0].1;
  }
  else if index == max_length
  {
    /* Value outside LUT -> x > x_max */
    y_val_out = vec_lut_xy[max_length].1;
  }
  else 
  {
    /* Linear interpolate between points inside LUT */
    y_val_out = linear_interpolate(x_val, 
                                vec_lut_xy[index].0, 
                                vec_lut_xy[index+1].0, 
                                vec_lut_xy[index].1, 
                                vec_lut_xy[index+1].1);
  }

  y_val_out
}