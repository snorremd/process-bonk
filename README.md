# process-bonk

<img src="./process-bonk.jpeg" style="width: 300px;" />

This is a simple binary program that will monitor a given process and kill it if it misbehaves.

## Usage

Monitor process specified in config file.

```sh
process-bonk ./example.toml
```

See [./example.toml](./example.toml) file for a config example.

## Installation

```sh
brew install snorremd/tap/process-bonk
```

## Development

```sh
cargo run dev ./example.toml
```

## Build

```sh
cargo run build --release
```

## License

Licensed under [MIT](./LICENSE).