# Command Line Applications in Rust

Repository dedicated to study the content of: [Command Line Applications in Rust](https://rust-cli.github.io/book/)

## Related docs

- [Learn Rust](https://www.rust-lang.org/learn)
- [ripgrep (rg)](https://github.com/BurntSushi/ripgrep)
- [Struct std::path::PathBuf](https://doc.rust-lang.org/1.39.0/std/path/struct.PathBuf.html)
- [Clap - Command Line Argument Parser for Rust](https://docs.rs/clap/latest/clap/)
  - [Clap complete - shell completions](https://docs.rs/clap_complete/latest/clap_complete/)
- [Crate anyhow](https://docs.rs/anyhow/latest/anyhow/)
- [indicatif - A progress bar and cli reporting library for Rust](https://crates.io/crates/indicatif)
- [env_logger - A logging implementation for `log` which is configured via an environment variable.](https://docs.rs/env_logger/0.10.0/env_logger/)

## Related videos
- 

## Exercise: `grrs` lib

- Grep clone written in rust.

### Commands

#### Running application

```bash
cargo run -- LaTeX ../random.md
```
#### Setting environment variables before running the program to see log infos 
```bash
export RUST_LOG=info
```