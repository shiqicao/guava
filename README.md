# PDF Cropping Tool

This project is a Rust-based tool for cropping PDF documents based on the bounding box of the text content.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)
- `pdfium-render` crate (https://crates.io/crates/pdfium-render)

## Installing Pdfium

To use the `pdfium-render` crate, you need to install the Pdfium library. Follow the instructions below based on your operating system:

### Linux

1. Download the Pdfium library from the official repository:
   ```sh
   wget https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F4706/pdfium-linux.tgz
   ```

2. Extract the downloaded file:
   ```sh
   tar -xzf pdfium-linux.tgz
   ```

3. Move the extracted files to a directory included in your library path, e.g., `/usr/local/lib`:
   ```sh
   sudo mv pdfium-linux/lib/* /usr/local/lib/
   ```

4. Update the library cache:
   ```sh
   sudo ldconfig
   ```

## Building the Project

To build the project, run the following command in the project directory:

```sh
cargo build --release
```

This will create an optimized build of the project in the `target/release` directory.

## Running the Project

To run the project, use the following command:

```sh
cargo run --release -- <input-pdf> <output-pdf>
```

Replace `<input-pdf>` with the path to the input PDF file and `<output-pdf>` with the path where you want to save the cropped PDF file.

## Example

```sh
cargo run --release -- input.pdf output.pdf
```

This command will crop the `input.pdf` file based on the bounding box of the text content and save the result to `output.pdf`.

## Testing

To run the tests, use the following command:

```sh
cargo test
```

This will execute the tests defined in the project to ensure everything is working correctly.

## License

This project is licensed under the MIT License.