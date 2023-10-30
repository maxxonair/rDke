
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
  */
pub fn step(x_in: &Array1<f64>,
            dxdt: &dyn Fn(&Array1<f64>, f64, &DKE) -> Array1<f64>,
            t_in: f64,
            dt: f64,
            dke_core: &DKE) 
-> Array1<f64>
{
  /* Compute forth order Runge-Kutta weighted averages */
  let k1 = dxdt(x_in, t_in,
                dke_core);
  let k2 = dxdt(&(x_in + k1.clone() * (dt / 2.0)),t_in + (dt / 2.0),
                dke_core);
  let k3 = dxdt(&(x_in + k2.clone() * (dt * 0.5)),t_in + (dt * 0.5),
                dke_core);
  let k4 = dxdt(&(x_in + k3.clone() * dt),t_in + dt,
                dke_core);

  /* Compute state and time at t+dt */
  x_in + (k1 + 2.0 * k2 + 2.0 * k3 + k4) * dt * (1.0 / 6.0)
}