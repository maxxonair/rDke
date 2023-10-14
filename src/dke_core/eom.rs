
use ndarray::Array1;
use crate::constants::constants::*;


pub fn dxdt(
    t_in: f64,
    x_in: &Array1<f64>) 
-> Array1<f64>
{
  let mut dxdt_out = Array1
                                                    ::<f64>
                                                    ::zeros(STATE_VEC_NUM_ELEMENTS);

  /* Dummy policy to test the integrator -> Only gravity is pulling us down */
  dxdt_out[0] = x_in[3];
  dxdt_out[1] = x_in[4];
  dxdt_out[2] = x_in[5];

  dxdt_out[3] = 0.0;
  dxdt_out[4] = 0.0;
  dxdt_out[5] = -9.80665;

  dxdt_out
}
