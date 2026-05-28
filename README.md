# PNG message encoder and decoder
A command-line tool for hiding secret messages inside PNG images using custom chunks (steganography). The resulting images remain fully valid PNGs that open normally in any viewer.

A hands-on Rust project focused on binary file parsing, PNG specification, and building a practical CLI tool.

## Features
- **Encode** - Hide a message using a custom 4-letter chunk type
- **Decode** - Extract hidden messages
- **Remove** - Delete hidden messages while keeping the image valid
- **Print** - Show the internal structure of a PNG file
- Respects PNG "safe-to-copy" rules

## Installation

### Build from source
```bash
git clone git@github.com:UberChili/png-message-encoder-and-decoder.git
cd png-message-encoder-and-decoder

cargo build --release

# or for Windows:
# copy target\release\pngmsg.exe C:\Tools\pngmsg.exe
```
 
Make sure ~/bin (or equivalent) is in your PATH.

### Run directly without installing
```bash
cargo run --release -- encode image.png RuSt "My secret message"
```

### Installing to system
Installing binaries directly to /usr/local via cargo install is generally discouraged on Linux. It bypasses your system's package manager, which can lead to orphaned files and system clutter.

The safest way to install using Cargo is to install the binary to your user's local bin directory:
```bash
# Install to ~/.cargo/bin
cargo install --path .

# Ensure ~/.cargo/bin is in your PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

## Usage
```bash
# Encode a message (creates or overwrites output)
# Accepts an optional output filename
pngmsg encode ./image.png RuSt "This is a hidden message!" ./image_with_secret.png

# Decode a message
pngmsg decode ./image_with_secret.png RuSt

# Remove a hidden message
pngmsg remove ./image_with_secret.png RuSt

# Print PNG structure
pngmsg print ./image.png
```

## Options
- **encode** requires: **filepath**, **chunk_type** (4 letters), and **message**
- An optional output filename where to save the modified image can be provided
- All commands require a .png file

## Example workflow

```bash
# 1. Hide message
pngmsg encode photo.png MySg "Meeting at 22:00 behind the old warehouse" -o photo_secret.png

# 2. Verify
pngmsg print photo_secret.png

# 3. Extract
pngmsg decode photo_secret.png MySg
```
