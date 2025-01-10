import pandas as pd
import numpy as np
from fair_ml import (
    data_bias,
    model_bias
)

def get_data() -> pd.DataFrame:
    headers = [
        "sex",
        "length",
        "diameter",
        "height",
        "whole_weight",
        "shucked_weight",
        "viscera_weight",
        "shell_weight",
        "rings",
    ]
    """
    Using abalone dataset to test
    """
    return pd.read_csv("abalone.data", names=headers)

def generate_synthetic_scores(row: int) -> float:
    op = 1 if np.random.random() > 0.5 else -1
    return row + (2 * np.random.random() * op)

def test_db_numpy(bl_df, runtime_test) -> None:
    """
    testing using a numpy array with the db methods
    """
    db_bl = data_bias.perform_analysis(
        bl_df["sex"].to_numpy(),
        bl_df["rings"].to_numpy(),
        "M",
        15
    )

    db_runtime = data_bias.perform_analysis(
        runtime_test["sex"].to_numpy(),
        runtime_test["rings"].to_numpy(),
        "M",
        15
    )

    runtime_check = data_bias.runtime_comparison(
        db_bl,
        db_runtime,
        0.15
    )

    print(f"baseline:\n{db_bl}")
    print("\n")
    print(f"runtime:\n{db_runtime}")
    print("\n")
    print(f"runtime check:\n{runtime_check}")


def test_db_list(bl_df, runtime_test) -> None:
    """
    testing using a numpy array with the db methods
    """
    db_bl = data_bias.perform_analysis(
        bl_df["sex"].to_list(),
        bl_df["rings"].to_list(),
        "M",
        15
    )

    db_runtime = data_bias.perform_analysis(
        runtime_test["sex"].to_list(),
        runtime_test["rings"].to_list(),
        "M",
        15
    )

    runtime_check = data_bias.runtime_comparison(
        db_bl,
        db_runtime
    )

    print(f"baseline\n{db_bl}")
    print("\n")
    print(f"runtime\n:{db_runtime}")
    print("\n")
    print(f"runtime check:{runtime_check}")

def test_mb_numpy(bl_df, runtime_test):
    bl = model_bias.perform_analysis(
        bl_df["sex"].to_numpy(),
        bl_df["rings"].to_numpy(),
        bl_df["preds"].to_numpy(),
        "M",
        15,
        15.0
    )

    runtime = model_bias.perform_analysis(
        runtime_test["sex"].to_numpy(),
        runtime_test["rings"].to_numpy(),
        runtime_test["preds"].to_numpy(),
        "M",
        15,
        15.0
    )

    runtime_check = model_bias.runtime_comparison(
        bl,
        runtime,
        0.15
    )

    print(f"bl:\n{bl}" )
    print("\n")
    print(f"runtime:\n{runtime}")
    print("\n")
    print(f"check:\n{runtime_check}")


def test_mb_list(bl_df, runtime_test):
    bl = model_bias.perform_analysis(
        bl_df["sex"].to_list(),
        bl_df["rings"].to_list(),
        bl_df["preds"].to_list(),
        "M",
        15,
        15.0
    )

    runtime = model_bias.perform_analysis(
        runtime_test["sex"].to_list(),
        runtime_test["rings"].to_list(),
        runtime_test["preds"].to_list(),
        "M",
        15,
        15.0
    )

    runtime_check = model_bias.runtime_comparison(
        bl,
        runtime,
        0.15
    )

    print(f"bl:\n{bl}" )
    print("\n")
    print(f"runtime:\n{runtime}")
    print("\n")
    print(f"check:\n{runtime_check}")

if __name__ == "__main__":
    df = get_data()
    df["preds"] = df.rings.apply(
        generate_synthetic_scores
    )

    bl_df = df.iloc[:int(.60 * df.shape[0]), :]
    runtime_test = df.iloc[int(.60 * df.shape[0]) + 1:, :]
    print("TESTING DATA BIAS WITH NUMPY ARRAYS...")
    test_db_numpy(bl_df, runtime_test)
    print("\n")

    print("TESTING DATA BIAS WITH LIST ARRAYS...")
    test_db_list(bl_df, runtime_test)

    print("\n")
    print("TESTING MB with numpy...")
    test_mb_numpy(bl_df, runtime_test)

    print("\n")
    print("TESTING MB with lists...")
    test_mb_list(bl_df, runtime_test)
