/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/flood-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use std::io::Result;

pub mod in_borrowed;
pub mod in_stream;
pub mod out_borrowed;
pub mod out_stream;
pub mod prelude;

pub trait WriteOctetStream {
    fn write(&mut self, v: &[u8]) -> Result<()>;
    fn write_u64(&mut self, v: u64) -> Result<()>;
    fn write_i64(&mut self, v: i64) -> Result<()>;
    fn write_u32(&mut self, v: u32) -> Result<()>;
    fn write_i32(&mut self, v: i32) -> Result<()>;
    fn write_u16(&mut self, v: u16) -> Result<()>;
    fn write_i16(&mut self, v: i16) -> Result<()>;
    fn write_u8(&mut self, v: u8) -> Result<()>;
    fn write_i8(&mut self, v: i8) -> Result<()>;
}

pub trait ReadOctetStream {
    fn read(&mut self, v: &mut [u8]) -> Result<()>;
    fn read_u64(&mut self) -> Result<u64>;
    fn read_i64(&mut self) -> Result<i64>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_i32(&mut self) -> Result<i32>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_u8(&mut self) -> Result<u8>;
    fn read_i8(&mut self) -> Result<i8>;
    #[must_use]
    fn has_reached_end(&self) -> bool;
}

pub trait Deserialize: Sized {
    fn deserialize(stream: &mut impl ReadOctetStream) -> Result<Self>;
}

pub trait Serialize: Sized {
    fn serialize(&self, stream: &mut impl WriteOctetStream) -> Result<()>;
}

/// A trait for deserializing objects from a octet buffer.
pub trait BufferDeserializer: Sized {
    /// Deserializes an instance of `Self` from the given octet buffer.
    ///
    /// # Parameters
    ///
    /// * `buf`: A octet slice that contains the serialized data.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<Self>`, which is:
    /// - `Ok((Self, usize))` on successful deserialization, where the `usize` indicates the number of octets consumed.
    /// - `Err(io::Error)` if deserialization fails, which could be due to invalid data or unexpected format.
    fn deserialize(buf: &[u8]) -> Result<(Self, usize)>;
}

/// A trait for serializing objects to an octet buffer.
pub trait BufferSerializer: Sized {
    /// Serializes `self` into the given mutable octet buffer.
    ///
    /// # Parameters
    ///
    /// * `buf`: A mutable reference to a octet slice that will be populated with serialized data.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<usize>`, which is:
    /// - `Ok(usize)` representing the number of octets written to the buffer on success.
    /// - `Err(io::Error)` if serialization fails, which could occur due to issues with writing to the buffer.
    fn serialize(&self, buf: &mut &[u8]) -> Result<usize>;
}
