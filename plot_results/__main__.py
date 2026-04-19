from rich.console import Console
from rich.table import Table
from rich.prompt import Prompt

import pandas as pd

# Import your plotting functions
from __init__ import *

console = Console()


def main():
    try:
        res = pd.read_csv("./data_out/out.csv")
    except Exception as e:
        console.print(f"[red]Error loading data:[/red] {e}")
        return

    # Menu options
    options = {
        "1": ("Plot Altitude", lambda: plot_altitude(res)),
        "2": ("Plot Density", lambda: plot_density(res)),
        "3": ("Plot Ground Track (Lon/Lat)", lambda: plot_long_lat(res)),
        "4": ("Plot Position/Velocity/Acceleration", lambda: plot_state(res)),
        "q": ("Quit", None),
    }

    # Build table
    table = Table(title="📊 Data Plotting Menu")

    table.add_column("Option", justify="center", style="cyan", no_wrap=True)
    table.add_column("Description", style="magenta")

    for key, (desc, _) in options.items():
        table.add_row(key, desc)

    console.print(table)

    # Prompt user
    while True:
        choice = Prompt.ask(
            "Select an option", choices=list(options.keys()), default="q"
        )

        if choice == "q":
            console.print("[bold green]Exiting...[/bold green]")
            break

        _, func = options[choice]

        try:
            console.print(f"[yellow]Running:[/yellow] {options[choice][0]}")
            func()
        except Exception as e:
            console.print(f"[red]Error while plotting:[/red] {e}")

        console.print()  # spacing


if __name__ == "__main__":
    main()
