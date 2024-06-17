import cv2
import numpy as np
import os
import pandas as pd
import sys
from mediapipe.python.solutions import drawing_utils, hands
from mediapipe.python.solutions.drawing_utils import DrawingSpec
from numpy import ndarray


def get_image(image_path: str) -> ndarray:
    image = cv2.imread(image_path)
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    return image


def write_image(image: ndarray, output_path: str) -> None:
    make_dir(output_path)
    image = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)
    cv2.imwrite(output_path, image)


def annotate_image(image: ndarray, landmarks: list) -> ndarray:
    annotated_image = image.copy()
    for landmark in landmarks:
        drawing_utils.draw_landmarks(
            annotated_image,
            landmark,
            hands.HAND_CONNECTIONS,
            DrawingSpec(color=(255, 0, 0), thickness=10, circle_radius=10),
            DrawingSpec(color=(0, 255, 0), thickness=5, circle_radius=2),
        )
    return annotated_image

def make_dir(path: str) -> None:
    path_list = path.split("\\")
    current_path = ""

    if path_list[-1].find('.') != -1:
        path_list = path_list[:-1]

    for path in path_list:
        current_path = os.path.join(current_path, path)
        if not os.path.exists(current_path):
            os.mkdir(current_path)


class MediaPipeConverter:
    def __init__(self):
        self.hands = hands.Hands(
            static_image_mode=False,
            max_num_hands=2,
            min_detection_confidence=0.5,
            min_tracking_confidence=0.5
        )
        self.landmark_data = []

    def get_landmarks(self, image: ndarray) -> list:
        results = self.hands.process(image)

        if not results.multi_hand_landmarks:
            return []

        return results.multi_hand_landmarks

    def process_directory(self, label: str, input_dir: str, output_dir: str) -> None:
        for filename in os.listdir(input_dir):
            image_path = os.path.join(input_dir, filename)
            image = get_image(image_path)

            landmarks = self.get_landmarks(image)
            self.convert_to_dict(filename, label, landmarks)

            annotated_image = annotate_image(image, landmarks)

            output_path = os.path.join(output_dir, filename)
            write_image(annotated_image, output_path)

            print("Finished Processing", flush=True)

    def convert_to_dict(self, name: str, label: str, landmarks: str) -> None:
        if landmarks is None:
            landmarks = []

        if len(landmarks) > 0:
            first_hand = np.array([landmark for landmark in landmarks[0].landmark]).flatten()
        else:
            first_hand = np.zeros(21)

        if len(landmarks) > 1:
            second_hand = np.array([landmark for landmark in landmarks[1].landmark]).flatten()
        else:
            second_hand = np.zeros(21)

        for i in range(2):
            hand_landmarks = []
            if i == 0:
                hand_landmarks.extend(first_hand)
                hand_landmarks.extend(second_hand)
            else:
                hand_landmarks.extend(second_hand)
                hand_landmarks.extend(first_hand)

            hand_data = {
                "Image_No": name,
                "Label": label
            }

            rel_coords = np.zeros(3)

            for j, landmark in enumerate(hand_landmarks):
                if landmark is None:
                    for dim in ['x', 'y', 'z']:
                        hand_data[f"Landmark_{j + 1}_{dim}"] = 0
                    continue

                if j % 21 == 0:
                    for k, dim in enumerate(['x', 'y', 'z']):
                        rel_coords[k] = getattr(hand_landmarks[j], dim, 0)

                for k, dim in enumerate(['x', 'y', 'z']):
                    hand_data[f"Landmark_{j + 1}_{dim}"] = getattr(landmark, dim, 0) - rel_coords[k]

            self.landmark_data.append(hand_data)

    def dump_to_csv(self, output_path: str) -> None:
        df = pd.DataFrame(self.landmark_data)

        make_dir(output_path)
        if os.path.exists(output_path):
            old_df = pd.read_csv(output_path)
            df = pd.concat([old_df, df], ignore_index=True)

        df.to_csv(output_path, index=False)




if __name__ == '__main__':
    input_dir = sys.argv[1]
    output_dir = sys.argv[2]
    output_csv = sys.argv[3]

    label = input_dir.split("\\")[-1]

    converter = MediaPipeConverter()
    converter.process_directory(label, input_dir, output_dir)
    converter.dump_to_csv(output_csv)


