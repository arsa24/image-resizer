
# Image Resizer CLI

A command-line tool for resizing and converting images. This tool allows you to resize images, change their format, and optionally delete the original files after processing.

## Features

- Resize image by percentage.
- Specify output file extension.
- Process all images in the current directory.
- Optionally delete the original files after processing.

## Prerequisites

- Rust programming language installed. You can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).
- Image libraries `image` and `clap` for processing and argument parsing.

## Installation

To use the Image Resizer, follow the steps below to set it up:

### Clone the Repository

```bash
git clone https://github.com/yourusername/image-resizer.git
cd image-resizer
```

### Build the Project

1. **Build in Debug Mode** (For development):
    ```bash
    cargo build
    ```

2. **Build in Release Mode** (For production):
    ```bash
    cargo build --release
    ```

After building, the executable will be found in the `target/debug/` or `target/release/` directory.

### Run the Project

You can run the project directly from the command line using `cargo run` or by running the built binary.

### Example Usage

1. **Resize a Single Image**
    ```bash
    cargo run -- path/to/your/image.jpg --reduce 50 --extension png
    ```

    This will reduce the image's size by 50% and save it with a `.png` extension.

2. **Resize All Images in the Current Directory**
    ```bash
    cargo run -- --all
    ```

    This will resize all valid image files in the current directory using the default settings (`reduce 50` and `jpg` extension).

3. **Resize All Images and Delete Original Files**
    ```bash
    cargo run -- --all --delete
    ```

    This will resize all valid image files in the current directory and delete the original files after processing.

4. **Custom Reduce and Extension**
    ```bash
    cargo run -- --reduce 70 --extension png --all
    ```

    This command will resize all images in the current directory by 70% and convert them to `.png` format.

### Command-Line Arguments

| Argument             | Description                                                                 | Default Value |
|----------------------|-----------------------------------------------------------------------------|---------------|
| `file`               | Path to the input image file. If not specified, the program will look for files in the current directory. | N/A           |
| `-r, --reduce`       | Set the image quality (1-100). Reduces the image size by the given percentage. | `50`          |
| `-e, --extension`    | Specify the output file extension.                                           | `jpg`         |
| `-a, --all`          | Process all image files in the directory.                                    | N/A           |
| `-d, --delete`       | Delete the input file after processing.                                      | N/A           |

---

## Example Workflow

1. **Resize a single image:**

    ```bash
    cargo run -- path/to/image.jpg --reduce 75 --extension png
    ```

    This will resize the image to 75% of its original size and save it as a PNG file.

2. **Resize all images in the current directory:**

    ```bash
    cargo run -- --all
    ```

    This will resize all images in the current directory by 50% and save them as JPG files.

3. **Resize and delete original images after processing:**

    ```bash
    cargo run -- --all --delete
    ```

    This will resize all images in the directory and then delete the original files after processing.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
