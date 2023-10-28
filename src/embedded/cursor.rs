use alloc::vec::Vec;

use embedded_io::Read;
use embedded_io::ErrorType;

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
        Cursor { inner: self.inner.clone(), pos: self.pos }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.inner.clone_from(&other.inner);
        self.pos = other.pos;
    }
}

impl<T: ErrorType> ErrorType for Cursor<T> {
    type Error = T::Error;
}

impl Read for Cursor<Vec<u8>>
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let n = Read::read(&mut self.remaining_slice(), buf)?;

        self.pos += n as u32;
        Ok(n)
    }
}

impl Read for Cursor<&[u8]>
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let n = Read::read(&mut self.remaining_slice(), buf)?;

        self.pos += n as u32;
        Ok(n)
    }
}
