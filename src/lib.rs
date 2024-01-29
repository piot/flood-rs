pub trait WriteOctetStream {
    fn write_u32(&mut self, v: u32) -> std::io::Result<()>;
    fn write_i32(&mut self, v: i32) -> std::io::Result<()>;
    fn write_u16(&mut self, v: u16) -> std::io::Result<()>;
    fn write_i16(&mut self, v: i16) -> std::io::Result<()>;
    fn write_u8(&mut self, v: u8) -> std::io::Result<()>;
    fn write_i8(&mut self, v: i8) -> std::io::Result<()>;
}

pub trait ReadOctetStream {
    fn read_u32(&mut self) -> std::io::Result<u32>;
    fn read_i32(&mut self) -> std::io::Result<i32>;
    fn read_u16(&mut self) -> std::io::Result<u16>;
    fn read_i16(&mut self) -> std::io::Result<i16>;
    fn read_u8(&mut self) -> std::io::Result<u8>;
    fn read_i8(&mut self) -> std::io::Result<i8>;
}

#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind, Result};

    use super::*;

    pub struct TestWriteOctetStream {
        pub data: Vec<u8>,
    }

    impl TestWriteOctetStream {
        pub fn new() -> Self {
            Self { data: Vec::new() }
        }
    }

    pub struct TestReadOctetStream {
        pub data: Vec<u8>,
    }

    impl WriteOctetStream for TestWriteOctetStream {
        fn write_u32(&mut self, _v: u32) -> Result<()> {
            //self.data.push(v);
            Ok(())
        }

        fn write_i32(&mut self, v: i32) -> Result<()> {
            self.write_u32(v as u32)
        }

        fn write_u16(&mut self, _v: u16) -> Result<()> {
            //self.data.push(v);
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

    impl ReadOctetStream for TestReadOctetStream {
        fn read_i32(&mut self) -> Result<i32> {
            Ok(0)
        }

        fn read_u32(&mut self) -> Result<u32> {
            Ok(0)
        }
        fn read_i16(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn read_u16(&mut self) -> Result<u16> {
            //self.data.pop()
            Ok(0)
        }

        fn read_i8(&mut self) -> Result<i8> {
            Ok(0)
        }

        fn read_u8(&mut self) -> Result<u8> {
            self.data
                .pop()
                .ok_or_else(|| Error::new(ErrorKind::Other, "No more data"))
        }
    }

    #[test]
    fn write_u32() {
        let mut out_stream = TestWriteOctetStream::new();
        let result = out_stream.write_u32(0x12345678);
        assert!(result.is_ok());
    }
}
