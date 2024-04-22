//! io

#[derive(Debug)]
pub enum Error {
    /// An EOF error was encountered before reading the exact amount of requested bytes.
    UnexpectedEof,
    /// The target slice was full and so could not receive any new data.
    Full,
}

pub type Result<T> = core::result::Result<T, Error>;

/// Blocking reader.
///
/// This trait is the `nson` equivalent of [`std::io::Read`].
pub trait Read {
    /// Read some bytes from this source into the specified buffer, returning how many bytes were read.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => buf = &mut buf[n..],
                Err(e) => return Err(e),
            }
        }
        if buf.is_empty() {
            Ok(())
        } else {
            Err(Error::UnexpectedEof)
        }
    }
}

/// Read is implemented for `&[u8]` by copying from the slice.
///
/// Note that reading updates the slice to point to the yet unread part.
/// The slice will be empty when EOF is reached.
impl Read for &[u8] {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let amt = core::cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }
}

impl<T: ?Sized + Read> Read for &mut T {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        T::read(self, buf)
    }
}

/// Blocking writer.
///
/// This trait is the `nson` equivalent of [`std::io::Write`].
pub trait Write {
    /// Write a buffer into this writer, returning how many bytes were written.
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    /// Flush this output stream, blocking until all intermediately buffered contents reach their destination.
    fn flush(&mut self) -> Result<()>;

    /// Write an entire buffer into this writer.
    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => panic!("write() returned Ok(0)"),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl Write for alloc::vec::Vec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<T: ?Sized + Write> Write for &mut T {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        T::write(self, buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        T::flush(self)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Cursor<T> {
    inner: T,
    pos: u32,
}

impl<T> Cursor<T> {
    pub const fn new(inner: T) -> Cursor<T> {
        Cursor { pos: 0, inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub const fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub const fn position(&self) -> u32 {
        self.pos
    }

    pub fn set_position(&mut self, pos: u32) {
        self.pos = pos;
    }
}

impl<T> Cursor<T>
where
    T: AsRef<[u8]>,
{
    pub fn remaining_slice(&self) -> &[u8] {
        let len = self.pos.min(self.inner.as_ref().len() as u32);
        &self.inner.as_ref()[(len as usize)..]
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.inner.as_ref().len() as u32
    }
}

impl<T> Clone for Cursor<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Cursor {
            inner: self.inner.clone(),
            pos: self.pos,
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.inner.clone_from(&other.inner);
        self.pos = other.pos;
    }
}

impl Read for Cursor<alloc::vec::Vec<u8>> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = Read::read(&mut self.remaining_slice(), buf)?;

        self.pos += n as u32;
        Ok(n)
    }
}

impl Read for Cursor<&[u8]> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = Read::read(&mut self.remaining_slice(), buf)?;

        self.pos += n as u32;
        Ok(n)
    }
}
