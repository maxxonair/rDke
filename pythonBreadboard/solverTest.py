import numpy as np
import matplotlib.pyplot as plt
import time

from util.rk4Solver import RK4solver 

def getScMass():
  m_kg = 10
  return m_kg

def getInvScMass():
  return 1/getScMass()

def getSumOfForces():
  sumF_N = np.zeros((3,1), dtype=float)
  # Set gravity as only force 
  sumF_N[2] = -9.80665 * getScMass()

  return sumF_N

def buildDerivative(t_in, x_in):
  dxdt = np.zeros_like(x_in)
  Forces = getSumOfForces()
  invMass = getInvScMass()

  dxdt[0] = x_in[3]
  dxdt[1] = x_in[4]
  dxdt[2] = x_in[5]

  # Get Newton to dxdt start
  dxdt[3] = Forces[0] * invMass
  dxdt[4] = Forces[1] * invMass
  dxdt[5] = Forces[2] * invMass

  return dxdt

def main():
  x_start = np.zeros((6,1), dtype=float)
  # Set start position to [0,0,100]
  x_start[2] = 10

  x = x_start
  t_start = 0 
  t_end = 60
  dt_s = 0.01

  t_vec = np.arange(t_start, t_end, dt_s)
  alt_m = np.zeros_like(t_vec)
  vel_m = np.zeros_like(t_vec)
  # Create list to store states for each time step
  stateList = []
  stateList.append(x_start)

  solver = RK4solver()

  start_time = time.time()
  for index, t in enumerate(t_vec):
    (x, t_out) = solver.rk4_step(x,t,dt_s, buildDerivative)
    alt_m[index] = x[2]
    vel_m[index] = x[5]
    # stateList.append(x_start)
    # print(t)
    # print(x)
    # print('')

  exec_time = (time.time()-start_time)
  print(f'Number of steps: {len(t_vec)}')
  print(f'Solver exec time [s]: {exec_time}')
  print(f'Exec time per iteration [ms]: {exec_time/len(t_vec) * 1000}')

  plt.plot(t_vec, alt_m)
  plt.show()

  

if __name__=="__main__":
  main()