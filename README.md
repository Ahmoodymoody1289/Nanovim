# NanoVim 🦀

> A minimal terminal text editor written in Rust.

NanoVim is a small, lightweight text editor built from scratch in Rust.
It started as a learning project while working through [The Rust Programming Language](https://doc.rust-lang.org/book/), and is gradually growing into a usable terminal editor.

## ✨ Features

* 📂 Open and save text files
* 📝 Insert and delete text
* ↵ Insert new lines with `Enter`
* 🧭 Navigate using the arrow keys
* 📜 Scroll through larger files
* 💾 Save manually with `Ctrl + S`
* 💾 Automatically save when exiting

## 📦 Installation

### Prerequisites

* [Rust](https://www.rust-lang.org/) 1.70 or newer
* Linux or macOS

### Clone the repository

```bash
git clone https://github.com/Ahmoodymoody1289/Nanovim.git
cd Nanovim
```

### Install

If you're using the included installation script:

```bash
./install.sh
```

You can then open a file with:

```bash
nvim filename.txt
```

### Run without installing

You can also run NanoVim directly through Cargo:

```bash
cargo run -- filename.txt
```

## 🚀 Usage

```bash
nvim <path/to/file>
```

For example:

```bash
nvim ~/Documents/notes.txt
```

## ⌨️ Keyboard Controls

| Key          | Action                |
| ------------ | --------------------- |
| `Arrow Keys` | Move / scroll         |
| `Ctrl + S`   | Save the current file |
| `Esc`        | Exit NanoVim          |

## 🛠️ Development

Clone the repository and build it with Cargo:

```bash
git clone https://github.com/Ahmoodymoody1289/Nanovim.git
cd Nanovim
cargo build
```

To run the editor during development:

```bash
cargo run -- filename.txt
```

## 🗺️ Roadmap

NanoVim is still an early-stage project. Planned improvements may include:

* [ ] Better text editing
* [ ] More keyboard commands
* [ ] Improved file handling
* [ ] More robust error handling
* [ ] Search and replace
* [ ] Additional editor features

## 📚 Learning

NanoVim was originally created as a way to learn Rust while following [The Rust Programming Language](https://doc.rust-lang.org/book/).

The project will continue evolving as I learn more Rust.

## 📄 License

NanoVim is licensed under the MIT License. See [`LICENSE`](LICENSE) for details.

