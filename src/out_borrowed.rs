/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/flood-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use crate::WriteOctetStream;
use std::io::{Error, ErrorKind, Result};

pub struct OctetRefWriter<'a> {
    data: &'a mut [u8],
    offset: usize,
}

impl<'a> OctetRefWriter<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data, offset: 0 }
    }

    /// Returns the number of bytes written so far.
    #[inline]
    pub fn bytes_written(&self) -> usize {
        self.offset
    }

    /// Returns the remaining capacity of the internal slice.
    #[inline]
    pub fn remaining_capacity(&self) -> usize {
        self.data.len() - self.offset
    }

    fn ensure_capacity(&self, additional: usize) -> Result<()> {
        if self.remaining_capacity() < additional {
            return Err(Error::new(
                ErrorKind::WriteZero,
                "Not enough capacity in buffer",
            ));
        }
        Ok(())
    }

    pub fn data(&mut self) -> &mut [u8] {
        self.data
    }
}

impl<'a> WriteOctetStream for OctetRefWriter<'a> {
    fn write_u64(&mut self, v: u64) -> Result<()> {
        self.ensure_capacity(8)?;
        self.data[self.offset..self.offset + 8].copy_from_slice(&v.to_be_bytes());
        self.offset += 8;
        Ok(())
    }

    fn write_i64(&mut self, v: i64) -> Result<()> {
        self.write_u64(v as u64)
    }

    fn write_u32(&mut self, v: u32) -> Result<()> {
        self.ensure_capacity(4)?;
        self.data[self.offset..self.offset + 4].copy_from_slice(&v.to_be_bytes());
        self.offset += 4;
        Ok(())
    }

    fn write_i32(&mut self, v: i32) -> Result<()> {
        self.write_u32(v as u32)
    }

    fn write_u16(&mut self, v: u16) -> Result<()> {
        self.ensure_capacity(2)?;
        self.data[self.offset..self.offset + 2].copy_from_slice(&v.to_be_bytes());
        self.offset += 2;
        Ok(())
    }

    fn write_i16(&mut self, v: i16) -> Result<()> {
        self.write_u16(v as u16)
    }

    fn write_u8(&mut self, v: u8) -> Result<()> {
        self.ensure_capacity(1)?;
        self.data[self.offset] = v;
        self.offset += 1;
        Ok(())
    }

    fn write_i8(&mut self, v: i8) -> Result<()> {
        self.write_u8(v as u8)
    }

    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.ensure_capacity(buf.len())?;
        self.data[self.offset..self.offset + buf.len()].copy_from_slice(buf);
        self.offset += buf.len();
        Ok(())
    }
}
