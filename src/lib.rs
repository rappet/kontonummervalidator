// Copyright 2020 Raphael Peters
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

mod error;
pub use error::{KontonummerResult, KontonummerError};

/* Most used methodes:
    2534|00
    2186|88
    1651|06
    1463|63
    1004|10
    890|32
    796|09
    746|13
    680|28
    607|01
    527|76
    503|20
    496|34
    225|38
    213|99
    129|61
    122|A2
    106|03
    58|C7
    55|68
*/

pub fn check_blz(mark: u8, account_number: u64) -> KontonummerResult<()>
{
    match mark {
        0x00 => check_pattern_00(account_number, &[2,1]),
        0x01 => check_pattern_01(account_number, &[3, 7, 1]),
        0x02 => check_pattern_02(account_number, &[2, 3, 4, 5, 6, 7, 8, 9]),
        0x03 => check_pattern_01(account_number, &[2, 1]),
        0x04 => check_pattern_02(account_number, &[2, 3, 4, 5, 6, 7]),
        0x05 => check_pattern_01(account_number, &[7, 3, 1]),
        0x06 => check_pattern_06(account_number, &[2, 3, 4, 5, 6, 7]),
        0x07 => check_pattern_02(account_number, &[2, 3, 4, 5, 6, 7, 8, 9, 10]),
        0x08 => if account_number >= 60000 {
            Ok(())
        } else {
            check_pattern_00(account_number, &[2, 1])
        },
        0x09 => Ok(()),
        0x10 => check_pattern_06(account_number, &[2, 3, 4, 5, 6, 7, 8, 9, 10]),
        0x11 => unimplemented!(),
        _ => Err(KontonummerError::UnknownMark)
    }
}

fn check_pattern_00(mut account_number: u64, pattern: &[u8]) -> KontonummerResult<()> {
    let provided_checksum = account_number % 10;
    account_number /= 10;

    let mut checksum = 0u64;
    while account_number != 0 {
        for weight in pattern {
            checksum += simple_checksum((*weight as u64) * (account_number % 10));
            account_number /= 10;
        }
    }
    checksum %= 10;
    checksum = (10 - checksum) % 10;

    if provided_checksum == checksum {
        Ok(())
    } else {
        Err(KontonummerError::InvalidChecksum)
    }
}

fn check_pattern_01(mut account_number: u64, pattern: &[u8]) -> KontonummerResult<()> {
    let provided_checksum = account_number % 10;
    account_number /= 10;

    let mut checksum = 0u64;
    while account_number != 0 {
        for weight in pattern {
            checksum += (*weight as u64) * (account_number % 10);
            account_number /= 10;
        }
    }
    checksum %= 10;
    checksum = (10 - checksum) % 10;

    if provided_checksum == checksum {
        Ok(())
    } else {
        Err(KontonummerError::InvalidChecksum)
    }
}

fn check_pattern_02(mut account_number: u64, pattern: &[u8]) -> KontonummerResult<()> {
    let provided_checksum = account_number % 10;
    account_number /= 10;

    let mut checksum = 0u64;
    while account_number != 0 {
        for weight in pattern {
            checksum += (*weight as u64) * (account_number % 10);
            account_number /= 10;
        }
    }
    
    checksum %= 11;
    checksum = 11 - checksum;

    match (checksum, provided_checksum) {
        (10, _) => Err(KontonummerError::CalculatedChecksumNotUsable),
        (a, b) if a == b => Ok(()),
        _ => Err(KontonummerError::InvalidChecksum)
    }
}

fn check_pattern_06(mut account_number: u64, pattern: &[u8]) -> KontonummerResult<()> {
    let provided_checksum = account_number % 10;
    account_number /= 10;

    let mut checksum = 0u64;
    while account_number != 0 {
        for weight in pattern {
            checksum += (*weight as u64) * (account_number % 10);
            account_number /= 10;
        }
    }
    
    checksum %= 11;
    checksum = 11 - checksum;

    match (checksum, provided_checksum) {
        (10, 0) => Ok(()),
        (a, b) if a == b => Ok(()),
        _ => Err(KontonummerError::InvalidChecksum)
    }
}

fn simple_checksum(mut number: u64) -> u64 {
    let mut sum = 0;
    while number != 0 {
        sum += number % 10;
        number /= 10;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::check_blz;

    #[test]
    fn test_mark_00() {
        assert_eq!(check_blz(0x00, 9290701), Ok(()));
        assert_eq!(check_blz(0x00, 539290858), Ok(()));
        assert_eq!(check_blz(0x00, 1501824), Ok(()));
        assert_eq!(check_blz(0x00, 1501832), Ok(()));
    }

    #[test]
    fn test_mark_06() {
        assert_eq!(check_blz(0x06, 94012341), Ok(()));
        assert_eq!(check_blz(0x06, 5073321010), Ok(()));
    }

    #[test]
    fn test_mark_10() {
        assert_eq!(check_blz(0x10, 12345008), Ok(()));
        assert_eq!(check_blz(0x10, 87654008), Ok(()));
    }

    #[test]
    fn test_check_pattern_00() {
        use crate::check_pattern_00;

        assert_eq!(check_pattern_00(0,  &[1]), Ok(()));
        assert_eq!(check_pattern_00(118, &[1]), Ok(()));
        assert_eq!(check_pattern_00(884, &[1]), Ok(()));
        assert_eq!(check_pattern_00(886, &[2]), Ok(()));
        assert_eq!(check_pattern_00(885, &[1, 2]), Ok(()));
    }

    #[test]
    fn test_simple_checksum() {
        use crate::simple_checksum;

        assert_eq!(simple_checksum(1), 1);
        assert_eq!(simple_checksum(0), 0);
        assert_eq!(simple_checksum(7), 7);
        assert_eq!(simple_checksum(16), 7);
        assert_eq!(simple_checksum(123), 6);
        assert_eq!(simple_checksum(999), 27);
    }
}
