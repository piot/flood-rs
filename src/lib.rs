use std::io::{Cursor, Read, Result};

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
    fn write_debug_marker(&mut self, marker: u8) -> Result<()>;
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
    fn verify_debug_marker(&mut self, expected: u8) -> Result<()>;
}

pub struct OutOctetStream {
    pub data: Vec<u8>,
    pub should_write_markers: bool,
}

impl OutOctetStream {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            should_write_markers: false,
        }
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

    fn write_debug_marker(&mut self, marker: u8) -> Result<()> {
        if self.should_write_markers {
            self.write_u8(marker)
        } else {
            Ok(())
        }
    }
}

pub struct InOctetStream {
    pub cursor: Cursor<Vec<u8>>,
    pub should_verify_markers: bool,
}

impl InOctetStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            cursor: Cursor::new(data.clone()),
            should_verify_markers: false,
        }
    }

    pub fn new_from_cursor(cursor: Cursor<Vec<u8>>) -> Self {
        Self {
            cursor,
            should_verify_markers: false,
        }
    }
}

impl ReadOctetStream for InOctetStream {
    fn read(&mut self, v: &mut [u8]) -> Result<()> {
        self.cursor.read_exact(v)?;
        Ok(())
    }

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

    fn verify_debug_marker(&mut self, expected: u8) -> Result<()> {
        if !self.should_verify_markers {
            return Ok(()); // Skip verification;
        }
        let mut buf = [0; 1];
        self.cursor.read_exact(&mut buf)?;
        if buf[0] != expected {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid marker, expected: {}, got: {}", expected, buf[0]),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn write_u32() {
        const EXPECTED_U32: u32 = 0x12345678;
        let mut out_stream = OutOctetStream::new();
        let result = out_stream.write_u32(EXPECTED_U32);
        assert!(result.is_ok());

        let mut in_stream = InOctetStream::new(out_stream.data);
        let result = in_stream.read_u32();
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
        let result = in_stream.read_u32();
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
        in_stream.verify_debug_marker(0xcb).unwrap();
        let result = in_stream.read_u32();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EXPECTED_U32);
    }
}
