# process-bonk

<img src="./process-bonk.jpeg" style="width: 300px;" />

This is a simple binary program that will monitor a given process and kill it if it misbehaves.

## Installation

```sh
brew install snorremd/tap/process-bonk
```

## Usage

Monitor process specified in config file.

```sh
process-bonk ./example.toml
```

See [./example.toml](./example.toml) file for a config example.
To get correct CPU usage you need to run the program with the correct privileges.
E.g. for processes owned by root use sudo to run process as root.

### Homebrew service

If you want to run it as a service:

```sh
# Specify config
open $(brew --prefix)/etc/process-bonk/process-bonk.toml

# Start service
brew services start process-bonk

# Look at logs
tail -f $(brew --prefix)/var/log/process-bonk.log
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