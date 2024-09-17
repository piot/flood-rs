/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/flood-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use std::io::{Cursor, Read, Result};

use crate::ReadOctetStream;

pub struct OctetRefReader<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> OctetRefReader<'a> {
    #[must_use]
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    #[must_use]
    pub fn new_from_cursor(cursor: Cursor<&'a [u8]>) -> Self {
        Self { cursor }
    }
}

impl<'a> Read for OctetRefReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.cursor.read(buf)
    }
}

impl<'a> ReadOctetStream for OctetRefReader<'a> {
    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        self.cursor.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        self.cursor.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.cursor.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.cursor.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.cursor.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        self.cursor.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.cursor.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        self.cursor.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.cursor.read_exact(buf)
    }

    #[must_use]
    fn has_reached_end(&self) -> bool {
        self.cursor.position() as usize == self.cursor.get_ref().len()
    }
}
