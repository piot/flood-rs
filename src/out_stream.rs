/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/flood-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use crate::WriteOctetStream;
use std::io::Result;

pub struct OutOctetStream {
    data: Vec<u8>,
}

impl OutOctetStream {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Returns an owned `Vec<u8>` containing the octets.
    ///
    /// Clones the internal data, which can be expensive for large vectors.
    #[inline]
    pub fn octets(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Returns a borrowed slice of the internal octets.
    ///
    /// This method is more efficient as it avoids cloning.
    #[inline]
    pub fn octets_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Default for OutOctetStream {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteOctetStream for OutOctetStream {
    fn write(&mut self, v: &[u8]) -> Result<()> {
        self.data.extend_from_slice(v);
        Ok(())
    }

    fn write_u64(&mut self, v: u64) -> Result<()> {
        self.data.push((v >> 56) as u8);
        self.data.push((v >> 48) as u8);
        self.data.push((v >> 40) as u8);
        self.data.push((v >> 32) as u8);
        self.data.push((v >> 24) as u8);
        self.data.push((v >> 16) as u8);
        self.data.push((v >> 8) as u8);
        self.data.push(v as u8);
        Ok(())
    }

    fn write_i64(&mut self, v: i64) -> Result<()> {
        self.write_u64(v as u64)
    }

    fn write_u32(&mut self, v: u32) -> Result<()> {
        self.data.push((v >> 24) as u8);
        self.data.push((v >> 16) as u8);
        self.data.push((v >> 8) as u8);
        self.data.push(v as u8);
        Ok(())
    }

    fn write_i32(&mut self, v: i32) -> Result<()> {
        self.write_u32(v as u32)
    }

    fn write_u16(&mut self, v: u16) -> Result<()> {
        self.data.push((v >> 8) as u8);
        self.data.push(v as u8);
        Ok(())
    }

    fn write_i16(&mut self, v: i16) -> Result<()> {
        self.write_u16(v as u16)
    }

    fn write_u8(&mut self, v: u8) -> Result<()> {
        self.data.push(v);
        Ok(())
    }

    fn write_i8(&mut self, v: i8) -> Result<()> {
        self.write_u8(v as u8)
    }
}
