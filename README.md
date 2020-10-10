[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
# Connection reset

A simple TCP server that listens on port 8081 and sends a bunch of bytes

## Usage

1. To build from source, Rust stable is required.

```
$ cargo build --release
```

2. Please set the following environment variables:

- `$CONN_RES_SECRET` 
	Secret that you wish to send
- `$CONN_RES_ADDR`
	Address you wish to listen from in `ip_addr:port` format.
- `RUST_LOG`(optional)
	There are three log levels, `debug`, `info` and `warn`

3. Run the program with:

```
$ cargo run --release
```
