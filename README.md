# sfsync

**S**imple-**F**ile-**SYNC** software

## Installation

### Download Binaly

Download from [Release page(https://github.com/cykps/sfsync/releases)](https://github.com/cykps/sfsync/releases)

### Build From Source

```
git clone git@github.com:cykps/sfsync.git
cd sfsync
cargo build
```

Executable file will be created in `target/release`.

## Usage

### Server

```
$ sfsync --serve
```

### Client

```
$ sfsync
```

### Options

```
Usage: sfsync [OPTIONS]

Options:
  -s, --serve
  -p, --port <PORT>  [default: 3000]
  -h, --help         Print help
  -V, --version      Print version
```

