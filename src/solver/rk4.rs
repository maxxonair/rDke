
/*
 * @brief: Class to implement constant step size 4th order Runge-Kutta ODE solver 
 *
 * 
 */

 /* Import external crates */
 use ndarray::Array1;

 /* Import local crates */
use crate::dke_core::dke_core::DKE;

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
            dxdt: &dyn Fn(&Array1<f64>, f64, &DKE) -> Array1<f64>,
            t_in: f64,
            dt: f64,
            dke_core: &DKE) 
-> Array1<f64>
{
  let t1: f64 = t_in;
  let t2: f64 = t_in + (dt / 2.0);
  let t3: f64 = t_in + (dt / 2.0);
  let t4: f64 = t_in + dt;

  /* Compute forth order Runge-Kutta weighted averages */
  let x_1: Array1<f64> = x_in.clone();
  let k1: Array1<f64> = dxdt(&x_1, t1, dke_core);
  
  let x_2: Array1<f64> = x_in.clone() + k1.clone() * (dt / 2.0);
  let k2: Array1<f64> = dxdt(&x_2, t2, dke_core);
  
  let x_3: Array1<f64> = x_in.clone() + k2.clone() * (dt * 0.5);
  let k3: Array1<f64> = dxdt(&x_3, t3, dke_core);
  
  let x_4: Array1<f64> = x_in.clone() + k3.clone() * dt;
  let k4: Array1<f64> = dxdt(&x_4, t4, dke_core);

  /* Propagate state at t + dt */
  x_in + (k1 + 2.0 * k2 + 2.0 * k3 + k4) * dt / 6.0
}