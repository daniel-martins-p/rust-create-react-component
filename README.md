# Rust Create React Component

This is a super simple component generator. It facilitates the creation of a React components through a CLI with few options.

```bash
create-react-component --name Button --output ./src/components
```

## Installing

You must install [Rust](https://www.rust-lang.org/tools/install) to get it working.

```bash
git pull git@github.com:daniel-martins-p/rust-create-react-component.git
cargo build && cargo install --path .
```

## Running

After `installing` use the create-react-component command to create your component.

## Options

| Arguments          | Description                      |
| ------------------ | -------------------------------- |
| `--name` or `-n`   | **Required** Component name      |
| `--output` or `-o` | Where all files should be placed |
