/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/flood-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use flood_rs::prelude::*;
use std::io;

#[test]
fn test_write_and_read_u32() -> io::Result<()> {
    const EXPECTED_U32: u32 = 0x12345678;
    let mut out_stream = OutOctetStream::new();
    out_stream.write_u32(EXPECTED_U32)?;

    let mut in_stream = InOctetStream::new(out_stream.octets_ref());
    let value = in_stream.read_u32()?;
    assert_eq!(value, EXPECTED_U32);

    Ok(())
}

fn write_u32(out_stream: &mut impl WriteOctetStream, v: u32) -> io::Result<()> {
    out_stream.write_u32(v)
}

fn read_u32(in_stream: &mut impl ReadOctetStream) -> io::Result<u32> {
    in_stream.read_u32()
}

#[test]
fn test_ref_write_and_read_u32() -> io::Result<()> {
    const EXPECTED_U32: u32 = 0x12345678;
    let mut buf: [u8; 4] = [42; 4];
    let mut out_stream = OctetRefWriter::new(&mut buf);
    write_u32(&mut out_stream, EXPECTED_U32)?;

    let mut in_stream = OctetRefReader::new(out_stream.data());
    let value = read_u32(&mut in_stream)?;
    assert_eq!(value, EXPECTED_U32);

    let should_be_error = in_stream.read_u8();
    assert!(matches!(
        should_be_error.err(),
        Some(e) if e.kind() == io::ErrorKind::UnexpectedEof
    ));

    Ok(())
}
