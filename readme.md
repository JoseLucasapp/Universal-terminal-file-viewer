# Universal terminal file viewer 🧾

[![Rust](https://img.shields.io/badge/Rust-Stable-informational)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

***Universal terminal file viewer*** is a lightweight CLI tool written in Rust to display **images, PDFs, and plain text files directly in your terminal**, without needing any GUI tools. It supports colorful image rendering with ASCII blocks, PDF viewing, and structured terminal output.

---

* [Installation](#-installation)
* [Usage](#-usage)

  * [Image Mode](#image-mode)
  * [PDF Mode](#pdf-mode)
  * [Text Mode](#text-mode)
* [Contributing](#-contributing)
* [License](#license)

---

## 📦 Installation

Download the latest installer from the [Releases page](https://github.com/JoseLucasapp/Universal-terminal-file-viewer/releases) and run the `.exe` file:

```bash
see_file_installer.exe
```

This will:

* Install `see_file.exe` to `C:\Program Files\see_file`
* Automatically add it to your system `PATH`
* Allow you to run `see_file` from any terminal

After installation, try:

```bash
see_file --help
```

---

### 🔧 Manual install (optional)

Alternatively, if you prefer to build manually from source:

```bash
git clone https://github.com/JoseLucasapp/Universal-terminal-file-viewer
cd Universal-terminal-file-viewer
cargo build --release
```

To make it available globally:

```bash
copy target\release\see_file.exe C:\Users\<your-user>\.cargo\bin
```

Or add its folder to your PATH manually.

---

## 📖 Usage

```bash
see_file [OPTIONS] --image <path>
see_file [OPTIONS] --pdf <path>
see_file [OPTIONS] --text <path>
```

### ⚙️ Global Options

| Option           | Description                                  |
| ---------------- | -------------------------------------------- |
| `--width`, `-w`  | Horizontal scale factor for block size (1–2) |
| `--height`, `-v` | Vertical scale factor (1–9)                  |

---

## 🖼️ Image Mode

Display any image in the terminal using RGB ASCII blocks.

```bash
see_file --image ./example.jpg -w 2 -v 3
```

---

## 📄 PDF Mode

Extract and print PDF text:

```bash
see_file --pdf ./resume.pdf
```

---

## 📜 Text Mode

Display plain text files directly:

```bash
see_file --text ./notes.txt
```

---

## 🤝 Contributing

Contributions are welcome! Feel free to submit issues, ideas, or pull requests on [GitHub](https://github.com/joselucasapp/Universal-terminal-file-viewer).

---

## 📄 License

[MIT](./LICENSE) © [joselucasapp](https://github.com/joselucasapp)