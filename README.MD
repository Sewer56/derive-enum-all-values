# derive-enum-all-values

[![Crates.io](https://img.shields.io/crates/v/derive-enum-all-values.svg)](https://crates.io/crates/derive-enum-all-values)
[![Docs.rs](https://docs.rs/derive-enum-all-values/badge.svg)](https://docs.rs/derive-enum-all-values)
[![CI](https://github.com/Sewer56/derive-enum-all-values/workflows/CI/badge.svg)](https://github.com/Sewer56/derive-enum-all-values/actions)

## About

A procedural derive macro which exposes a method named `all_values` that returns all defined values as a compile-time static.

## Development

How to develop this project.

***Clone this Repository:***
```bash
# When cloning, make sure symlinks are enabled
git clone -c core.symlinks=true https://github.com/Sewer56/derive-enum-all-values.git
```

***Install Rust:***
- Install the [Rust Toolchain.][rust-toolchain]***Setup IDE***
- This repository is fully with VSCode. [Guidance below](#visual-studio-code-integration).

### Visual Studio Code Integration

`Code`/`VSCode` is the de-facto Rust development environment.  

The following extensions are required:  
- [rust-analyzer][rust-analyzer] for Rust support.  
- [coverage-gutters][coverage-gutters] for Coverage support.  
- [CodeLLDB][codelldb] for debugging.  
- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) easier dependency management.  

The VSCode configuration in Reloaded projects (`.vscode`) contain the following:  
- Run Rust linter `clippy` on Save.  
- Run code format `rustfmt` on Save.  
- Tasks for common operations (generate documentation, active CI/CD etc.).  

These configurations are in the `.vscode` folder; and the tasks can be ran via `Ctrl+Shift+P -> Run Task`.  

## File Layout

The following is the expected file layout for your project:

```
.vscode/
src/
Cargo.toml
```

The `src` folder should contains all source code for your project.  
`Cargo.toml` should be in the root of the project.  

## Contributing

See [CONTRIBUTING](CONTRIBUTING.MD) for guidance on how to contribute to this project.  

## License

Licensed under [MIT](./LICENSE).  

[codelldb]: https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb
[coverage-gutters]: https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters
[rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
[rust-toolchain]: https://www.rust-lang.org/tools/install