import pandas as pd
import numpy as np
import plotly.graph_objects as go
from pathlib import Path
from util import read_results


def plot_density(res: pd.DataFrame):
    """
    Plot density vs altitude using Plotly (dark theme)
    """
    altitude_km = res["altitude_pcpf_m"].to_numpy() / 1000
    density = res["atmos_density_kgmmm"].to_numpy()

    fig = go.Figure()

    # Scatter plot
    fig.add_trace(go.Scatter(x=altitude_km, y=density, mode="markers", name="Density"))

    # Axis labels + log scale
    fig.update_xaxes(title_text="Altitude [km]")
    fig.update_yaxes(title_text="Density [kg/m³]", type="log")

    # Dark theme
    fig.update_layout(
        template="plotly_dark",
        title="Atmospheric Density vs Altitude",
        showlegend=False,
    )

    fig.show()


if __name__ == "__main__":
    results = read_results(Path("./data_out/out.csv"))
    plot_density(results)
