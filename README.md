# rsleep

[![Crates.io](https://img.shields.io/crates/v/rsleep?style=for-the-badge&logo=rust)](https://crates.io/crates/rsleep)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue?style=for-the-badge&logo=rust)](https://www.rust-lang.org)

A prettier version of the sleep command that shows you something is going on.

`rsleep` is a modern Rust-based alternative to the classic `sleep` command, featuring a beautiful progress bar that visualizes the waiting time. Perfect for scripts, demonstrations, or when you just want to see something happening during a delay.

## ✨ Features

- **Visual Progress Bar**: See exactly how much time has passed and how much is remaining
- **Sub-second Precision**: Supports decimal seconds (e.g., `2.5` for 2.5 seconds)
- **Lightweight**: Small binary with minimal dependencies
- **Fast Startup**: Built with Rust for quick execution
- **Cross-platform**: Works on Linux, macOS, and Windows

## 📦 Installation

### From Crates.io

```bash
cargo install rsleep
```

### From Source

```bash
git clone https://github.com/danielsan/rsleep
cd rsleep
cargo install --path .
```

### Build from Source (without installing)

```bash
git clone https://github.com/danielsan/rsleep
cd rsleep
cargo build --release
# Binary will be at target/release/rsleep
```

## 🚀 Usage

Basic usage:

```bash
rsleep <seconds>
```

### Examples

Sleep for 5 seconds:
```bash
rsleep 5
```

Sleep for 2.5 seconds:
```bash
rsleep 2.5
```

Use in a shell script:
```bash
#!/bin/bash
echo "Starting process..."
rsleep 3
echo "Process complete!"
```

Use in a loop with delays:
```bash
for i in {1..5}; do
    echo "Iteration $i"
    rsleep 1.5
done
```

## 📖 Command Line Options

```
rsleep <seconds>

Arguments:
  <seconds>    Duration to sleep in seconds (supports decimals)

Examples:
  rsleep 5     # Sleep for 5 seconds
  rsleep 2.5   # Sleep for 2.5 seconds
```

## 🎨 Progress Bar

The progress bar displays:
- **Spinner**: Animated spinner showing activity
- **Elapsed Time**: Time passed since start
- **Progress Bar**: Visual representation of completion
- **Position/Total**: Current step / Total steps
- **ETA**: Estimated time remaining

Example output:
```
⠙ [00:00:02] [##########>---------] 8/20 (3s)
```

## 🔧 Why rsleep?

- **Better Feedback**: Unlike the standard `sleep` command which provides no output, `rsleep` shows you exactly what's happening
- **Script Debugging**: Makes it easier to see where your script is spending time
- **User Experience**: Provides visual feedback in interactive scripts
- **Learning Tool**: Great for teaching timing concepts or demonstrating delays

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

**Daniel Santana** - [@danielsan](https://github.com/danielsan)

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/danielsan/rsleep/issues).

## ⭐ Show your support

Give a ⭐️ if this project helped you!
