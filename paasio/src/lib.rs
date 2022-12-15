use std::io::{Read, Result, Write};

// - The total number of bytes read/written.
// - The total number of read/write operations.
pub struct ReadStats<R> {
    wrapped: R,
    reads: usize,
    read_bytes: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            reads: 0usize,
            read_bytes: 0usize,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.read_bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;

        let result = self.wrapped.read(buf);
        if let Ok(result) = result {
            self.read_bytes += result;
        }
        result
    }
}

// - The total number of bytes read/written.
// - The total number of read/write operations.
pub struct WriteStats<W> {
    wrapped: W,
    writes: usize,
    write_bytes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            writes: 0,
            write_bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.write_bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;

        self.wrapped.write(buf).map(|bytes| {
            self.write_bytes += bytes;

            bytes
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
