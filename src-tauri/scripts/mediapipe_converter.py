import sys
import numpy as np
import os
import cv2
import mediapipe as mp

class MediaPipeConverter:
    def __init__(self):
        self.mp_drawing = mp.solutions.drawing_utils
        self.mp_drawing_styles = mp.solutions.drawing_styles
        self.mp_hands = mp.solutions.hands
        self.hands = self.mp_hands.Hands(
            static_image_mode=False,
            max_num_hands=2,
            min_detection_confidence=0.5,
            min_tracking_confidence=0.5
        )

    def get_image(self, image_path) -> np.ndarray:
        image = cv2.imread(image_path)
        image = cv2.flip(image, 1)
        image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
        return image

    def get_landmarks(self, image):
        results = self.hands.process(image)
        if not results.multi_hand_landmarks:
            return None

        return results.multi_hand_landmarks

    def annotate_image(self, image, landmarks):
        annotated_image = image.copy()
        for landmark in landmarks:
            self.mp_drawing.draw_landmarks(
                annotated_image, landmark, self.mp_hands.HAND_CONNECTIONS,
                self.mp_drawing_styles.get_default_hand_landmarks_style(),
                self.mp_drawing_styles.get_default_hand_connections_style()
            )
        return annotated_image

    def make_output_dir(self, output_dir):
        print("RAW OUTPUT:", output_dir)
        dirs = output_dir.split('\\')
        for i in range(1, len(dirs)):
            output_dir = '\\'.join(dirs[:i])

            print(output_dir)
            if not os.path.exists(output_dir):
                os.mkdir(output_dir)

    def write_image(self, image, output_path):
        image = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)
        self.make_output_dir(output_path)
        cv2.imwrite(output_path, image)

    def process_from_directory(self, input_dir, output_dir):
        for filename in os.listdir(input_dir):
            image_path = os.path.join(input_dir, filename)
            image = self.get_image(image_path)
            landmarks = self.get_landmarks(image)
            if landmarks is not None:
                annotated_image = self.annotate_image(image, landmarks)
                output_path = os.path.join(output_dir, filename)
                self.write_image(annotated_image, output_path)


def convert_images(input_dir, output_dir):
    converter = MediaPipeConverter()
    converter.process_from_directory(input_dir, output_dir)



if __name__ == '__main__':
    input_dir = sys.argv[1]
    output_dir = sys.argv[2]

    print(f"Converting images from {input_dir} to {output_dir}")

    convert_images(input_dir, output_dir)