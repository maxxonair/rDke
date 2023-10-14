

import numpy as np

class RK4solver():


  def __init__(self):
    doNothing = True

  def rk4_step(self, x_in, t_in, dt, buildDerivative):
    """
    General purpose ODE solver using the forth order Runge-Kutta
    method (RK4)


    """
    # Create Vector to store output state n+1
    x_out = np.zeros_like(x_in)

    # Build forth order Runge-Kutta weighted averages
    k1 = buildDerivative(t_in, x_in)
    k2 = buildDerivative(t_in+dt/2, k1*dt/2)
    k3 = buildDerivative(t_in+dt/2, k2*dt/2)
    k4 = buildDerivative(t_in + dt, k3*dt)

    # Compute output state at t+dt
    x_out = x_in + dt/6 * (k1 + k2 + k3 + k4)
    t_out = t_in + dt
    return (x_out, t_out)