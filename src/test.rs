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
