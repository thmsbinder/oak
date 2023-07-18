# Using the same base image that is used in https://github.com/Trusted-AI/adversarial-robustness-toolbox
FROM nvidia/cuda:11.3.1-cudnn8-runtime-ubuntu20.04

# Isntall all required dependencies and packages
RUN apt-get update -y
RUN apt-get install -y python3 python3-distutils python3-pip --no-install-recommends curl

# Note installing keras directly resulted in some issues with saving the model.
RUN pip3 install --no-cache-dir tensorflow==2.12.0

# Install cleverhans for generating adversarial examples.
RUN pip3 install --no-cache-dir cleverhans==4.0.0

# Copy files in the current directory to project.
RUN mkdir /project
WORKDIR /project
COPY . /project

# Download model
RUN curl --output mnist_model.tar.gz  https://ent-server-62sa4xcfia-ew.a.run.app/raw/sha256:63ea4a9906dbbb3a154f069a0d4bc6a2272b9b8151e69fbd57029c826c2cd779