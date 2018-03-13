# mmtf-rs
*WARNING: Work In Progress*

[![Build Status](https://travis-ci.org/mgxm/mmtf-rs.svg?branch=master)](https://travis-ci.org/mgxm/mmtf-rs)
[![Cargo](https://img.shields.io/crates/v/mmtf.svg)](https://crates.io/crates/mmtf)
[![Docs.rs](https://docs.rs/mmtf/badge.svg)](https://docs.rs/mmtf)
![Crates.io](https://img.shields.io/crates/l/mmtf.svg)


## Decoder and Encoder for the Macromolecular Transmission Format (MMTF)

> The macromolecular transmission format (MMTF) is a binary encoding of biological structures.
> It includes the coordinates, the topology and associated data. Specifically, a large subset of
> the data in `mmCIF` or `PDB` files can be represented. Pronounced goals are a reduced file size for
> efficient transmission over the Internet or from hard disk to memory and fast decoding/parsing speed.
> Additionally the format aims to be easy to understand and implement to facilitates its dissemination.

For a more detailed information and specifications, please, take a look at their official [documentation](https://mmtf.rcsb.org/).

This crate use the deserialize and serialize from Rust `MessagePack` and their integration with Serde,
providing a simple and easily decoder and encoder for MMTF structures

## Examples
### Decoder

```rust
extern crate mmtf;
use std::fd::File;

let file = File::open("/path/to/file.mmtf");
let mmtf = Mmtf::from(file);

```

### TODO Encoder

```rust
```


## TODO
- [ ] Encoder

License: Apache-2.0
