
/*
 * @brief: Class to implement constant step size 4th order Runge-Kutta ODE solver 
 *
 * 
 */
 use ndarray::Array1;

 /* Import constants */
 use crate::constants::state::*;

 /*
 * -----------------------------------------------------------------------------
 *                    [struct definition]
 * -----------------------------------------------------------------------------
 */

 /*
  * @brief: This function implements a single integration step using the forth
  *         order Runge-Kutta method
  *
  */
pub fn step(x_in: &Array1<f64>,
            dxdt: &dyn Fn(f64, &Array1<f64>, &Array1<f64>) -> Array1<f64>,
            t_in: f64,
            dt: f64) 
-> Array1<f64>
{
  /* Compute forth order Runge-Kutta weighted averages */
  let k1 = dxdt(t_in, x_in, x_in);
  let k2 = dxdt(t_in + dt * 0.5,
                                        &(k1.clone() * dt * 0.5),
                                        x_in);
  let k3 = dxdt(t_in + dt * 0.5,
                                        &(k2.clone() * dt * 0.5),
                                        x_in);
  let k4 = dxdt(t_in + dt,
                                        &(k3.clone() * dt),
                                        x_in);

  /* Compute state and time at t+dt */
  x_in + (k1 + k2 + k3 + k4) * dt * (1.0 / 6.0)
}