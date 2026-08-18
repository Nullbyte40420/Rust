# 🦀 Chapter 1: Getting Started with Rust

Welcome to Rust! Chapter 1 covers setting up the Rust environment, understanding the toolchain, writing a bare-bones Rust program using `rustc`, and managing real-world projects using **Cargo**.

## 1. Rust Toolchain Overview

Rust relies on three core command-line tools:

- **`rustup`**: The installer and version manager for Rust. It manages toolchain versions, updates (`rustup update`), and cross-compilation targets.

- **`rustc`**: The standalone core Rust compiler. It compiles `.rs` source code directly into executable machine binaries.

- **`cargo`**: Rust's official **Build System & Package Manager**. It manages project creation, dependencies (crates), compilation, and execution.

## 2. Hello, World! (Direct Compilation with `rustc`)

### A. Writing the Code (`main.rs`)

Create a file named `main.rs` and add the following code:

Rust

```rust
fn main() {
    println!("Hello, world!");
}
```

### B. Code Anatomy

1. **`fn main()`**: The `main` function is the mandatory entry point for every executable Rust program.

2. **`fn`**: Keyword used to declare a function in Rust.

3. **`println!(...)`**:

   - Calling a **Rust Macro** (indicated by the `!`).

   - If it were a regular function call, it would be written as `println(...)` without the `!`.

4. **`"Hello, world!"`**: Passed as a string argument to `println!`.

5. **`;`**: The semicolon signifies that an expression has ended and a statement is complete.

### C. Compiling and Running

In your terminal, compile the source file using `rustc`:

```bash
# 1. Compile the source code into a binary
rustc main.rs

# 2. Run the generated binary
./main        # On Linux / macOS
.\main.exe    # On Windows
```

> ⚠️ **Limitation of** **`rustc`**: Compiling directly with `rustc` is fine for simple single-file programs. However, as projects grow to multiple files and external dependencies, managing compilation manually becomes difficult. This is where **Cargo** comes in.

## 3. Hello, Cargo! (Project Management with Cargo)

Cargo is the standard tool used across the Rust ecosystem to handle project management, dependency builds, and compilation workflows.

### A. Creating a New Project with Cargo

Run the following command in your terminal:

```bash
cargo new hello_cargo
cd hello_cargo
```

This generates a standardized project structure:

```text
hello_cargo/
├── .gitignore          <-- Configured to ignore target/ directory
├── Cargo.toml          <-- Project manifest file
└── src/
    └── main.rs         <-- Source code entry point
```

### B. Understanding `Cargo.toml`

`Cargo.toml` uses the **TOML** (*Tom's Obvious, Minimal Language*) format for configuration:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
# External libraries (crates) needed for this project go here
```

- **`[package]`**: Defines package metadata (name, version, Rust edition).

- **`[dependencies]`**: Lists external packages (crates) your project depends on.

### C. Essential Cargo Commands

Cargo simplifies the development workflow through four key commands:

#### 1. `cargo build`

Compiles your code and creates an executable file inside `target/debug/hello_cargo` (or `.exe` on Windows).

```bash
cargo build
```

#### 2. `cargo run`

Compiles the code (if changes were made) and executes the resulting binary in a single step.

```bash
cargo run
```

#### 3. `cargo check`

Scans your code for syntax, type, and ownership errors **without generating a binary**.

```bash
cargo check
```

> 💡 **Pro-Tip**: `cargo check` is significantly faster than `cargo build` because it skips the code-generation phase. Developers use it constantly while writing code to verify correctness.

#### 4. `cargo build --release`

Compiles your code with **high-level compiler optimizations** for production deployment. It outputs the optimized binary in `target/release/`.

```bash
cargo build --release
```

## 4. `rustc` vs `cargo` Summary

| **Feature**               | **Direct rustc**     | **Cargo Workflow**                    |
| ------------------------- | -------------------- | ------------------------------------- |
| **Primary Use**           | Single-file testing  | Real-world application development    |
| **Dependency Management** | Manual linking       | Automatic via `Cargo.toml`            |
| **Output Location**       | Current directory    | Standardized `target/` directory      |
| **Optimization Flags**    | Manual command flags | Handled automatically via `--release` |

## 🎯 Key Takeaways for Chapter 1

1. **Always use Cargo** for Rust projects to ensure consistent folder structure and build management.

2. The `!` in `println!` indicates a **macro**, not a standard function call.

3. Use **`cargo check`** frequently while coding to catch compilation errors quickly without waiting for full binary builds.

4. Never commit the `target/` directory to Git (Cargo creates a `.gitignore` automatically for this).
