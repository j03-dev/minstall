# Make my own package manager

## Description
This is a simple package manager for my own use. It is written in rust and it is not yet complete.

## Install rust if not yet install
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build
```sh
git clone https://github.com/j03-dev/minstall
cd minstall
./build.sh
```

## How to use
### for search package
```bash
target/release/minstall -s <package-name>
```

### for install package
```bash
target/release/minstall -i <package-name>
```
