# Colab Compression Script


## Overview

This script is designed to reduce the file size of documents before sending them to Kindle devices via email. By utilizing the zlib library and running the compression process until a specified reduction percentage is achieved, users can efficiently compress large files, optimizing them for Kindle's email size limitations. Simply run the script in Google Colab, providing the necessary inputs, and enjoy reduced file sizes for a smoother Kindle reading experience.This repository contains a Python script designed for use in Google Colab that utilizes the zlib library to compress files. The compression process continues until a specified reduction percentage is achieved. The compressed file is then saved to a user-defined directory with a specified filename. This script can be useful for reducing the file size of documents before sending them to Kindle devices via email.

## Usage

1. Clone the repository:

```bash
   git clone https://github.com/KardelRuveyda/compress-ebook-kindle.git
   cd colab-compression-script
```

- Open the compress-ebook-for-kindle.ipynb notebook in Google Colab.
- Run the cells in the notebook, providing the necessary inputs when prompted.
- The script will compress the file and save the compressed version to the specified directory.

## Example

```bash
input_path = '/content/gdrive/MyDrive/Colab Notebooks/Books/NonCompress/asilacakkadin.mobi'
output_dir = '/content/gdrive/MyDrive/Colab Notebooks/Books/Compress/'
output_filename = 'asilacakkadin-compress.mobi'
reduction_percentage = 20

compress(input_path, output_dir, output_filename, reduction_percentage)
```

### Notes

- Ensure you have the necessary permissions to read the input file and write to the output directory.
- The script uses the zlib library for compression.
- Adjust the input parameters (file path, output directory, filename, reduction percentage) as needed.
