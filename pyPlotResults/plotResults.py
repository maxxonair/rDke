


import numpy as np
import pandas as pd 
import matplotlib.pyplot as plt


def plotPosVelAcc(res):
  """
  Plot position and velocity in 3 axes
  """
  f, ax = plt.subplots(3, 3, sharex=True)
  ax[0, 0].plot(res["sim_time_s"],
             res["pos_x_iframe"])
  ax[1, 0].plot(res["sim_time_s"],
            res["pos_y_iframe"])
  ax[2, 0].plot(res["sim_time_s"],
             res["pos_z_iframe"])
  
  ax[0, 1].plot(res["sim_time_s"],
             res["vel_x_iframe"])
  ax[1, 1].plot(res["sim_time_s"],
            res["vel_y_iframe"])
  ax[2, 1].plot(res["sim_time_s"],
             res["vel_z_iframe"])
  
  ax[0, 2].plot(res["sim_time_s"],
             res["acc_x_iframe"])
  ax[1, 2].plot(res["sim_time_s"],
            res["acc_y_iframe"])
  ax[2, 2].plot(res["sim_time_s"],
             res["acc_z_iframe"])
  
  ax[2, 0].set_xlabel("sim_time_s", fontsize=7)
  ax[2, 1].set_xlabel("sim_time_s", fontsize=7)
  ax[2, 2].set_xlabel("sim_time_s", fontsize=7)

  ax[0, 0].set_ylabel("pos_x_iframe [m]", fontsize=7)
  ax[1, 0].set_ylabel("pos_y_iframe [m]", fontsize=7)
  ax[2, 0].set_ylabel("pos_z_iframe [m]", fontsize=7)

  ax[0, 1].set_ylabel("vel_x_iframe [m/s]", fontsize=7)
  ax[1, 1].set_ylabel("vel_y_iframe [m/s]", fontsize=7)
  ax[2, 1].set_ylabel("vel_z_iframe [m/s]", fontsize=7)

  ax[0, 2].set_ylabel("acc_x_iframe [m/ss]", fontsize=7)
  ax[1, 2].set_ylabel("acc_y_iframe [m/ss]", fontsize=7)
  ax[2, 2].set_ylabel("acc_z_iframe [m/ss]", fontsize=7)

  for i in range(3):
    for j in range(3):
      ax[i, j].tick_params(axis='x', labelsize=7)
      ax[i, j].tick_params(axis='y', labelsize=7)
  
  plt.show()

def main():
  res = pd.read_csv("./data_out/out.csv")

  plotPosVelAcc(res)

if __name__=="__main__":
  main()