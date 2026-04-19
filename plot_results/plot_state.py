import plotly.graph_objects as go
from plotly.subplots import make_subplots
import pandas as pd
import numpy as np
from pathlib import Path

from util import read_results


def plot_state(res: pd.DataFrame):
    """
    Plot position, velocity, acceleration in 3x3 grid using Plotly (dark theme)
    """

    fig = make_subplots(
        rows=3,
        cols=3,
        shared_xaxes=True,
        vertical_spacing=0.06,
        horizontal_spacing=0.05,
        subplot_titles=[
            "pos_x",
            "vel_x",
            "acc_x",
            "pos_y",
            "vel_y",
            "acc_y",
            "pos_z",
            "vel_z",
            "acc_z",
        ],
    )

    t = res["sim_time_s"]

    # --- Position ---
    fig.add_trace(go.Scatter(x=t, y=res["pos_x_pci"], mode="lines"), row=1, col=1)
    fig.add_trace(go.Scatter(x=t, y=res["pos_y_pci"], mode="lines"), row=2, col=1)
    fig.add_trace(go.Scatter(x=t, y=res["pos_z_pci"], mode="lines"), row=3, col=1)

    # --- Velocity ---
    fig.add_trace(go.Scatter(x=t, y=res["vel_x_pci"], mode="lines"), row=1, col=2)
    fig.add_trace(go.Scatter(x=t, y=res["vel_y_pci"], mode="lines"), row=2, col=2)
    fig.add_trace(go.Scatter(x=t, y=res["vel_z_pci"], mode="lines"), row=3, col=2)

    # --- Acceleration ---
    fig.add_trace(go.Scatter(x=t, y=res["acc_x_pci"], mode="lines"), row=1, col=3)
    fig.add_trace(go.Scatter(x=t, y=res["acc_y_pci"], mode="lines"), row=2, col=3)
    fig.add_trace(go.Scatter(x=t, y=res["acc_z_pci"], mode="lines"), row=3, col=3)

    # Axis labels (bottom row x labels)
    for col in range(1, 4):
        fig.update_xaxes(title_text="sim_time_s", row=3, col=col)

    # Y labels
    fig.update_yaxes(title_text="pos_x_pci [m]", row=1, col=1)
    fig.update_yaxes(title_text="pos_y_pci [m]", row=2, col=1)
    fig.update_yaxes(title_text="pos_z_pci [m]", row=3, col=1)

    fig.update_yaxes(title_text="vel_x_pci [m/s]", row=1, col=2)
    fig.update_yaxes(title_text="vel_y_pci [m/s]", row=2, col=2)
    fig.update_yaxes(title_text="vel_z_pci [m/s]", row=3, col=2)

    fig.update_yaxes(title_text="acc_x_pci [m/s²]", row=1, col=3)
    fig.update_yaxes(title_text="acc_y_pci [m/s²]", row=2, col=3)
    fig.update_yaxes(title_text="acc_z_pci [m/s²]", row=3, col=3)

    # Dark theme + layout polish
    fig.update_layout(
        template="plotly_dark",
        height=900,
        showlegend=False,
        title="Position, Velocity, Acceleration vs Time",
    )

    # Gridlines (enabled by default, but force consistency)
    fig.update_xaxes(showgrid=True)
    fig.update_yaxes(showgrid=True)

    fig.show()


if __name__ == "__main__":
    res = pd.read_csv("./data_out/out.csv")
    plot_state(res)
