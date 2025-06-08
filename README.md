<h3 align="center">
<img src="https://raw.githubusercontent.com/kevinJ045/rew-docs/main/assets/logo.png" width="100" />
<br/>
Rew(Rust)
<br/>
</h3>
<p  align="center">
<a href="https://github.com/kevinj045/rew/stargazers">  <img src="https://img.shields.io/github/stars/kevinj045/rew?style=for-the-badge&logo=starship&color=cba6f7&logoColor=9399b2&labelColor=181825" alt="GitHub stars"/></a>
<a href="https://github.com/kevinj045/rew/issues">
  <img src="https://img.shields.io/github/issues/kevinj045/guilib?style=for-the-badge&logo=gitbook&color=f5c2e7&logoColor=9399b2&labelColor=181825" alt="GitHub issues"/></a>
<a href="https://github.com/kevinj045/rew">  <img src="https://img.shields.io/github/forks/kevinj045/rew?style=for-the-badge&logo=git&color=94e2d5&logoColor=9399b2&labelColor=181825" alt="GitHub forks"/></a>
<a href="https://www.npmjs.com/package/rayous">  <img src="https://img.shields.io/npm/v/@makano/rew?style=for-the-badge&logo=npm&color=b4befe&logoColor=9399b2&labelColor=181825" alt="npm version" /></a>
</p>

Rew is a coffeescript first runtime with the focus of making an environment suitable for FFI. 

```coffee
using namespace std::ns;

export function main()
  print "hello world"
```

## Available features
### Core Runtime

| Feature                     | Description                                                | Status         |
|-----------------------------|------------------------------------------------------------|----------------|
| Custom JS Runtime           | Built using Deno's `JsRuntime`                             | ✅ Implemented |
| Persistent Runtime State    | Maintains global runtime context across calls              | ✅ Implemented |
| Module Execution            | Executes `.rew` (JavaScript/DSL) files                     | ✅ Implemented |
| Global Scope Injection      | Injects custom globals into JS context                     | ✅ Implemented |
| `__rew_symbols` Support     | Exposes available FFI functions/types via JSON             | ✅ Implemented |
| Multi-threaded Runtime      | Support for running multiple runtimes in parallel          | 🕓 Planned     |
| Runtime Metrics             | Collect and expose runtime performance metrics             | 🕓 Planned     |
| Threads Feature             | Enables running tasks in other threads                    | ✅ Implemented     |

### FFI System

| Feature                     | Description                                                | Status         |
|-----------------------------|------------------------------------------------------------|----------------|
| `rew_bindgen` Proc Macro    | Macro to register Rust functions/types                     | ✅ Implemented |
| Type/Struct Support         | Register Rust structs in FFI layer                         | 🔄 In Progress |
| Pointer/Buffer Handling     | Allow passing pointers and slices to/from JS              | 🔄 In Progress |
| Error Handling              | Native Rust → JS error translation                         | ✅ Implemented |
| JSON Return Marshalling     | Return complex Rust data as JSON to JS                     | ✅ Implemented |
| Custom FFI Signature DSL    | Support simplified syntax for defining signatures          | 🕓 Planned     |
| Async FFI Support           | Enable async Rust functions to be called from JS           | 🕓 Planned     |

### Directives System

| Feature                     | Description                                                | Status         |
|-----------------------------|------------------------------------------------------------|----------------|
| `#declare` Directive        | Local code transformation declarations                     | ✅ Implemented |
| `#declare*` Directive       | Global code transformation declarations                    | ✅ Implemented |
| AST Transform Engine        | Custom transformation engine for directives                | ✅ Implemented |
| Type Inference System       | Basic type tracking/inference for variables and expressions| 🕓 Planned     |
| Directive Validation        | Ensure directives are syntactically and semantically valid | 🕓 Planned     |

### Standard Libraries

| Feature                     | Description                                                | Status         |
|-----------------------------|------------------------------------------------------------|----------------|
| Core FFI APIs               | Low-level interface for `rew.bind(...)`, etc.              | ✅ Implemented |
| File System API             | `fs.readFile`, `fs.writeFile`, etc.                        | ✅ Implemented |
| Networking API              | TCP/UDP sockets, basic `net.connect()`                     | ✅ Basics     |
| HTTP/HTTPS Server           | `http.createServer`, serve requests/responses              | ✅ Basics     |
| Fetch API                   | `fetch()` or similar high-level HTTP client                | ✅ Basics     |
| Timer API                   | `setTimeout`, `setInterval`                                | ✅ Implemented     |
| Database API                | Support for SQLite, Postgres, or other databases           | 🕓 Planned     |
| Stream API                  | Support for readable/writable streams                      | ✅ Implemented   |

### Developer UX

| Feature                     | Description                                                | Status         |
|-----------------------------|------------------------------------------------------------|----------------|
| REPL Interface              | Interactive console for testing                            | 🕓 Planned     |
| Logging / Debug             | Console output, runtime logs, debug messages               | ✅ Basic Logging |
| Inspector/Debugger          | DevTools inspector or debugging tools                      | 🕓 Planned     |
| Error Stack Traces          | Meaningful stack traces from Rust ↔ JS                     | 🔄 In Progress |
| Hot Module Reloading        | Automatically reload modules during development            | 🕓 Planned     |

### Package System

| Feature                     | Description                                                | Status         |
|-----------------------------|------------------------------------------------------------|----------------|
| Local Module Loading        | Support relative `import` or `require`                    | ✅ Implemented |
| URL Module Loading          | Load remote `.js`/`.rew` files                             | 🕓 Planned     |
| Native Module Support       | Load `.so`/`.dll` FFI libraries dynamically                | 🕓 Planned     |
| `rewpkgs` Registry          | Optional registry for standard or community modules        | 🕓 Planned     |
| Dependency Management       | Handle versioning and resolution of dependencies           | 🕓 Planned     |

### Tooling

| Feature                     | Description                                                | Status         |
|-----------------------------|------------------------------------------------------------|----------------|
| CLI Runner (`rew run`)      | CLI tool to run `.coffee` files                               | ✅ Implemented |
| Linter                      | Basic syntax checking and semantic warnings                | 🕓 Planned     |
| Formatter                   | Pretty printer for `.coffee` source code                      | 🕓 Planned     |
| Language Server (LSP)       | IDE support with diagnostics, autocomplete, etc.           | 🕓 Planned     |
| Test Runner                 | Built-in testing framework for `.rew` files                | 🕓 Planned     |
| Documentation Generator     | Generate API documentation from `.rew` files               | 🕓 Planned     |
| Building Language           | A language for defining build processes                   | 🕓 Planned     |
| Bundling Rew Files          | Build and bundle `.brew` files into deployable artifacts    | ✅ Implemented |


## Original rew
The original rew has been moved to [rew-node](https://github.com/kevinj045/rew-node/).
