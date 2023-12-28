
import numpy as np
import pandas as pd 
import matplotlib.pyplot as plt
import math


def plotDensity():
  """
  Plot position and velocity in 3 axes
  """
  res = pd.read_csv("./data_out/out.csv")
  f, ax = plt.subplots(1, 1, sharex=False)

  #-------------
  ax.scatter(res["altitude_pcpf_m"].to_numpy()/1000,res["atmos_density_kgmmm"].to_numpy())
  
  ax.set_xlabel("altitude [km]", fontsize=7)

  ax.set_ylabel("density [kg/m3]", fontsize=7)
  ax.grid()
  
  plt.yscale("log")

  plt.show()

if __name__=="__main__":
  plotDensity()