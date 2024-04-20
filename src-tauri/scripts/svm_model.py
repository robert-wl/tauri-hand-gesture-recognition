import json
import matplotlib.pyplot as plt
import numpy as np
import os
import pandas as pd
import pickle
import seaborn as sns
import sys
from numpy import ndarray
from pandas import DataFrame
from sklearn import svm
from sklearn.metrics import accuracy_score, precision_score, recall_score, f1_score, confusion_matrix, \
    classification_report
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import LabelEncoder, MinMaxScaler

ZERO_PAD_CONSTANT = 10e-6
DEFAULT_MODEL_NAME = 'model.pkl'
DEFAULT_OUTPUT_PATH = 'models'
DEFAULT_CONFUSION_MATRIX_NAME = 'confusion_matrix.png'
DEFAULT_JSON_NAME = 'output.json'


def make_dir(path):
    path_list = path.split("\\")
    current_path = ""

    if path_list[-1].contains("."):
        path_list = path_list[:-1]

    for path in path_list:
        current_path = os.path.join(current_path, path)
        if not os.path.exists(current_path):
            os.mkdir(current_path)


def preprocess_data(raw_data: DataFrame) -> (ndarray, ndarray, ndarray):
    encoder = LabelEncoder()
    label = encoder.fit_transform(raw_data['Label'])

    data = raw_data.drop(columns=['Image_No', 'Label'])

    scaler = MinMaxScaler()
    np_data = np.array(data).astype('float32')
    np_data = np.where(np_data == 0, ZERO_PAD_CONSTANT, np_data)
    np_data = scaler.fit_transform(np_data)

    data_class = raw_data['Label'].unique()

    print(f'Data Class: {data_class}')
    return np_data, label, data_class


def split_data(data: ndarray, label: ndarray) -> (ndarray, ndarray, ndarray, ndarray):
    x_train, x_test, y_train, y_test = train_test_split(data, label, test_size=0.3, random_state=42)
    return x_train, x_test, y_train, y_test


def evaluate_model(model: svm.SVC, data_test: ndarray, label_test: ndarray, data_class: ndarray) -> dict:
    data_prediction = model.predict(data_test)

    accuracy = accuracy_score(label_test, data_prediction)
    precision = precision_score(label_test, data_prediction, average='weighted')
    recall = recall_score(label_test, data_prediction, average='weighted')
    f1 = f1_score(label_test, data_prediction, average='weighted')
    cm = confusion_matrix(label_test, data_prediction)

    plot_heatmap(cm, data_class)

    report = classification_report(label_test, data_prediction, target_names=data_class)

    output_json = {
        'accuracy': accuracy,
        'precision': precision,
        'recall': recall,
        'f1': f1,
        'confusion_matrix': cm.tolist(),
        'classification_report': report
    }

    return output_json


def save_model(model: svm.SVC, model_name: str, output_path: str = DEFAULT_OUTPUT_PATH,
               file_name: str = DEFAULT_MODEL_NAME) -> None:
    with open(f'{output_path}/{model_name}/{file_name}', 'wb') as file:
        pickle.dump(model, file)


def save_json(json_data: dict, model_name: str, output_path: str = DEFAULT_OUTPUT_PATH, file_name: str = DEFAULT_JSON_NAME) -> None:
    with open(f'{output_path}/{model_name}/{file_name}', 'w') as file:
        json.dump(json_data, file)


def plot_heatmap(cm: ndarray, class_names: ndarray, output_path: str = DEFAULT_CONFUSION_MATRIX_NAME) -> None:
    df_cm = pd.DataFrame(cm)
    plt.figure(figsize=(10, 7))
    sns.heatmap(
        df_cm,
        annot=True,
        cmap='viridis',
        xticklabels=class_names,
        yticklabels=class_names
    )
    plt.xlabel('Prediction')
    plt.ylabel('Actual')
    plt.savefig(output_path)


def train_model(path: str, model_name: str, kernel: str) -> None:
    data = pd.read_csv(path)
    data, label, data_class = preprocess_data(data)

    data_train, data_test, label_train, label_test = split_data(data, label)

    model = svm.SVC(kernel=kernel)

    model.fit(data_train, label_train)

    output_json = evaluate_model(model, data_test, label_test, data_class)

    save_json(output_json, model_name)
    save_model(model, model_name)


if __name__ == '__main__':
    dataset_path = sys.argv[1]
    model_name = sys.argv[2]
    kernel = sys.argv[3]

    train_model(dataset_path, model_name, kernel)
