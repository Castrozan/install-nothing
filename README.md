# install-nothing

A terminal application that simulates installing things. It doesn't actually install anything.

![Demo](demo/old-demo.gif)

## Usage

```bash
cargo build --release
cargo run --release
```

Press Ctrl+C to stop.

### Pick what to install

```bash
# Install specific stages
cargo run --release -- deno

# Install everything (default)
cargo run --release -- --all
```

See available stages:
```bash
cargo run --release -- --help
```

## License

Do whatever you want with it.
