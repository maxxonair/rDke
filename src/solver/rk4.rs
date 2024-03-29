
/*
 * @brief: Class to implement constant step size 4th order Runge-Kutta ODE solver 
 *
 * 
 */

 /* Import external crates */
 use ndarray::Array1;

 /* Import local crates */
use crate::environment::environment::Environment;

 /* Import constants */
/* None */

 /*
 * -----------------------------------------------------------------------------
 *                    [struct definition]
 * -----------------------------------------------------------------------------
 */

 /*
  * @brief: This function implements a single integration step using the forth
  *         order Runge-Kutta method
  *
  * @param[in] : x_in - Vector containing the full state vector of the previous 
  *                     step
  *
  * @param[in] : dxdt - Function containing the  equations of motion
  *
  * @param[in] : t_in - Time corresponding to the previous state 
  *
  */
pub fn step(x_in: &Array1<f64>,
            dxdt: &dyn Fn(&Array1<f64>, &mut Environment) -> Array1<f64>,
            dt: f64,
            environment: &mut Environment) 
-> Array1<f64>
{
  /* Compute forth order Runge-Kutta weighted averages */
  let x_1: Array1<f64> = x_in.clone();
  let k1: Array1<f64> = dxdt(&x_1, environment);
  
  let x_2: Array1<f64> = x_in.clone() + k1.clone() * (dt / 2.0);
  let k2: Array1<f64> = dxdt(&x_2, environment);
  
  let x_3: Array1<f64> = x_in.clone() + k2.clone() * (dt / 2.0);
  let k3: Array1<f64> = dxdt(&x_3, environment);
  
  let x_4: Array1<f64> = x_in.clone() + k3.clone() * dt;
  let k4: Array1<f64> = dxdt(&x_4, environment);

  /* Propagate state at t + dt */
  x_in + (k1 + 2.0 * k2 + 2.0 * k3 + k4) * dt / 6.0
}