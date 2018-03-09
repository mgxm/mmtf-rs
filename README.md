# mmtf-rs
*WARNING: Work In Progress*

[![Build Status](https://travis-ci.org/mgxm/mmtf-rs.svg?branch=master)](https://travis-ci.org/mgxm/mmtf-rs)



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
- [ ] Decoder <code>[11/15]</code>
    -   [X] Type 1
    -   [X] Type 2
    -   [X] Type 3
    -   [X] Type 4
    -   [X] Type 5
    -   [X] Type 6
    -   [X] Type 7
    -   [X] Type 8
    -   [X] Type 9
    -   [X] Type 10
    -   [X] Type 11
    -   [ ] Type 12
    -   [ ] Type 13
    -   [ ] Type 14
    -   [ ] Type 15
- [ ] Encoder <code>[0/15]</code>
    -   [ ] Type 1
    -   [ ] Type 2
    -   [ ] Type 3
    -   [ ] Type 4
    -   [ ] Type 5
    -   [ ] Type 6
    -   [ ] Type 7
    -   [ ] Type 8
    -   [ ] Type 9
    -   [ ] Type 10
    -   [ ] Type 11
    -   [ ] Type 12
    -   [ ] Type 13
    -   [ ] Type 14
    -   [ ] Type 15

License: Apache-2.0
