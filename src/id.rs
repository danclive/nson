use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;

use rand::{self, thread_rng, Rng};

static COUNTER: Lazy<AtomicU16> = Lazy::new(|| AtomicU16::new(thread_rng().gen()));

pub use crate::core::id::*;

impl Id {
    /// Generate a new Id
    ///
    /// # Examples
    ///
    /// ```
    /// use nson::id::Id;
    ///
    /// let id = Id::new();
    ///
    /// println!("{:?}", id);
    /// ```
    pub fn new() -> Id {
        let timestamp = timestamp();
        let counter = gen_count();
        let random_bytes = random_bytes();

        let mut bytes: [u8; 12] = [0; 12];

        bytes[..6].copy_from_slice(&timestamp[2..]);
        bytes[6..8].copy_from_slice(&counter);
        bytes[8..].copy_from_slice(&random_bytes);

        Id::with_bytes(bytes)
    }
}

#[inline]
fn timestamp() -> [u8; 8] {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_millis() as u64;

    time.to_be_bytes()
}

#[inline]
fn gen_count() -> [u8; 2] {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);

    count.to_be_bytes()
}

#[inline]
fn random_bytes() -> [u8; 4] {
    let rand_num: u32 = thread_rng().gen();

    rand_num.to_be_bytes()
}
