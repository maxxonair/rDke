import pandas as pd
from pathlib import Path


def read_results(res_path: Path):
    """
    Read results from CSV file and return as a pandas DataFrame.
    """
    res = pd.read_csv(res_path)
    return res
