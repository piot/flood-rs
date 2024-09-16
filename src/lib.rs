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

pub trait Deserialize {
    fn deserialize(stream: &mut impl ReadOctetStream) -> Result<Self>
    where
        Self: Sized;
}

pub trait Serialize {
    fn serialize(&self, stream: &mut impl WriteOctetStream) -> Result<()>
    where
        Self: Sized;
}
