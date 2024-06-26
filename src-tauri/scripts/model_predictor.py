import cv2
import json
import numpy as np
import os
import pickle
import sys
from mediapipe.python.solutions import drawing_utils, hands
from mediapipe.python.solutions.drawing_utils import DrawingSpec
from numpy import ndarray
from sklearn import svm
from sklearn.linear_model import LogisticRegression
from sklearn.neighbors import KNeighborsClassifier
from sklearn.preprocessing import MinMaxScaler

DEFAULT_MODEL_NAME = "model.pkl"
DEFAULT_SCALER_NAME = "scaler.pkl"
DEFAULT_JSON_NAME = "specification.json"
DEFAULT_TEMP_DIR = "temp"
DEFAULT_INPUT_IMAGE = "input.png"
DEFAULT_OUTPUT_IMAGE = "output.png"


class MediapipeConverter:
    def __init__(self):
        self.hands = hands.Hands(
            static_image_mode=True,
            max_num_hands=2,
            min_detection_confidence=0.5,
            min_tracking_confidence=0.5,
        )
        self.hand_point_style = DrawingSpec(
            color=(255, 0, 0), thickness=10, circle_radius=10
        )
        self.hand_line_style = DrawingSpec(
            color=(0, 255, 0), thickness=5, circle_radius=2
        )

    def annotate_image(self, image: ndarray, landmarks: list) -> ndarray:
        annotated_image = image.copy()
        for landmark in landmarks:
            drawing_utils.draw_landmarks(
                annotated_image,
                landmark,
                hands.HAND_CONNECTIONS,
                self.hand_point_style,
                self.hand_line_style,
            )
        return annotated_image

    def get_landmarks(self, image: ndarray) -> list:
        results = self.hands.process(image)

        if not results.multi_hand_landmarks:
            return []

        return results.multi_hand_landmarks

    def convert_to_arr(self, landmarks: list) -> ndarray:
        if landmarks is None:
            return np.zeros(21 * 2)

        hand_data = []

        if len(landmarks) > 0:
            first_hand = np.array([landmark for landmark in landmarks[0].landmark]).flatten()
        else:
            first_hand = np.zeros(21)

        if len(landmarks) > 1:
            second_hand = np.array([landmark for landmark in landmarks[1].landmark]).flatten()
        else:
            second_hand = np.zeros(21)

        hand_landmarks = np.concatenate([first_hand, second_hand])

        rel_coords = np.zeros(3)

        for i, landmark in enumerate(hand_landmarks):
            if landmark is None:
                hand_data.extend([0, 0, 0])
                continue

            if i % 21 == 0:
                for k, dim in enumerate(["x", "y", "z"]):
                    rel_coords[k] = getattr(hand_landmarks[i], dim, 0)

            for k, dim in enumerate(["x", "y", "z"]):
                hand_data.append(getattr(hand_landmarks[i], dim, 0) - rel_coords[k])

        return np.array(hand_data)

    def extract(self, image):
        landmarks = self.get_landmarks(image)
        landmark_arr = self.convert_to_arr(landmarks)
        annotated_image = self.annotate_image(image, landmarks)

        return landmark_arr, annotated_image


def read_model(path: str, file_name: str) -> svm.SVC | LogisticRegression | KNeighborsClassifier | MinMaxScaler:
    model_path = os.path.join(path, file_name)
    with open(model_path, "rb") as file:
        model = pickle.load(file)
        return model


def read_json(path: str) -> dict:
    json_path = os.path.join(path, DEFAULT_JSON_NAME)
    with open(json_path, "r") as file:
        return json.load(file)


def read_image() -> ndarray:
    path = os.path.join(DEFAULT_TEMP_DIR, DEFAULT_INPUT_IMAGE)
    image = cv2.imread(path)
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    return image


def write_image(image: ndarray):
    path = os.path.join(DEFAULT_TEMP_DIR, DEFAULT_OUTPUT_IMAGE)
    image = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)
    cv2.imwrite(path, image)


def predict(dir: str):
    model = read_model(dir, DEFAULT_MODEL_NAME)
    scaler = read_model(dir, DEFAULT_SCALER_NAME)
    image = read_image()

    json = read_json(dir)

    landmark, image = MediapipeConverter().extract(image)
    classes = json["data_class"]

    data = scaler.transform([landmark])

    prediction = model.predict(data)

    write_image(image)
    print(classes[prediction[0]])


if __name__ == "__main__":
    model_dir = sys.argv[1]

    predict(model_dir)
