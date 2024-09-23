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


### Add as a MacOS service

If you don't want to install with Homebrew, first clone the project and build the binary:

```sh
git clone git@github.com:snorremd/process-bonk.git
cd process-bonk
cargo run build --release
```

Then add the binary as a service.
Replace `<path-to-binary>` and `<path-to-config>` in the `.plist` file with the correct paths to the binary and config file.

```sh
cp example.toml process-bonk.toml # make a copy of example config
cp example.plist process-bonk.plist # make a copy of example plist
open process-bonk.toml # Edit the config file to specify correct process to monitor
open process-bonk.plist # Edit the plist file to point to the correct binary path
sudo chown root:wheel process-bonk.plist
sudo chmod 644 process-bonk.plist
sudo cp process-bonk.plist /Library/LaunchDaemons/io.snorre.process-bonk.plist
sudo launchctl load /Library/LaunchDaemons/io.snorre.process-bonk.plist
```

You are free to put the binary and config files wherever you want.
The plist file should point to the correct binary and config paths.

### Homebrew service

Running the service with Homebrew as root is not recommended.
You can run as non-root user though:

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