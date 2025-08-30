<div align="center">
  <img src="https://raw.githubusercontent.com/kevinJ045/rew-docs/main/assets/logo.png" width="100" alt="Rew Logo" />
  <h1>Rew (Rust)</h1>
  <p>
    <strong>Rew is basically my playground for a coffeescipt runtime. It’s built on Rust, Deno Core and V8, and mostly intended to be quick and FFI compatible.</strong>
  </p>
  <p>
    <a href="https://github.com/kevinj045/rew/stargazers">
      <img src="https://img.shields.io/github/stars/kevinj045/rew?style=for-the-badge&logo=starship&color=cba6f7&logoColor=9399b2&labelColor=181825" alt="GitHub stars"/>
    </a>
    <a href="https://github.com/kevinj045/rew/issues">
      <img src="https://img.shields.io/github/issues/kevinj045/rew?style=for-the-badge&logo=gitbook&color=f5c2e7&logoColor=9399b2&labelColor=181825" alt="GitHub issues"/>
    </a>
    <a href="https://github.com/kevinj045/rew/forks">
      <img src="https://img.shields.io/github/forks/kevinj045/rew?style=for-the-badge&logo=git&color=94e2d5&logoColor=9399b2&labelColor=181825" alt="GitHub forks"/>
    </a>
  </p>
</div>

Rew is a CoffeeScript runtime that aims to provide a simple developer-friendly environment. It mostly provides basic a toolkit for most tasks on it's own with the aim to have better libraries built on the FFI layer that rew provides.

While mainly designed for coffeescript, It can also run simple js that isn't dependent on nodejs or any other platform.

```coffee
public package example;

function example::main()
  rew::io::out.print "Hello!"
```

## Core Features

| Feature               | Description                                                                                                                              |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| **Web API Compatible**| Includes a few APIs like `fetch`, `setTimeout`, `Request`, `Response`, and WebSockets.                              |
| **Built-in Tooling**  | CLI tool (`rew`) for running, and managing scripts.                                                           |
| **FFI System**        | A Foreign Function Interface for calling Rust code from CoffeeScript, and vice-versa.                                    |
| **JSX Support**       | Built-in support for JSX compiler written in rust.                                                |

## Available APIs and Extensions

Rew provides a rich set of built-in APIs, made available through extensions. Here's a summary of what's available:

| Category         | Extension         | Description                                                                                             |
| ---------------- | ----------------- | ------------------------------------------------------------------------------------------------------- |
| **Console**      | `ext/console`     | Provides IO logging.                                                              |
| **File System**  | `ext/fs`          | Enables file system access (`read`, `write`, etc.).                                             |
| **HTTP**         | `ext/http`        | Implements `fetch`, WebSockets, and other HTTP-related APIs.                                            |
| **Networking**   | `ext/net`         | Low-level networking APIs.                                                                              |
| **OS**           | `ext/os`          | Provides access to operating system-level information and utilities.                                    |
| **FFI**          | `ext/ffi`         | The Foreign Function Interface for interoperability with native code.                                   |
| **Process**      | `ext/process`     | Provides information and control over the current process.                                              |
| **Web**          | `ext/web`         | A collection of other web-standard APIs.                                                                |

## Project Architecture

The Rew project is organized into a workspace of several crates, each with a specific responsibility:

| Crate                   | Description                                                                                             |
| ----------------------- | ------------------------------------------------------------------------------------------------------- |
| `rew-cli`               | The command-line interface for the Rew runtime.                                                         |
| `rew-compiler`          | Handles the compilation and transformation of different source languages (JS, TS, CoffeeScript, JSX).   |
| `rew-core`              | Core utilities and types shared across the project.                                                     |
| `rew-data-manager`      | Manages data and assets for the runtime.                                                                |
| `rew-extensions`        | Implements the built-in APIs and extensions.                                                            |
| `rew-jsx`               | Provides JSX transformation capabilities.                                                               |
| `rew-permissions`       | The permission-based security model.                                                                    |
| `rew-runtime`           | The core JavaScript/TypeScript runtime, built on Deno Core.                                             |
| `rew-vfile`             | A virtual file system abstraction.                                                                      |

## Getting Started

To get started with Rew, You can either install it through installer scripts, or with cargo and rust.

### Through Pimmy installer

1.  **Run the installer script:**
    ```bash
    curl -fsSL https://raw.githubusercontent.com/kevinj045/rew.pimmy/main/install-rew.sh | bash
    ```
    - **Windows**
    ```powershell
    irm https://raw.githubusercontent.com/kevinj045/rew.pimmy/main/install-windows.ps1 | iex
    ```
    > Note: I recommend you read the scripts to see what you're running
2.  **Test:**
    ```bash
    rew run ./test/all.coffee
    ```

### Through cargo

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/kevinj045/rew.git
    cd rew
    ```

2.  **Install the CLI:**
    ```bash
    cargo install --path ./rew-cli
    ```

3.  **Run a file:**
    You can run any of the test files to see the runtime in action. For example:
    ```bash
    rew run ./test/all.coffee
    ```

## Contributing

YES PLEASE.

## License

MIT — do whatever you want, just don’t sue me.