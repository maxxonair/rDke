
import numpy as np
import pandas as pd 
import matplotlib.pyplot as plt
import math


def plotAltitude():
  """
  Plot position and velocity in 3 axes
  """
  res = pd.read_csv("./data_out/out.csv")
  f, ax = plt.subplots(3, 1, sharex=False)

  vel_magn_ms = np.sqrt(  np.multiply(res["vel_x_iframe"].to_numpy(),res["vel_x_iframe"].to_numpy())
                           + np.multiply(res["vel_y_iframe"].to_numpy(), res["vel_y_iframe"].to_numpy()) 
                           + np.multiply(res["vel_z_iframe"].to_numpy(), res["vel_z_iframe"].to_numpy()))
  #-------------
  ax[0].plot((res["longitude_pcpf_deg"]),vel_magn_ms)
  
  ax[0].set_ylabel("Velocity [m/s] ", fontsize=7)

  ax[0].set_xlabel("Longitude [deg]", fontsize=7)
  ax[0].grid()
  #-------------
  ax[1].plot((res["longitude_pcpf_deg"].to_numpy()), (res["latitude_pcpf_deg"].to_numpy()))
  
  ax[1].set_xlabel("Longitude [deg]", fontsize=7)

  ax[1].set_ylabel("Latitude [deg]", fontsize=7)
  ax[1].grid()
  #-------------
  ax[2].plot((res["longitude_pcpf_deg"].to_numpy()),res["altitude_pcpf_m"].to_numpy()/1000)
  
  ax[2].set_ylabel("Altitude [km]", fontsize=7)

  ax[2].set_xlabel("Longitude [deg]", fontsize=7)
  ax[2].grid()

  plt.show()

if __name__=="__main__":
  plotAltitude()