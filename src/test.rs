/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/flood-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
#[cfg(test)]
use super::*;

#[test]
fn write_u32() {
    const EXPECTED_U32: u32 = 0x12345678;
    let mut out_stream = OutOctetStream::new();
    let result = out_stream.write_u32(EXPECTED_U32);
    assert!(result.is_ok());

    let mut in_stream = InOctetStream::new(out_stream.data);
    assert!(!in_stream.has_reached_end());
    let result = in_stream.read_u32();
    assert!(in_stream.has_reached_end());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), EXPECTED_U32);
}

#[test]
fn write_markers() {
    const EXPECTED_U32: u32 = 0x12345678;
    let mut out_stream = OutOctetStream::new();
    out_stream.should_write_markers = true;
    out_stream.write_debug_marker(0xcb).unwrap();
    let result = out_stream.write_u32(EXPECTED_U32);
    assert_eq!(out_stream.data.len(), 5);

    assert!(result.is_ok());

    let mut in_stream = InOctetStream::new(out_stream.data);
    in_stream.should_verify_markers = true;
    in_stream.verify_debug_marker(0xcb).unwrap();
    assert!(!in_stream.has_reached_end());
    let result = in_stream.read_u32();
    assert!(in_stream.has_reached_end());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), EXPECTED_U32);
}

#[test]
fn skip_write_markers() {
    const EXPECTED_U32: u32 = 0x12345678;
    let mut out_stream = OutOctetStream::new();
    out_stream.write_debug_marker(0xcb).unwrap();
    let result = out_stream.write_u32(EXPECTED_U32);
    assert_eq!(out_stream.data.len(), 4);
    assert!(result.is_ok());

    let mut in_stream = InOctetStream::new(out_stream.data);
    assert!(!in_stream.has_reached_end());
    in_stream.verify_debug_marker(0xcb).unwrap();
    assert!(!in_stream.has_reached_end());
    let result = in_stream.read_u32();
    assert!(in_stream.has_reached_end());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), EXPECTED_U32);
}
