from scipy.linalg import norm
import numpy as np
import sys
import os
import cv2 as cv

print(os.getcwd())

def import_images(wanted_image):
    image_files = [f for f in os.listdir("./pca_images") if f.endswith(".png")]

    if wanted_image not in image_files:
        raise FileNotFoundError

    flattened_images = []
    flattened_input = None
    for image in image_files:
        filepath = f"./pca_images/{image}"
        image_array = cv.imread(filepath)

        if image_array is None:
            raise FileNotFoundError
        
        image_rgb = cv.cvtColor(image_array, cv.COLOR_BGR2RGB)
        image_rgb = np.array(image_rgb, dtype=np.float64)

        flattened_image = image_rgb.flatten()
        
        if image != wanted_image:
            flattened_images.append(flattened_image)
        else:
            flattened_input = flattened_image
        
    image_files.remove(wanted_image)
    return image_files, flattened_images, flattened_input

def calculate_suggestion(file_names, flattened_images, flattened_input):
    suggestions = []
    for i in range(len(file_names)):
        closest_image = ""
        min_dist = np.inf
        for i in range(len(file_names)):
            if file_names[i] in suggestions:
                continue

            dist = norm(flattened_input - flattened_images[i])
            if dist < min_dist:
                min_dist = dist
                closest_image = file_names[i]
        
        suggestions.append(closest_image)

    return suggestions

input = sys.argv[1]
print(input)
file_names, flattened_images, flattened_input = import_images(input)
suggested = calculate_suggestion(file_names, flattened_images, flattened_input)

print(suggested[0:5])