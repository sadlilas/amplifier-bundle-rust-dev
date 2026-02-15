# amplifier-bundle-rust-dev

Comprehensive Rust development tools for Amplifier.

Provides:
- **LSP integration** — rust-analyzer for code intelligence
- **Code quality** — cargo fmt, clippy, cargo check integration
- **Auto-checking** — hooks that run on file write/edit
- **Stub detection** — identifies todo!(), unimplemented!(), // TODO patterns
- **Expert agents** — rust-dev (quality) + code-intel (LSP navigation)

## Usage

```yaml
includes:
  - bundle: git+https://github.com/microsoft/amplifier-bundle-rust-dev@main
```

## Individual Behaviors

```yaml
# LSP only (no quality hooks):
includes:
  - bundle: rust-dev:behaviors/rust-lsp

# Quality tools only:
includes:
  - bundle: rust-dev:behaviors/rust-quality
```

## License

This project is licensed under the MIT License.
