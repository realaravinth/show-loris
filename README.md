[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)

# Slowloris

A simple TCP server that listens on port 8081 and sends a bunch of bytes
A hackish [Slowloris](https://en.wikipedia.org/wiki/Slowloris_(computer_security)) DoS client

## Usage

1. To build from source, Rust stable is required.

```
$ cargo build --release
```

2. Configuration currently needs to be hard-coded, this will hopefully
   be changed later.


3. Run the program with:

```
$ cargo run --release
```
