# see\_file 🧾

[![Rust](https://img.shields.io/badge/Rust-Stable-informational)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

***see\_file*** is a lightweight CLI tool written in Rust to display **images, PDFs, and plain text files directly in your terminal**, without needing any GUI tools. It supports colorful image rendering with ASCII blocks, PDF viewing (as text or visual), and structured terminal output.

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

Clone and build with Cargo:

```bash
git clone https://github.com/your-username/see_file
cd see_file
cargo build --release
```

Or install globally with:

```bash
cargo install --path .
```

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

Contributions are welcome! Feel free to submit issues, ideas, or pull requests on [GitHub](https://github.com/joselucasapp/see_file).

---

## 📄 License

[MIT](./LICENSE) © [joselucasapp](https://github.com/joselucasapp)