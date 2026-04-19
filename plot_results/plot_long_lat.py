import numpy as np
import pandas as pd
import plotly.graph_objects as go
import math
import base64
from pathlib import Path

from util import read_results


def plot_long_lat():
    """
    Plot longitude vs latitude over a background map using Plotly (dark theme)
    """
    res = pd.read_csv("./data_out/out.csv")

    # Convert image to base64 so Plotly can display it
    with open("assets/images/earth_map.jpg", "rb") as f:
        encoded_image = base64.b64encode(f.read()).decode()

    fig = go.Figure()

    # Add background image
    fig.add_layout_image(
        dict(
            source=f"data:image/jpg;base64,{encoded_image}",
            xref="x",
            yref="y",
            x=-180,
            y=90,
            sizex=360,
            sizey=180,
            sizing="stretch",
            opacity=1.0,
            layer="below",
        )
    )

    # Scatter plot (trajectory)
    fig.add_trace(
        go.Scatter(
            x=res["longitude_pcpf_deg"],
            y=res["latitude_pcpf_deg"],
            mode="markers",
            marker=dict(size=3),
            name="Trajectory",
        )
    )

    # Axis formatting
    fig.update_xaxes(title_text="Longitude [deg]", range=[-180, 180], showgrid=True)

    fig.update_yaxes(
        title_text="Latitude [deg]",
        range=[-90, 90],
        showgrid=True,
        scaleanchor="x",  # keeps aspect ratio correct
        scaleratio=1,
    )

    # Dark theme layout
    fig.update_layout(
        template="plotly_dark",
        title="Ground Track",
        showlegend=False,
        margin=dict(l=40, r=40, t=40, b=40),
    )

    fig.show()


if __name__ == "__main__":
    results = read_results(Path("./data_out/out.csv"))
    plot_long_lat(results)
