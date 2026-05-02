# 🦀 Rust Beginner Toolkit

> **Moringa School AI Capstone Project** — An interactive Command-Line Toolkit built in Rust, demonstrating core language concepts through a practical, menu-driven application.

---

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the Project](#running-the-project)
- [Project Structure](#project-structure)
- [Usage Guide](#usage-guide)
- [Rust Concepts Covered](#rust-concepts-covered)
- [Troubleshooting](#troubleshooting)
- [Author](#author)

---

## Overview

The **Rust Beginner Toolkit** is a hands-on educational project designed to introduce Rust fundamentals through a real, working CLI application. It features a menu-driven interface where users can greet individuals, manage student scores, and discover Rust facts — all while exploring key language concepts like structs, ownership, vectors, and pattern matching.

---

## Features

| Feature | Description |
|---|---|
| 👋 Greet Someone | Personalized greeting using a `Greeter` struct |
| 📝 Add Student Score | Add a student name and score to an in-memory list |
| 📋 List All Students | Display all saved students and their scores |
| 💡 Rust Fact | Learn a random fact about the Rust language |
| 🚪 Quit | Exit the program cleanly |

---

## Prerequisites

Before running this project, make sure you have the following installed:

- **Rust & Cargo** (v1.60 or later recommended)
- A terminal / command-line interface
- (Optional) A code editor like [VS Code](https://code.visualstudio.com/) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension

To check if Rust is already installed:

```bash
rustc --version
cargo --version
```

---

## Installation

### Step 1 — Install Rust

**Linux / macOS:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, restart your terminal or run:

```bash
source $HOME/.cargo/env
```

**Windows:**

Download and run the installer from [https://rustup.rs](https://rustup.rs).

---

### Step 2 — Clone or Create the Project

**Option A — Clone this repository (if hosted on GitHub):**

```bash
git clone https://github.com/your-username/rust_toolkit.git
cd rust_toolkit
```

**Option B — Create from scratch:**

```bash
cargo new rust_toolkit
cd rust_toolkit
```

Then replace the contents of `src/main.rs` with the full source code provided in this project.

---

## Running the Project

### ▶️ Run in Development Mode

This compiles and runs the project in a single command:

```bash
cargo run
```

### 🏗️ Build a Release Binary

To compile an optimized binary for production:

```bash
cargo build --release
```

Then execute the compiled binary directly:

```bash
# On Linux / macOS
./target/release/rust_toolkit

# On Windows
.\target\release\rust_toolkit.exe
```

### ✅ Run Tests (if any are defined)

```bash
cargo test
```

### 🔍 Check for Errors Without Compiling

```bash
cargo check
```

### 📦 Clean Build Artifacts

```bash
cargo clean
```

---

## Project Structure

```
rust_toolkit/
├── Cargo.toml        # Project metadata and dependencies
├── Cargo.lock        # Locked dependency versions
├── README.md         # Project documentation (this file)
└── src/
    └── main.rs       # Main application source code
```

---

## Usage Guide

Once the program is running, you will see the main menu:

```
───────────────────────────────────────────────────────
                       MAIN MENU
───────────────────────────────────────────────────────
1. Greet Someone
2. Add Student Score
3. List All Students
4. Show Rust Fact
5. Quit

Enter your choice (1-5):
```

**Menu Options:**

- **1 — Greet Someone:** Enter any name and receive a personalised Rust-themed greeting.
- **2 — Add Student Score:** Enter a student's name and a score between 0–100 to save it to the session list.
- **3 — List All Students:** Displays all students added during the current session.
- **4 — Show Rust Fact:** Prints an interesting fact about the Rust programming language.
- **5 — Quit:** Exits the program.

> ⚠️ **Note:** Student data is stored in memory only. It will be lost when the program exits.

---

## Rust Concepts Covered

This project intentionally demonstrates the following beginner-friendly Rust concepts:

| Concept | Where It Appears |
|---|---|
| `struct` definitions | `Greeter`, `Student` structs |
| `impl` blocks & methods | `Greeter::new()`, `Greeter::greet()` |
| Ownership & borrowing | String handling throughout |
| `Vec<T>` — dynamic arrays | `students` vector |
| `match` expressions | Menu input handling |
| `loop` control flow | Main program loop |
| Standard I/O (`stdin`/`stdout`) | User input and output |
| Error handling with `.unwrap_or()` | Score parsing fallback |
| String operations | `.trim()`, `.parse()`, `.to_string()` |

---

## Troubleshooting

**`command not found: cargo`**
> Rust is not installed or not in your PATH. Re-run the installer or add `~/.cargo/bin` to your PATH:
> ```bash
> export PATH="$HOME/.cargo/bin:$PATH"
> ```

**`error: couldn't read src/main.rs`**
> Make sure you are inside the project directory (`cd rust_toolkit`) before running `cargo run`.

**Score shows as `0` even after entering a number**
> The input may contain non-numeric characters. Enter only digits (e.g., `85`) when prompted for a score.

**Build takes a long time on first run**
> This is normal — Cargo downloads and compiles dependencies on the first build. Subsequent builds are much faster.

---

## Author

**Prepared by:** Michael
**For:** Moringa School AI Capstone Project

---

*Happy Rusting! 🦀*
