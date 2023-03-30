//#![no_std]

#[cfg(feature = "distribution")]
pub mod distribution;

/// Returns an initial offset for use with a generator.
///
/// ```
/// let ptr = eprng::initial_offset() as *const u8;
/// assert_eq!(0u8, unsafe { *ptr });
/// ```
#[inline]
pub fn initial_offset() -> usize {
    &0u8 as *const u8 as usize
}

/// Fills the given buffer with digit characters.
///
/// Radix is the base, i.e. with radix 10 you will get chars '0'-'9':
///
/// ```
/// let mut buf = ['\0'; 16384];
/// eprng::digit_chars(&mut buf, eprng::initial_offset(), 10);
/// ```
#[inline]
pub fn digit_chars(buf: &mut [char], offset: usize, radix: u32) {
    for (i, val) in buf.iter_mut().enumerate() {
        *val = char::from_digit((offset + i).count_zeros() % radix, radix).unwrap();
    }
}

/// Fills the given buffer with bytes.
///
/// ```
/// let mut buf = [0u8; 16384];
/// eprng::bytes(&mut buf, eprng::initial_offset());
/// ```
#[inline]
pub fn bytes(buf: &mut [u8], offset: usize) {
    for (i, val) in buf.iter_mut().enumerate() {
        *val = (offset + i).count_zeros() as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let offset = initial_offset();

        let mut buf1 = ['\0'; 32];
        digit_chars(&mut buf1, offset, 10);

        let mut buf2 = ['\0'; 32];
        digit_chars(&mut buf2, offset, 10);

        assert_eq!(buf1, buf2);
    }

    #[test]
    fn partial_eq() {
        let offset = initial_offset();

        let mut buf1 = ['\0'; 32];
        digit_chars(&mut buf1, offset, 10);

        let mut buf2 = ['\0'; 32];
        digit_chars(&mut buf2, offset + 1, 10);

        assert_eq!(buf1[1..], buf2[..31]);
    }

    #[test]
    fn transparency() {
        // Adding 32 to this offset keeps the amounts of zeros exactly the same in the range we are inspecting!
        let offset = 0b000000000_00000000_1111111_11110111_00001100_00100101_01100100_10100000;

        let mut buf1 = ['\0'; 32];
        digit_chars(&mut buf1, offset, 10);

        let mut buf2 = ['\0'; 32];
        digit_chars(&mut buf2, offset + 32, 10);

        assert_eq!(buf1, buf2);
    }

    #[test]
    fn ne() {
        let offset = initial_offset();

        let mut buf1 = ['\0'; 32];
        digit_chars(&mut buf1, offset, 10);

        let mut buf2 = ['\0'; 32];
        digit_chars(&mut buf2, offset + 0xF0F0, 10);

        assert_ne!(buf1, buf2);
    }
}
