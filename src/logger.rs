use std::io::{self, Write};

#[derive(Debug)]
pub(crate) struct Logger<'a, W: Write> {
    out: &'a mut W,
    debug: bool,
}

impl<'a, W: Write> Logger<'a, W> {
    pub(crate) fn new(out: &'a mut W, debug: bool) -> Self {
        Self { out, debug }
    }

    pub(crate) fn debug(&mut self, message: &str) -> io::Result<()> {
        if self.debug {
            return self.out.write_all(message.as_bytes());
        }
        Ok(())
    }

    pub(crate) fn warn(&mut self, message: &str) -> io::Result<()> {
        self.out.write_all(message.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct Buffer {
        inner: Vec<u8>,
    }

    impl Write for Buffer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let _ = self.inner.extend(buf);
            Ok(self.inner.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }

        fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
            let _ = self.inner.extend(buf);
            Ok(())
        }
    }

    #[test]
    fn debug_mode() {
        let mut buffer = Buffer { inner: vec![] };
        let mut logger = Logger::new(&mut buffer, true);
        logger.debug("TEST!").unwrap();
        assert_eq!(
            buffer,
            Buffer {
                inner: b"TEST!".to_vec()
            }
        )
    }

    #[test]
    fn debug_mode_disabled() {
        let mut buffer = Buffer { inner: vec![] };
        let mut logger = Logger::new(&mut buffer, false);
        logger.debug("TEST!").unwrap();
        assert_eq!(buffer, Buffer { inner: vec![] })
    }

    #[test]
    fn warn() {
        let mut buffer = Buffer { inner: vec![] };
        let mut logger = Logger::new(&mut buffer, false);
        logger.warn("W!").unwrap();
        assert_eq!(
            buffer,
            Buffer {
                inner: b"W!".to_vec()
            }
        )
    }
}
