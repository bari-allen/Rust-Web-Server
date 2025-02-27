from scipy.linalg import eigh
import numpy as np
import matplotlib.pyplot as plt
import cv2 as cv
import sys
import os

def load(filename):
    filepath = f"./images/{filename}"
    image = cv.imread(filepath)

    if image is None:
        raise FileNotFoundError
    
    image_rgb = cv.cvtColor(image, cv.COLOR_BGR2RGB)

    image_rgb = cv.resize(image_rgb, (60, 60))


    image_array = np.asarray(image_rgb, dtype=np.float64)
    flattened_image = image_array.flatten()

    return flattened_image

def get_covariance(dataset):
    multiplier = 1 / (len(dataset) - 1)
    dot_product_sum = np.dot(np.transpose(dataset), dataset)
    covariance_mat = multiplier * dot_product_sum
    return covariance_mat

def get_eig_prop(covariance_mat, prop):
    eigenvals, eigenvecs = eigh(covariance_mat)
    divisor = np.sum(eigenvals)
    threshold = prop * divisor

    idx = eigenvals > threshold
    eigenvals = eigenvals[idx][::-1]
    eigenvecs = eigenvecs[:, idx][:, ::-1]

    eigenval_mat = np.diag(eigenvals)
    return eigenval_mat, eigenvecs

def get_eig(covariance_mat, k):
    n = len(covariance_mat)
    eigenvals, eigenvecs = eigh(covariance_mat, subset_by_index=[n - k, n - 1])

    eigenvals = eigenvals[::-1]
    eigenvecs = eigenvecs[:, ::-1]
    
    eigenvals = np.diag(eigenvals)
    return eigenvals, eigenvecs

def project_and_reconstruct(image, eigenvecs):
    alpha = np.dot(image, eigenvecs)
    pca_image = np.dot(alpha, np.transpose(eigenvecs))
    return pca_image

image_files = [f for f in os.listdir('./images') if f.endswith(('.png'))]

flattened_images = []
for filename in image_files:
    flattened_image = load(filename)
    flattened_images.append(flattened_image)

dataset = np.asarray(flattened_images)
dataset -= np.mean(dataset)

covariance_mat = get_covariance(dataset)

eigenvals, eigenvecs = get_eig_prop(covariance_mat, 0.001)

for filename in image_files:
    image = load(filename)
    reconstructed = project_and_reconstruct(image, eigenvecs)
    reconstructed = np.clip(reconstructed, 0, 255).astype(np.uint8)
    reconstructed = np.reshape(reconstructed, (60, 60, 3))

    path = "./pca_images"
    reconstructed = cv.cvtColor(reconstructed, cv.COLOR_RGB2BGR)
    path = os.path.join(path, filename)
    cv.imwrite(path, reconstructed)