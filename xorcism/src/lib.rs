use std::{
    borrow::Borrow,
    io::{Read, Write},
    ops::BitXor,
};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    key: &'a [u8],
    key_cursor: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Xorcism {
            key: key.as_ref(),
            key_cursor: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let iter = self.key.iter().cycle().skip(self.key_cursor);
        data.iter_mut().zip(iter).for_each(|(d, k)| {
            *d ^= k;
        });
        self.key_cursor = (self.key_cursor + data.len()) % self.key.len();
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<
        'c: 'b,
        'b,
        T: Borrow<u8> + BitXor<Output = u8>,
        Data: IntoIterator<Item = T> + 'b,
    >(
        &'c mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + 'b {
        let v = self.key.iter().cycle().skip(self.key_cursor);
        data.into_iter().zip(v).map(|(d, k)| {
            self.key_cursor = (self.key_cursor + 1) % self.key.len();
            d.borrow() ^ k
        })
    }

    pub fn reader(self, r: impl Read + 'a) -> impl Read + 'a {
        XorcismIO { x: self, io: r }
    }

    pub fn writer(self, w: impl Write + 'a) -> impl Write + 'a {
        XorcismIO { x: self, io: w }
    }
}

pub struct XorcismIO<'a, T> {
    x: Xorcism<'a>,
    io: T,
}

impl<'a, T> Read for XorcismIO<'a, T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.io.read(buf).map(|size| {
            self.x.munge_in_place(buf);
            size
        })
    }
}

impl<'a, T> Write for XorcismIO<'a, T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf = &mut buf.to_owned();

        self.x.munge_in_place(buf);

        self.io.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.io.flush()
    }
}
