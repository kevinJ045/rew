# Rew Workspace Structure

This project has been split into multiple smaller crates for better modularity and maintainability. Here's an overview of the new structure:

## Crates Overview

### 1. `rew-core` - Core utilities and types
- **Path**: `./rew-core/`
- **Purpose**: Shared utilities, types, and configuration handling
- **Key modules**:
  - `utils` - File system utilities, app discovery
  - Core types like `BuildOptions`, `AppConfig`, `AppInfo`

### 2. `rew-compiler` - CoffeeScript/TypeScript compiler
- **Path**: `./rew-compiler/`
- **Purpose**: Compilation logic for CoffeeScript, TypeScript, and Civet
- **Key modules**:
  - `compiler` - Main compilation logic and tokenization
  - `declarations` - Declaration engine for variable tracking
  - `civet` - Civet script compilation
  - `jsx` - JSX compilation support

### 3. `rew-extensions` - Deno core extensions
- **Path**: `./rew-extensions/`
- **Purpose**: All deno_core extensions for runtime capabilities
- **Key modules**:
  - `ext/console` - Console API
  - `ext/fs` - File system operations
  - `ext/http` - HTTP client/server
  - `ext/io` - I/O operations
  - `ext/networking` - Network operations
  - `ext/os` - Operating system APIs
  - `ext/process` - Process management
  - `ext/web` - Web APIs
  - And more...

### 4. `rew-runtime` - JavaScript/CoffeeScript runtime
- **Path**: `./rew-runtime/`
- **Purpose**: Main runtime engine built on deno_core
- **Key modules**:
  - `runtime` - Core runtime implementation
  - `builtins` - Built-in modules
  - `workers` - Worker thread support
  - `data_manager` - Data persistence

### 5. `rew-cli` - Command-line interface
- **Path**: `./rew-cli/`
- **Purpose**: CLI application and entry point
- **Key features**:
  - Command parsing with clap
  - File execution
  - Compilation commands
  - Brew file building

### 6. Existing specialized crates (unchanged)
- `jsx/` - JSX compilation library
- `rew-qrew/` - QREW runtime
- `rew-qrew-stub/` - QREW stub
- `rew_bindgen/` - Binding generation
- `rew_bindgen_macros/` - Bindgen macros

## Benefits of This Structure

1. **Modularity**: Each crate has a single, well-defined responsibility
2. **Reusability**: Core functionality can be used independently
3. **Faster builds**: Only changed crates need rebuilding
4. **Better testing**: Each crate can be tested in isolation
5. **Cleaner dependencies**: Explicit dependency relationships
6. **Library support**: Core functionality can be used as libraries

## Workspace Configuration

The root `Cargo.toml` now defines a workspace with:
- Shared dependency versions in `[workspace.dependencies]`
- Common package metadata in `[workspace.package]`
- All member crates listed in `[workspace.members]`

## Building

### Build all crates:
```bash
cargo build
```

### Build specific crate:
```bash
cargo build -p rew-cli
cargo build -p rew-runtime
# etc.
```

### Using the build script:
```bash
./build.sh
```

## Next Steps

1. **Fix imports**: Some imports may need adjustment after the split
2. **Update tests**: Move tests to appropriate crates
3. **Documentation**: Add crate-level documentation
4. **CI/CD**: Update build pipelines for workspace structure
5. **Versioning**: Consider independent versioning for crates

## Dependency Graph

```
rew-cli
├── rew-core
├── rew-runtime
│   ├── rew-core
│   ├── rew-compiler
│   └── rew-extensions
rew-compiler
└── (minimal external deps)
rew-extensions
└── (deno_* crates)
rew-core
└── (minimal external deps)
```

This structure makes the codebase more maintainable and allows for better separation of concerns.
