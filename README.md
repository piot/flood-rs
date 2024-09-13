# flood-rs

A Rust library for reading and writing octet streams, facilitating custom serialization and deserialization.

## Overview

flood-rs provides traits and implementations for working with [networked ordered, big endian](https://en.wikipedia.org/wiki/Endianness) octet streams.
It defines the `WriteOctetStream` and `ReadOctetStream` traits for writing to and reading from octet streams,
along with concrete implementations like `OutOctetStream` and `InOctetStream`.

This library is useful for scenarios requiring custom serialization logic, such as network protocols, file formats, or inter-process communication.

## Features

* Custom Octet Stream Traits: Implement `WriteOctetStream` and `ReadOctetStream` to define your own serialization logic.
* Built-in Implementations: Use `OutOctetStream` for writing and `InOctetStream` for reading in-memory byte buffers.
* Primitive Type Support: Read and write primitive types like u8, i8, u16, i16, u32, i32, u64, and i64.
* Serialization Traits: Implement `Serialize` and `Deserialize` for custom data structures.

## Installation

Add flood-rs to your Cargo.toml:

```toml
[dependencies]
flood-rs = "0.0.7"
```

## Usage

### Writing Data to an Octet Stream

```rust
use flood_rs::{OutOctetStream, WriteOctetStream};
use std::io::Result;

fn main() -> Result<()> {
    let mut stream = OutOctetStream::new();
    stream.write_u32(42)?;
    stream.write_i16(-123)?;
    stream.write_u8(255)?;
    let octets = stream.octets();
    Ok(())
}
```

### Reading Data from an Octet Stream

```rust
use flood_rs::{InOctetStream, ReadOctetStream};
use std::io::Result;

fn main() -> Result<()> {
    let data = &[0x00, 0x00, 0x00, 0x2A, 0xFF, 0x85, 0xFF];
    let mut stream = InOctetStream::new(data);
    let value_u32 = stream.read_u32()?;
    let value_i16 = stream.read_i16()?;
    let value_u8 = stream.read_u8()?;
    // Use the deserialized values
    println!("u32: {}, i16: {}, u8: {}", value_u32, value_i16, value_u8);
    Ok(())
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
