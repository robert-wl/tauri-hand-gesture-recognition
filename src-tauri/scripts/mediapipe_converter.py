import cv2
import numpy as np
import os
import pandas as pd
import sys
from mediapipe.python.solutions import drawing_utils, drawing_styles, hands


def get_image(image_path) -> np.ndarray:
    image = cv2.imread(image_path)
    image = cv2.flip(image, 1)
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    return image


def write_image(image, output_path):
    make_dir(output_path)
    image = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)
    cv2.imwrite(output_path, image)


def annotate_image(image, landmarks):
    annotated_image = image.copy()
    for landmark in landmarks:
        drawing_utils.draw_landmarks(
            annotated_image,
            landmark,
            hands.HAND_CONNECTIONS,
            drawing_styles.get_default_hand_landmarks_style(),
            drawing_styles.get_default_hand_connections_style()
        )
    return annotated_image

def make_dir(path):
    path_list = path.split("\\")
    current_path = ""

    for path in path_list[:-1]:
        current_path = os.path.join(current_path, path)
        if not os.path.exists(current_path):
            os.mkdir(current_path)


class MediaPipeConverter:
    def __init__(self, label):
        self.hands = hands.Hands(
            static_image_mode=False,
            max_num_hands=2,
            min_detection_confidence=0.5,
            min_tracking_confidence=0.5
        )
        self.label = label
        self.landmark_data = []

    def get_landmarks(self, image):
        results = self.hands.process(image)

        if not results.multi_hand_landmarks:
            return None

        return results.multi_hand_landmarks

    def process_from_directory(self, input_dir, output_dir):
        for filename in os.listdir(input_dir):
            image_path = os.path.join(input_dir, filename)
            image = get_image(image_path)
            landmarks = self.get_landmarks(image)
            self.convert_to_dict(filename, landmarks)
            if landmarks is not None:
                annotated_image = annotate_image(image, landmarks)
                output_path = os.path.join(output_dir, filename)
                write_image(annotated_image, output_path)
            print("Finished Processing", flush=True)

    def convert_to_dict(self, name, landmarks):
        if landmarks is None:
            landmarks = []

        hand_data = {
            "Image_No": name,
            "Label": self.label
        }

        hand_landmarks = [landmark for landmark in (landmarks[i].landmark for i in range(2) if len(landmarks) > i)]
        hand_landmarks = np.array(hand_landmarks).flatten()
        hand_landmarks = np.pad(hand_landmarks, (0, 42 - len(hand_landmarks)), mode='constant', constant_values=(None, None))

        relative_x = 0
        relative_y = 0
        relative_z = 0

        for i, landmark in enumerate(hand_landmarks):
            if landmark is None:
                hand_data[f"Landmark_{i + 1}_X"] = 0
                hand_data[f"Landmark_{i + 1}_Y"] = 0
                hand_data[f"Landmark_{i + 1}_Z"] = 0
                continue

            if i == 21:
                relative_x = getattr(hand_landmarks[1], 'x', 0)
                relative_y = getattr(hand_landmarks[1], 'y', 0)
                relative_z = getattr(hand_landmarks[1], 'z', 0)

            x = getattr(landmark, 'x', 0) - relative_x
            y = getattr(landmark, 'y', 0) - relative_y
            z = getattr(landmark, 'z', 0) - relative_z

            hand_data[f"Landmark_{i + 1}_X"] = x
            hand_data[f"Landmark_{i + 1}_Y"] = y
            hand_data[f"Landmark_{i + 1}_Z"] = z

        self.landmark_data.append(hand_data)

    def dump_to_csv(self, output):
        df = pd.DataFrame(self.landmark_data)
        
        make_dir(output)
        if os.path.exists(output):
            old_df = pd.read_csv(output)
            df = pd.concat([old_df, df], ignore_index=True)
        
        df.to_csv(output, index=False)


if __name__ == '__main__':
    input_dir = sys.argv[1]
    output_dir = sys.argv[2]
    output_csv = sys.argv[3]


    label = input_dir.split("\\")[-1]

    converter = MediaPipeConverter(label)
    converter.process_from_directory(input_dir, output_dir)
    converter.dump_to_csv(output_csv)


