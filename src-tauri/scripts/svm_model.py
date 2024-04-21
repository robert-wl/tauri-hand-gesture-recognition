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
DEFAULT_SCALER_NAME = 'scaler.pkl'
DEFAULT_OUTPUT_PATH = 'models'
DEFAULT_CONFUSION_MATRIX_NAME = 'confusion_matrix.png'
DEFAULT_JSON_NAME = 'specification.json'


def make_dir(path):
    path_list = path.split("\\")
    current_path = ""

    if path_list[-1].find('.') != -1:
        path_list = path_list[:-1]

    for path in path_list:
        current_path = os.path.join(current_path, path)
        if not os.path.exists(current_path):
            os.mkdir(current_path)


def preprocess_data(raw_data: DataFrame) -> (ndarray, ndarray, ndarray, MinMaxScaler):
    encoder = LabelEncoder()
    label = encoder.fit_transform(raw_data['Label'])

    data = raw_data.drop(columns=['Image_No', 'Label'])

    scaler = MinMaxScaler()
    np_data = np.array(data).astype('float32')
    np_data = np.where(np_data == 0, ZERO_PAD_CONSTANT, np_data)
    np_data = scaler.fit_transform(np_data)

    data_class = raw_data['Label'].unique()

    # print(f'Data Class: {data_class}')
    return np_data, label, data_class, scaler


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

    plot_heatmap(cm, data_class, model_name)

    report: dict = classification_report(label_test, data_prediction, target_names=data_class, output_dict=True)

    # print(classification_report(label_test, data_prediction, target_names=data_class))
    report_accuracy = report.pop('accuracy')
    report_macro_avg = report.pop('macro avg')
    report_weighted_avg = report.pop('weighted avg')


    output_json = {
        'accuracy': accuracy,
        'precision': precision,
        'recall': recall,
        'f1': f1,
        'confusion_matrix': cm.tolist(),
        'classification_report': {
            'class': report,
            'accuracy': report_accuracy,
            'macro_avg': report_macro_avg,
            'weighted_avg': report_weighted_avg
        }
    }

    return output_json


def save_model_specifications(model: svm.SVC, dataset_name: str, model_name: str, data_test: ndarray, label_test: ndarray, data_class: ndarray) -> None:
    output_json = evaluate_model(model, data_test, label_test, data_class)

    params = model.get_params()

    output_json['hyperparameters'] = {key: str(value) for key, value in params.items()}
    output_json['dataset_name'] = dataset_name
    output_json['name'] = model_name
    output_json['data_class'] = data_class.tolist()

    save_json(output_json, model_name)
    pass


def save_model(model: svm.SVC, model_name: str, output_path: str = DEFAULT_OUTPUT_PATH,
               file_name: str = DEFAULT_MODEL_NAME) -> None:
    path = f'{output_path}\\{model_name}\\{file_name}'
    make_dir(path)
    with open(path, 'wb') as file:
        pickle.dump(model, file)


def save_json(json_data: dict, model_name: str, output_path: str = DEFAULT_OUTPUT_PATH, file_name: str = DEFAULT_JSON_NAME) -> None:
    path = f'{output_path}\\{model_name}\\{file_name}'
    make_dir(path)
    with open(path, 'w') as file:
        json.dump(json_data, file)


def plot_heatmap(cm: ndarray, class_names: ndarray, model_name: str, output_path: str = DEFAULT_OUTPUT_PATH, file_name: str = DEFAULT_CONFUSION_MATRIX_NAME) -> None:
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

    path = f'{output_path}\\{model_name}\\{file_name}'
    make_dir(path)
    plt.savefig(path)


def train_model(path: str, model_name: str, kernel: str, c: float, gamma: str | float, degree: int) -> None:
    data = pd.read_csv(path)
    data, label, data_class, scaler = preprocess_data(data)

    data_train, data_test, label_train, label_test = split_data(data, label)

    model = svm.SVC(kernel=kernel, verbose=True, C=c, gamma=gamma, degree=degree)

    model.fit(data_train, label_train)

    dataset_name = path.split('\\')[-2]

    save_model_specifications(model, dataset_name, model_name, data_test, label_test, data_class)
    save_model(model, model_name, file_name=DEFAULT_MODEL_NAME)
    save_model(scaler, model_name, file_name=DEFAULT_SCALER_NAME)



if __name__ == '__main__':
    dataset_path = sys.argv[1]
    model_name = sys.argv[2]
    kernel = sys.argv[3]
    c = sys.argv[4]
    gamma = sys.argv[5]
    degree = sys.argv[6]

    degree = int(degree) if kernel == 'poly' else 3
    gamma = 'scale' if gamma == 'scale' else 'auto' if gamma == 'auto' else float(gamma)
    c = float(c)

    train_model(dataset_path, model_name, kernel, c, gamma, degree)
