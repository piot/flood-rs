/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/flood-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use std::io::{self, Read, Result, Seek, SeekFrom, Write};

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

impl<W: Write> WriteOctetStream for W {
    /// Writes a octet slice to the stream.
    fn write(&mut self, v: &[u8]) -> io::Result<()> {
        self.write_all(v)
    }

    /// Writes a `u64` in big-endian octet order.
    fn write_u64(&mut self, v: u64) -> io::Result<()> {
        self.write_all(&v.to_be_bytes())
    }

    /// Writes an `i64` in big-endian octet order.
    fn write_i64(&mut self, v: i64) -> io::Result<()> {
        self.write_all(&v.to_be_bytes())
    }

    /// Writes a `u32` in big-endian octet order.
    fn write_u32(&mut self, v: u32) -> io::Result<()> {
        self.write_all(&v.to_be_bytes())
    }

    /// Writes an `i32` in big-endian octet order.
    fn write_i32(&mut self, v: i32) -> io::Result<()> {
        self.write_all(&v.to_be_bytes())
    }

    /// Writes a `u16` in big-endian octet order.
    fn write_u16(&mut self, v: u16) -> io::Result<()> {
        self.write_all(&v.to_be_bytes())
    }

    /// Writes an `i16` in big-endian octet order.
    fn write_i16(&mut self, v: i16) -> io::Result<()> {
        self.write_all(&v.to_be_bytes())
    }

    /// Writes a `u8` directly to the stream.
    fn write_u8(&mut self, v: u8) -> io::Result<()> {
        self.write_all(&[v])
    }

    /// Writes an `i8` directly to the stream.
    fn write_i8(&mut self, v: i8) -> io::Result<()> {
        self.write_all(&[v as u8])
    }
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
    fn has_reached_end(&mut self) -> bool;
}

/// Blanket implementation of `ReadOctetStream` for all types that implement `Read` and `Seek`.
impl<R: Read + Seek> ReadOctetStream for R {
    /// Reads a octet slice from the stream.
    fn read(&mut self, v: &mut [u8]) -> io::Result<()> {
        self.read_exact(v)
    }

    /// Reads a `u64` in big-endian octet order.
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    /// Reads an `i64` in big-endian octet order.
    fn read_i64(&mut self) -> io::Result<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    /// Reads a `u32` in big-endian octet order.
    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    /// Reads an `i32` in big-endian octet order.
    fn read_i32(&mut self) -> io::Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    /// Reads a `u16` in big-endian octet order.
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    /// Reads an `i16` in big-endian octet order.
    fn read_i16(&mut self) -> io::Result<i16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    /// Reads a `u8` directly from the stream.
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads an `i8` directly from the stream.
    fn read_i8(&mut self) -> io::Result<i8> {
        let octet = self.read_u8()?;
        Ok(octet as i8)
    }

    /// Checks if the stream has reached the end.
    ///
    /// This method attempts to peek one octet without consuming it. If no octet is available,
    /// it indicates that the end of the stream has been reached.
    /// It is a really hacky way to do it, wished there was another way.
    fn has_reached_end(&mut self) -> bool {
        let current_pos = match self.stream_position() {
            Ok(pos) => pos,
            Err(_) => return false, // Unable to seek, assume not at end
        };

        let mut buffer = [0u8; 1];
        let result = self.read_exact(&mut buffer);

        match result {
            Ok(_) => {
                let _ = self.seek(SeekFrom::Start(current_pos));
                false
            }
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => true,
            Err(_) => false,
        }
    }
}

/// Custom trait for seeking within an octet stream.
pub trait SeekOctetStream {
    /// Moves the cursor to the specified octet position from the start of the stream.
    ///
    /// # Arguments
    ///
    /// * `pos` - The octet position to seek to.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the operation is successful.
    /// * `Err(io::Error)` if an I/O error occurs.
    fn seek(&mut self, pos: u64) -> io::Result<()>;

    /// Retrieves the current octet position of the cursor in the stream.
    ///
    /// # Returns
    ///
    /// * `Ok(u64)` representing the current octet position.
    /// * `Err(io::Error)` if an I/O error occurs.
    fn stream_position(&mut self) -> io::Result<u64>;
}

/// Blanket implementation of `SeekOctetStream` for all types that implement `Seek`.
impl<S: Seek> SeekOctetStream for S {
    /// Moves the cursor to the specified octet position from the start.
    fn seek(&mut self, pos: u64) -> io::Result<()> {
        self.seek(SeekFrom::Start(pos)).map(|_| ())
    }

    /// Retrieves the current cursor position.
    fn stream_position(&mut self) -> io::Result<u64> {
        self.stream_position()
    }
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
