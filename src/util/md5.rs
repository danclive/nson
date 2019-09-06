use std::{fmt, mem};
use std::convert::From;
use std::io::{Result, Write};
use std::ops::{Deref, DerefMut};

/// A digest.
pub struct Digest(pub [u8; 16]);

impl Deref for Digest {
    type Target = [u8; 16];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Digest {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Digest> for [u8; 16] {
    #[inline]
    fn from(digest: Digest) -> Self {
        digest.0
    }
}

macro_rules! implement {
    ($kind:ident, $format:expr) => {
        impl fmt::$kind for Digest {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                for byte in &self.0 {
                    write!(formatter, $format, byte)?;
                }
                Ok(())
            }
        }
    }
}

implement!(LowerHex, "{:02x}");
implement!(UpperHex, "{:02X}");

/// A context.
#[derive(Copy)]
pub struct Context {
    handled: [u32; 2],
    buffer: [u32; 4],
    input: [u8; 64],
}

const PADDING: [u8; 64] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    /// Create a context for computing a digest.
    #[inline]
    pub fn new() -> Context {
        Context {
            handled: [0, 0],
            buffer: [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476],
            input: unsafe { mem::MaybeUninit::uninit().assume_init() },
        }
    }

    /// Consume data.
    pub fn consume<T: AsRef<[u8]>>(&mut self, data: T) {
        let mut input: [u32; 16] = unsafe { mem::MaybeUninit::uninit().assume_init() };
        let mut k = ((self.handled[0] >> 3) & 0x3F) as usize;

        let data = data.as_ref();
        let length = data.len() as u32;
        if (self.handled[0] + (length << 3)) < self.handled[0] {
            self.handled[1] += 1;
        }
        self.handled[0] += length << 3;
        self.handled[1] += length >> 29;

        for &value in data {
            self.input[k] = value;
            k += 1;
            if k != 0x40 {
                continue;
            }
            let mut j = 0;

            for item in input.iter_mut().take(16) {
                *item = (u32::from(self.input[j + 3]) << 24) |
                        (u32::from(self.input[j + 2]) << 16) |
                        (u32::from(self.input[j + 1]) <<  8) |
                        u32::from(self.input[j    ]);

                j += 4;
            }

            transform(&mut self.buffer, &input);
            k = 0;
        }
    }

    /// Finalize and return the digest.
    pub fn compute(mut self) -> Digest {
        let mut input: [u32; 16] = unsafe { mem::MaybeUninit::uninit().assume_init() };
        let k = ((self.handled[0] >> 3) & 0x3F) as usize;

        input[14] = self.handled[0];
        input[15] = self.handled[1];

        self.consume(&PADDING[..(if k < 56 { 56 - k } else { 120 - k })]);

        let mut j = 0;
        for item in input.iter_mut().take(14) {
            *item = (u32::from(self.input[j + 3]) << 24) |
                       (u32::from(self.input[j + 2]) << 16) |
                       (u32::from(self.input[j + 1]) <<  8) |
                       u32::from(self.input[j    ]);
            j += 4;
        }
        transform(&mut self.buffer, &input);

        let mut digest: [u8; 16] = unsafe { mem::MaybeUninit::uninit().assume_init() };

        let mut j = 0;
        for i in 0..4 {
            digest[j    ] = ((self.buffer[i]      ) & 0xFF) as u8;
            digest[j + 1] = ((self.buffer[i] >>  8) & 0xFF) as u8;
            digest[j + 2] = ((self.buffer[i] >> 16) & 0xFF) as u8;
            digest[j + 3] = ((self.buffer[i] >> 24) & 0xFF) as u8;
            j += 4;
        }

        Digest(digest)
    }
}

impl Clone for Context {
    #[inline]
    fn clone(&self) -> Context {
        *self
    }
}

impl From<Context> for Digest {
    #[inline]
    fn from(context: Context) -> Digest {
        context.compute()
    }
}

impl Write for Context {
    #[inline]
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.consume(data);
        Ok(data.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Compute the digest of data.
#[inline]
pub fn compute<T: AsRef<[u8]>>(data: T) -> Digest {
    let mut context = Context::new();
    context.consume(data.as_ref());
    context.compute()
}

fn transform(buffer: &mut [u32; 4], input: &[u32; 16]) {
    let (mut a, mut b, mut c, mut d) = (buffer[0], buffer[1], buffer[2], buffer[3]);

    macro_rules! add(
        ($a:expr, $b:expr) => ($a.wrapping_add($b));
    );
    macro_rules! rotate(
        ($x:expr, $n:expr) => (($x << $n) | ($x >> (32 - $n)));
    );

    {
        macro_rules! F(
            ($x:expr, $y:expr, $z:expr) => (($x & $y) | (!$x & $z));
        );
        macro_rules! T(
            ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => ({
                $a = add!(add!(add!($a, F!($b, $c, $d)), $x), $ac);
                $a = rotate!($a, $s);
                $a = add!($a, $b);
            });
        );

        const S1: u32 =  7;
        const S2: u32 = 12;
        const S3: u32 = 17;
        const S4: u32 = 22;

        T!(a, b, c, d, input[ 0], S1, 3_614_090_360);
        T!(d, a, b, c, input[ 1], S2, 3_905_402_710);
        T!(c, d, a, b, input[ 2], S3,  606_105_819);
        T!(b, c, d, a, input[ 3], S4, 3_250_441_966);
        T!(a, b, c, d, input[ 4], S1, 4_118_548_399);
        T!(d, a, b, c, input[ 5], S2, 1_200_080_426);
        T!(c, d, a, b, input[ 6], S3, 2_821_735_955);
        T!(b, c, d, a, input[ 7], S4, 4_249_261_313);
        T!(a, b, c, d, input[ 8], S1, 1_770_035_416);
        T!(d, a, b, c, input[ 9], S2, 2_336_552_879);
        T!(c, d, a, b, input[10], S3, 4_294_925_233);
        T!(b, c, d, a, input[11], S4, 2_304_563_134);
        T!(a, b, c, d, input[12], S1, 1_804_603_682);
        T!(d, a, b, c, input[13], S2, 4_254_626_195);
        T!(c, d, a, b, input[14], S3, 2_792_965_006);
        T!(b, c, d, a, input[15], S4, 1_236_535_329);
    }

    {
        macro_rules! F(
            ($x:expr, $y:expr, $z:expr) => (($x & $z) | ($y & !$z));
        );
        macro_rules! T(
            ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => ({
                $a = add!(add!(add!($a, F!($b, $c, $d)), $x), $ac);
                $a = rotate!($a, $s);
                $a = add!($a, $b);
            });
        );

        const S1: u32 =  5;
        const S2: u32 =  9;
        const S3: u32 = 14;
        const S4: u32 = 20;

        T!(a, b, c, d, input[ 1], S1, 4_129_170_786);
        T!(d, a, b, c, input[ 6], S2, 3_225_465_664);
        T!(c, d, a, b, input[11], S3,  643_717_713);
        T!(b, c, d, a, input[ 0], S4, 3_921_069_994);
        T!(a, b, c, d, input[ 5], S1, 3_593_408_605);
        T!(d, a, b, c, input[10], S2,   38_016_083);
        T!(c, d, a, b, input[15], S3, 3_634_488_961);
        T!(b, c, d, a, input[ 4], S4, 3_889_429_448);
        T!(a, b, c, d, input[ 9], S1,  568_446_438);
        T!(d, a, b, c, input[14], S2, 3_275_163_606);
        T!(c, d, a, b, input[ 3], S3, 4_107_603_335);
        T!(b, c, d, a, input[ 8], S4, 1_163_531_501);
        T!(a, b, c, d, input[13], S1, 2_850_285_829);
        T!(d, a, b, c, input[ 2], S2, 4_243_563_512);
        T!(c, d, a, b, input[ 7], S3, 1_735_328_473);
        T!(b, c, d, a, input[12], S4, 2_368_359_562);
    }

    {
        macro_rules! F(
            ($x:expr, $y:expr, $z:expr) => ($x ^ $y ^ $z);
        );
        macro_rules! T(
            ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => ({
                $a = add!(add!(add!($a, F!($b, $c, $d)), $x), $ac);
                $a = rotate!($a, $s);
                $a = add!($a, $b);
            });
        );

        const S1: u32 =  4;
        const S2: u32 = 11;
        const S3: u32 = 16;
        const S4: u32 = 23;

        T!(a, b, c, d, input[ 5], S1, 4_294_588_738);
        T!(d, a, b, c, input[ 8], S2, 2_272_392_833);
        T!(c, d, a, b, input[11], S3, 1_839_030_562);
        T!(b, c, d, a, input[14], S4, 4_259_657_740);
        T!(a, b, c, d, input[ 1], S1, 2_763_975_236);
        T!(d, a, b, c, input[ 4], S2, 1_272_893_353);
        T!(c, d, a, b, input[ 7], S3, 4_139_469_664);
        T!(b, c, d, a, input[10], S4, 3_200_236_656);
        T!(a, b, c, d, input[13], S1,  681_279_174);
        T!(d, a, b, c, input[ 0], S2, 3_936_430_074);
        T!(c, d, a, b, input[ 3], S3, 3_572_445_317);
        T!(b, c, d, a, input[ 6], S4,   76_029_189);
        T!(a, b, c, d, input[ 9], S1, 3_654_602_809);
        T!(d, a, b, c, input[12], S2, 3_873_151_461);
        T!(c, d, a, b, input[15], S3,  530_742_520);
        T!(b, c, d, a, input[ 2], S4, 3_299_628_645);
    }

    {
        macro_rules! F(
            ($x:expr, $y:expr, $z:expr) => ($y ^ ($x | !$z));
        );
        macro_rules! T(
            ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => ({
                $a = add!(add!(add!($a, F!($b, $c, $d)), $x), $ac);
                $a = rotate!($a, $s);
                $a = add!($a, $b);
            });
        );

        const S1: u32 =  6;
        const S2: u32 = 10;
        const S3: u32 = 15;
        const S4: u32 = 21;

        T!(a, b, c, d, input[ 0], S1, 4_096_336_452);
        T!(d, a, b, c, input[ 7], S2, 1_126_891_415);
        T!(c, d, a, b, input[14], S3, 2_878_612_391);
        T!(b, c, d, a, input[ 5], S4, 4_237_533_241);
        T!(a, b, c, d, input[12], S1, 1_700_485_571);
        T!(d, a, b, c, input[ 3], S2, 2_399_980_690);
        T!(c, d, a, b, input[10], S3, 4_293_915_773);
        T!(b, c, d, a, input[ 1], S4, 2_240_044_497);
        T!(a, b, c, d, input[ 8], S1, 1_873_313_359);
        T!(d, a, b, c, input[15], S2, 4_264_355_552);
        T!(c, d, a, b, input[ 6], S3, 2_734_768_916);
        T!(b, c, d, a, input[13], S4, 1_309_151_649);
        T!(a, b, c, d, input[ 4], S1, 4_149_444_226);
        T!(d, a, b, c, input[11], S2, 3_174_756_917);
        T!(c, d, a, b, input[ 2], S3,  718_787_259);
        T!(b, c, d, a, input[ 9], S4, 3_951_481_745);
    }

    buffer[0] = add!(buffer[0], a);
    buffer[1] = add!(buffer[1], b);
    buffer[2] = add!(buffer[2], c);
    buffer[3] = add!(buffer[3], d);
}

#[cfg(test)]
mod tests {
    use crate::util::md5;

    #[test]
    fn compute() {
        let inputs = [
            "",
            "a",
            "abc",
            "message digest",
            "abcdefghijklmnopqrstuvwxyz",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
        ];

        let outputs = [
            "d41d8cd98f00b204e9800998ecf8427e",
            "0cc175b9c0f1b6a831c399e269772661",
            "900150983cd24fb0d6963f7d28e17f72",
            "f96b697d7cb7938d525a2f31aaf161d0",
            "c3fcd3d76192e4007dfb496cca67e13b",
            "d174ab98d277d9f5a5611c2c9f419d9f",
            "57edf4a22be3c955ac49da2e2107b67a",
        ];

        for (input, &output) in inputs.iter().zip(outputs.iter()) {
            assert_eq!(format!("{:x}", md5::compute(input)), output);
        }
    }

    #[test]
    fn index() {
        let mut digest = md5::compute(b"abc");
        assert_eq!(digest[0], 0x90);
        assert_eq!(&digest[0], &0x90);
        assert_eq!(&mut digest[0], &mut 0x90);
    }
}
