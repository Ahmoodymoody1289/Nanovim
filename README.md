# NanoVim

A minimal text editor written in Rust. Built as a learning project following Chapter 10 of "The Rust Programming Language".

## Features

- Open and save text files
- Arrow key navigation
- Text insertion and deletion
- Line breaks with Enter key
- Scrollable buffer for large files
- Manual save with Ctrl+S
- Auto-save on exit

## Installation

### Install Script (Recommended)

```bash
git clone https://github.com/Ahmoodymoody1289/nanovim.git
cd nanovim
./install.sh
nvim filename.txt

Run_without_installing:
cargo run -- myfile.txt

Usage
nvim </Path/to/File>

Requirements:
1. Rust 1.70 or higher
2. Linux or macOS

Guide
Keyboard | Type
ESC      | Exit
Arrows   | Move/Scroll
Ctrl+S   | Save
