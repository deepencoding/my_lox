# My_Lox

**Stack**: Rust

---

## 🚀 Project Overview

My_Lox is a Rust implementation of the Lox programming language as presented in the book *Crafting Interpreters*. It includes both a tree-walk interpreter and a bytecode virtual machine, reimagined from the original Java reference to leverage Rust’s performance, memory safety, and idiomatic patterns. The interpreter supports lexical scoping, closures, and a rich feature set, providing a robust playground for language design and runtime systems in Rust.

---

## 🎯 Key Features

- **Tree-walk Interpreter**:  
  - Implements scanning, parsing, and AST evaluation for Lox scripts.  
  - Error reporting with line numbers and descriptive messages.  
  - Supports expressions, statements, control flow (`if`, `while`, `for`-style loops via `while`), functions, and classes as in the Crafting Interpreters specification.

- **Bytecode Virtual Machine (VM)**:  
  - Compiles Lox source into a compact bytecode format.  
  - Stack-based execution for faster runtime performance compared to tree-walk.  
  - Implements a stack, call frames, and a constant pool for literals.  
  - Garbage-collected or reference-counted object management (depending on implementation) for strings, functions, closures, and user-defined data.

- **Lexical Scoping & Closures**:  
  - First-class functions with proper environment chaining.  
  - Captures variables in enclosing scopes, implementing closures that outlive their defining context.  
  - Supports `return`, function declarations, and nested functions.

- **Rust-Idiomatic Design**:  
  - Uses enums (`enum Token`, `enum Expr`, `enum Stmt`, `enum OpCode`) and `match` for exhaustive, safe pattern matching.  
  - Traits for behavior abstractions (e.g., visitor patterns for AST traversal).  
  - Ownership and borrowing ensure memory safety, avoiding common pitfalls of manual memory management.  
  - Modular project layout (lexer, parser, interpreter, compiler/VM) with clear separation of concerns.

- **Performance Optimizations**:  
  - Reduced runtime overhead compared to the Java version: leveraging Rust’s zero-cost abstractions.  
  - Bytecode VM reduces repeated AST-walk overhead for hot code paths.  
  - Efficient memory footprint via Rust’s allocation patterns and optional reference counting where needed.

---

## 📁 Repository Structure

```

my\_lox/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry: REPL or script runner
│   ├── scanner.rs           # Tokenizer / Lexer
│   ├── token.rs             # Token definitions
│   ├── parser.rs            # AST builder
│   ├── ast.rs               # Expr/Stmt enums and visitors
│   ├── interpreter/         # Tree-walk interpreter module
│   │   ├── mod.rs
│   │   ├── environment.rs   # Runtime environments for variables
│   │   ├── eval.rs          # AST evaluator
│   │   └── errors.rs        # Runtime error types
│   ├── compiler/            # Bytecode compiler & VM
│   │   ├── mod.rs
│   │   ├── chunk.rs         # Bytecode chunk, OpCode enum
│   │   ├── compiler.rs      # AST to bytecode compiler
│   │   ├── vm.rs            # VM executor, stack frames, call logic
│   │   └── gc.rs            # Object management (if implemented)
│   └── utils.rs             # Utility functions (e.g., error reporting)
└── tests/                   # Optional: test scripts or Rust integration tests

```

> Adjust paths if your implementation differs. The above is a suggested modular layout.

---

## 🧪 Getting Started

### Prerequisites

- Rust toolchain (stable) installed: `rustup`, `cargo`.
- (Optional) `rustfmt` and `clippy` for formatting and linting.

### Build & Run

1. **Clone the repository**  
   ```bash
   git clone https://github.com/deepencoding/my_lox.git
   cd my_lox
    ```

2. **Build**

   ```bash
   cargo build --release
   ```

   Or for development:

   ```bash
   cargo build
   ```

3. **Run the REPL**

   ```bash
   cargo run
   ```

   * Starts an interactive prompt (`> `) where you can type Lox expressions and statements.
   * Example:

     ```lox
     > var a = "Hello";
     > print a + " World!";
     Hello World!
     ```

4. **Run a Script File**

   ```bash
   cargo run -- path/to/script.lox
   ```

   * Executes the given `.lox` file and prints output or errors to stdout.

5. **Switch Between Interpreter and VM**

   * By default, the implementation may choose tree-walk or VM mode via a command-line flag or compile-time feature.
   * Example:

     ```bash
     cargo run -- --vm path/to/script.lox
     ```
   * Consult CLI help:

     ```bash
     cargo run -- --help
     ```

---

## ⚙️ Configuration & Usage

* **Error Reporting**:

  * Syntax errors report line and message; runtime errors include stack trace.
  * Customize error verbosity or logging in `utils.rs` or error modules.

* **Bytecode VM Settings**:

  * Adjust stack size limits, maximum call depth in `vm.rs`.
  * Tune constant pool strategies or GC behaviors (`gc.rs`) if implemented.

* **Language Extensions**:

  * You can extend Lox with more features: classes (if not yet implemented), native functions, modules, or type annotations.
  * Add new syntax in `parser.rs`, AST nodes in `ast.rs`, and evaluation/compilation logic in interpreter/compiler modules.

---

## 🚧 Testing

* **Unit Tests**:

  * You may add Rust unit tests for scanner, parser, and utility functions under `src/` or in `tests/`.
  * Example:

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_scanner_simple() {
            let tokens = scan_tokens("1 + 2;");
            assert_eq!(tokens.len(), expected_len);
        }
    }
    ```

* **Integration Tests**:

  * Place `.lox` test scripts in `tests/lox_scripts/` and write a test harness that runs them via the CLI, checking output.
  * Example in `tests/integration.rs`:

    ```rust
    #[test]
    fn test_closure_behavior() {
        let output = run_lox_script("tests/lox_scripts/closure.lox");
        assert!(output.contains("expected output"));
    }
    ```

---

## 📚 Learning & References

* **Crafting Interpreters** by Bob Nystrom: Source of the Lox language specification, interpreter design, and bytecode VM architecture.
* Rust documentation on ownership, borrowing, enums, traits, and error handling.
* Community articles on writing interpreters/compilers in Rust for deeper insights.

---

## 🤝 Contributing

Contributions and suggestions are welcome! Possible enhancements:

* Additional language features (classes, inheritance, modules, generics).
* Optimizations in the VM (bytecode profiling, JIT experiments).
* Alternate GC strategies or memory management improvements.
* Better error diagnostics or debugging support.
* CI: integrate tests in GitHub Actions (e.g., run `cargo fmt -- --check`, `cargo clippy`, `cargo test`).

**Workflow**:

1. Fork the repo.
2. Create a feature branch: `git checkout -b feature/awesome-thing`.
3. Implement changes, add tests/docs.
4. Open a Pull Request against `main`.
5. Address review feedback; ensure CI passes.

---

## 🧾 License

This project is released under the **MIT License**. See [LICENSE](./LICENSE) for details.

---

## 🙋‍♂️ Author

**@deepencoding** – Built while reading *Crafting Interpreters*, exploring interpreter and VM design in Rust. Feel free to open issues, suggest improvements, or discuss advanced features!
