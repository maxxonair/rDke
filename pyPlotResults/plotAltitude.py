
import numpy as np
import pandas as pd 
import matplotlib.pyplot as plt
import math


def plotAltitude():
  """
  Plot position and velocity in 3 axes
  """
  res = pd.read_csv("./data_out/out.csv")
  f, ax = plt.subplots(2, 1, sharex=True)

  altitude_km = (np.sqrt(  np.multiply(res["pos_x_iframe"].to_numpy(),res["pos_x_iframe"].to_numpy())
                           + np.multiply(res["pos_y_iframe"].to_numpy(), res["pos_y_iframe"].to_numpy()) 
                           + np.multiply(res["pos_z_iframe"].to_numpy(), res["pos_z_iframe"].to_numpy())) - 6371000) / 1000

  vel_magn_ms = np.sqrt(  np.multiply(res["vel_x_iframe"].to_numpy(),res["vel_x_iframe"].to_numpy())
                           + np.multiply(res["vel_y_iframe"].to_numpy(), res["vel_y_iframe"].to_numpy()) 
                           + np.multiply(res["vel_z_iframe"].to_numpy(), res["vel_z_iframe"].to_numpy()))

  ax[0].plot(res["sim_time_s"].to_numpy(),altitude_km)
  
  ax[0].set_xlabel("sim_time_s", fontsize=7)

  ax[0].set_ylabel("Altitude [km]", fontsize=7)
  ax[0].grid()

  ax[1].plot(res["sim_time_s"].to_numpy(),vel_magn_ms)
  
  ax[1].set_xlabel("sim_time_s", fontsize=7)

  ax[1].set_ylabel("Velocity [m/s]", fontsize=7)
  ax[1].grid()

  plt.show()

if __name__=="__main__":
  plotAltitude()