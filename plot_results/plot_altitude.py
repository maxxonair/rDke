import pandas as pd
import numpy as np
import plotly.graph_objects as go
from plotly.subplots import make_subplots
from pathlib import Path


from util import read_results


def plot_altitude(res: pd.DataFrame):
    """
    Plot position and velocity in 3 axes using Plotly (dark theme)
    """

    # Compute velocity magnitude
    vel_magn_ms = np.sqrt(
        res["vel_x_pci"].to_numpy() ** 2
        + res["vel_y_pci"].to_numpy() ** 2
        + res["vel_z_pci"].to_numpy() ** 2
    )

    altitude_km = res["altitude_pcpf_m"].to_numpy() / 1000

    # Create subplots
    fig = make_subplots(
        rows=3,
        cols=1,
        subplot_titles=[
            "Altitude vs Time",
            "Altitude vs Velocity",
            "Gravity vs Altitude",
        ],
        vertical_spacing=0.08,
    )

    # --- Plot 1: Altitude vs Time ---
    fig.add_trace(
        go.Scatter(x=res["sim_time_s"], y=altitude_km, mode="lines", name="Altitude"),
        row=1,
        col=1,
    )

    # --- Plot 2: Altitude vs Velocity ---
    fig.add_trace(
        go.Scatter(
            x=vel_magn_ms, y=altitude_km, mode="lines", name="Velocity vs Altitude"
        ),
        row=2,
        col=1,
    )

    # --- Plot 3: Gravity vs Altitude ---
    fig.add_trace(
        go.Scatter(
            x=altitude_km, y=res["magn_grav_acc_mss"], mode="lines", name="Gravity"
        ),
        row=3,
        col=1,
    )

    # Axis labels
    fig.update_xaxes(title_text="sim_time_s", row=1, col=1)
    fig.update_yaxes(title_text="Altitude [km]", row=1, col=1)

    fig.update_xaxes(title_text="Velocity [m/s]", row=2, col=1)
    fig.update_yaxes(title_text="Altitude [km]", row=2, col=1)

    fig.update_xaxes(title_text="Altitude [km]", row=3, col=1)
    fig.update_yaxes(title_text="Grav. Acceleration [m/s²]", row=3, col=1)

    # Dark theme styling
    fig.update_layout(
        template="plotly_dark",
        height=900,
        showlegend=False,
        title="Flight Profile Charts: Altitude",
    )

    fig.show()


if __name__ == "__main__":
    results = read_results(Path("./data_out/out.csv"))
    plot_altitude(results)
