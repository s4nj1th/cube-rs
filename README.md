<div align="center">
    <h1>cube-rs</h1>
    <p>ASCII animation of a rotating cube in pure Rust using only standard library functions.</p>
</div>

## Screenshots

[![asciicast](https://asciinema.org/a/h0bOQzMyvln61QkTe538VjbtO.svg)](https://asciinema.org/a/h0bOQzMyvln61QkTe538VjbtO)

## Installation

### Option 1: Download Prebuilt Binary (Recommended)

Precompiled binaries are available on the [Releases](https://github.com/s4nj1th/cube-rs/releases) page.

1. Go to the [Releases](https://github.com/s4nj1th/cube-rs/releases) section.
2. Download the binary for your platform:

   * `cube-rs-x86_64-unknown-linux-gnu.tar.gz` (Linux)
   * `cube-rs-x86_64-apple-darwin.zip` (macOS)
   * `cube-rs-x86_64-pc-windows-msvc.zip` (Windows)
3. Extract it.
4. Run the binary:

   ```bash
   ./cube-rs -m
   ```

### Option 2: Build from Source

Clone the repository:

```bash
git clone https://github.com/s4nj1th/cube-rs.git
cd cube-rs
```

Build and install locally:

```bash
cargo install --path .
```

This will copy the binary to `~/.cargo/bin/cube-rs`.
Make sure `~/.cargo/bin` is in your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Usage

Run the program with one of the following flags:

```bash
cube-rs -s    # Small cube
cube-rs -m    # Medium cube (default if no flag is given)
cube-rs -l    # Large cube
```

If no flag is passed, it defaults to **medium** (`-m`).

## Run without installing

If you don’t want to install globally, you can run directly with Cargo:

```bash
cargo run -- -s
```

Or build and run an optimized release binary:

```bash
cargo build --release
./target/release/cube-rs -l
```

## Example

```bash
cube-rs -m
```

Will animate a cube rotating in your terminal.
Press `Ctrl + C` to exit.

## License

This project is licensed under the [MIT LICENSE](./LICENSE).
