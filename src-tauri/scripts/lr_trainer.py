import json
import matplotlib.pyplot as plt
import numpy as np
import os
import pandas as pd
import pickle
import seaborn as sns
import sys
from numpy import ndarray
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import (
    accuracy_score,
    precision_score,
    recall_score,
    f1_score,
    confusion_matrix,
    classification_report,
)
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import LabelEncoder, MinMaxScaler

ZERO_PAD_CONSTANT = 10e-6
DEFAULT_MODEL_NAME = "model.pkl"
DEFAULT_SCALER_NAME = "scaler.pkl"
DEFAULT_OUTPUT_PATH = "model"
DEFAULT_CONFUSION_MATRIX_NAME = "confusion_matrix.png"
DEFAULT_JSON_NAME = "specification.json"
LR_PENALTY = ["l2", "l1", "elasticnet", "none"]
LR_SOLVER = ["lbfgs", "liblinear", "newton-cg", "newton-cholesky", "sag", "saga"]

logs = []

def make_dir(path: str) -> None:
    path_list = path.split("\\")
    current_path = ""

    if path_list[-1].find(".") != -1:
        path_list = path_list[:-1]

    for path in path_list:
        current_path = os.path.join(current_path, path)
        if not os.path.exists(current_path):
            os.mkdir(current_path)


def save_model(
        model: LogisticRegression | MinMaxScaler,
        model_name: str,
        output_path: str = DEFAULT_OUTPUT_PATH,
        file_name: str = DEFAULT_MODEL_NAME,
) -> None:
    path = os.path.join(output_path, model_name, file_name)
    make_dir(path)
    with open(path, "wb") as file:
        pickle.dump(model, file)


def save_json(
        json_data: dict,
        model_name: str,
        output_path: str = DEFAULT_OUTPUT_PATH,
        file_name: str = DEFAULT_JSON_NAME,
) -> None:
    path = os.path.join(output_path, model_name, file_name)
    make_dir(path)
    with open(path, "w") as file:
        json.dump(json_data, file)


def plot_heatmap(
        cm: ndarray,
        class_names: ndarray,
        model_name: str,
        output_path: str = DEFAULT_OUTPUT_PATH,
        file_name: str = DEFAULT_CONFUSION_MATRIX_NAME,
) -> None:
    df_cm = pd.DataFrame(cm)
    plt.figure(figsize=(10, 7))
    sns.heatmap(
        df_cm,
        annot=True,
        cmap="viridis",
        xticklabels=class_names,
        yticklabels=class_names,
    )
    plt.xlabel("Prediction")
    plt.ylabel("Actual")

    path = os.path.join(output_path, model_name, file_name)
    make_dir(path)
    plt.savefig(path)


class LogisticRegressionModel:

    def __init__(self, model_name: str, dataset_name: str, penalty: str, c: float, solver: str, max_iter: int):
        self.model_name = model_name
        self.dataset_name = dataset_name
        self.model = LogisticRegression(penalty=penalty, C=c, solver=solver, max_iter=max_iter)
        self.encoder = LabelEncoder()
        self.scaler = MinMaxScaler()
        self.specifications = {}
        self.np_data = None
        self.label = None
        self.data_class = None

    def preprocess(self, path: str) -> None:
        raw_data = pd.read_csv(path)

        label = raw_data["Label"]
        label = self.encoder.fit_transform(label)

        data_class = raw_data["Label"].unique()

        data = raw_data.drop(columns=["Image_No", "Label"])

        np_data = np.array(data).astype("float32")
        np_data = np.where(np_data == 0, ZERO_PAD_CONSTANT, np_data)
        np_data = self.scaler.fit_transform(np_data)

        self.np_data = np_data
        self.label = label
        self.data_class = data_class


    def train_evaluate(self) -> None:
        data_train, data_test, label_train, label_test = train_test_split(
            self.np_data, self.label, test_size=0.3, random_state=42
        )

        self.model.fit(data_train, label_train)
        data_pred = self.model.predict(data_test)

        params = self.model.get_params()
        accuracy = accuracy_score(label_test, data_pred)
        precision = precision_score(label_test, data_pred, average="weighted")
        recall = recall_score(label_test, data_pred, average="weighted")
        f1 = f1_score(label_test, data_pred, average="weighted")
        cm = confusion_matrix(label_test, data_pred)
        report = classification_report(label_test, data_pred, target_names=self.data_class, output_dict=True)

        report_accuracy = report.pop("accuracy")
        report_macro_avg = report.pop("macro avg")
        report_weighted_avg = report.pop("weighted avg")

        hyperparameters = {key: str(value) for key, value in params.items()}

        self.specifications = {
            "algorithm": "Logistic Regression",
            "accuracy": accuracy,
            "precision": precision,
            "recall": recall,
            "f1": f1,
            "confusion_matrix": cm.tolist(),
            "classification_report": {
                "class": report,
                "accuracy": report_accuracy,
                "macro_avg": report_macro_avg,
                "weighted_avg": report_weighted_avg,
            },
            'hyperparameters': {
                'Lr': hyperparameters
            },
            'dataset_name': self.dataset_name,
            'name': model_name,
            'data_class': self.data_class.tolist()
        }


    def save(self) -> None:
        save_model(self.model, self.model_name, file_name=DEFAULT_MODEL_NAME)
        save_model(self.scaler, self.model_name, file_name=DEFAULT_SCALER_NAME)
        save_json(self.specifications, self.model_name)
        plot_heatmap(
            np.array(self.specifications["confusion_matrix"]),
            np.array(self.data_class),
            self.model_name
        )


if __name__ == "__main__":
    dataset_path = sys.argv[1]
    model_name = sys.argv[2]
    penalty = sys.argv[3]
    c = sys.argv[4]
    solver = sys.argv[5]
    max_iter = sys.argv[6]


    penalty = penalty if penalty in LR_PENALTY else "l2"
    c = float(c) if c != "None" else None
    solver = solver if solver in LR_SOLVER else "lbfgs"
    max_iter = int(max_iter) if max_iter != "None" else 100


    dataset_name = dataset_path.split("\\")[-2]

    model = LogisticRegressionModel(model_name, dataset_name, penalty, c, solver, max_iter)
    model.preprocess(dataset_path)
    model.train_evaluate()
    model.save()
