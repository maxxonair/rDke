
import numpy as np
import pandas as pd 
import matplotlib.pyplot as plt
import math


def plotAltitude():
  """
  Plot position and velocity in 3 axes
  """
  res = pd.read_csv("./data_out/out.csv")
  f, ax = plt.subplots(1, 1, sharex=False)

  # vel_magn_ms = np.sqrt(  np.multiply(res["vel_x_pci"].to_numpy(),res["vel_x_pci"].to_numpy())
  #                          + np.multiply(res["vel_y_pci"].to_numpy(), res["vel_y_pci"].to_numpy()) 
  #                          + np.multiply(res["vel_z_pci"].to_numpy(), res["vel_z_pci"].to_numpy()))
  
  # Load background map from file 
  img = plt.imread("assets/images/earth_map.jpg")
  #-------------
  ax.imshow(img, extent=[-180, 180, -90, 90])
  ax.scatter(res["longitude_pcpf_deg"],
             res["latitude_pcpf_deg"],
             marker='.',
             s=1,
             c='b')
  ax.set_xlabel("Longitude [deg]", fontsize=7)

  ax.set_ylabel("Latitude [deg]", fontsize=7)
  ax.grid()
  #-------------


  plt.show()

if __name__=="__main__":
  plotAltitude()