pub type KontonummerResult<T> = Result<T, KontonummerError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KontonummerError {
    UnknownMark,
    InvalidChecksum,
    // see 02
    CalculatedChecksumNotUsable,
}

impl Into<usize> for KontonummerError {
    fn into(self) -> usize {
        use KontonummerError::*;
        match self {
            UnknownMark                 => 0x01,
            InvalidChecksum             => 0x02,
            CalculatedChecksumNotUsable => 0x03,
        }
    }
}
