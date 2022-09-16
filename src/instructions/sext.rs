pub trait SignExtended<T>: Sized {
    /// Sign-extends to this type from the input type.
    fn sign_extend(_: T) -> Self;
}

impl SignExtended<u8> for u16 {
    fn sign_extend(value: u8) -> Self {
        value as i8 as i16 as u16
    }
}

impl SignExtended<u8> for u32 {
    fn sign_extend(value: u8) -> Self {
        value as i8 as i32 as u32
    }
}

impl SignExtended<u8> for u64 {
    fn sign_extend(value: u8) -> Self {
        value as i8 as i64 as u64
    }
}

impl SignExtended<u16> for u32 {
    fn sign_extend(value: u16) -> Self {
        value as i16 as i32 as u32
    }
}

impl SignExtended<u16> for u64 {
    fn sign_extend(value: u16) -> Self {
        value as i16 as i64 as u64
    }
}

impl SignExtended<u32> for u64 {
    fn sign_extend(value: u32) -> Self {
        value as i32 as i64 as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_to_u16() {
        assert_eq!(u16::sign_extend(0x00u8), 0x00u16);
        assert_eq!(u16::sign_extend(0x01u8), 0x01u16);
        assert_eq!(u16::sign_extend(0x7Fu8), 0x7Fu16);
        assert_eq!(u16::sign_extend(0x80u8), 0xFF80u16);
        assert_eq!(u16::sign_extend(0xFFu8), 0xFFFFu16);
    }

    #[test]
    fn test_u8_to_u32() {
        assert_eq!(u32::sign_extend(0x00u8), 0x00u32);
        assert_eq!(u32::sign_extend(0x01u8), 0x01u32);
        assert_eq!(u32::sign_extend(0x7Fu8), 0x7Fu32);
        assert_eq!(u32::sign_extend(0x80u8), 0xFFFFFF80u32);
        assert_eq!(u32::sign_extend(0xFFu8), 0xFFFFFFFFu32);
    }

    #[test]
    fn test_u8_to_u64() {
        assert_eq!(u64::sign_extend(0x00u8), 0x00u64);
        assert_eq!(u64::sign_extend(0x01u8), 0x01u64);
        assert_eq!(u64::sign_extend(0x7Fu8), 0x7Fu64);
        assert_eq!(u64::sign_extend(0x80u8), 0xFFFFFFFFFFFFFF80u64);
        assert_eq!(u64::sign_extend(0xFFu8), 0xFFFFFFFFFFFFFFFFu64);
    }

    #[test]
    fn test_u16_to_u32() {
        assert_eq!(u32::sign_extend(0x0000u16), 0x0000u32);
        assert_eq!(u32::sign_extend(0x0001u16), 0x0001u32);
        assert_eq!(u32::sign_extend(0x7FFFu16), 0x7FFFu32);
        assert_eq!(u32::sign_extend(0x8000u16), 0xFFFF8000u32);
        assert_eq!(u32::sign_extend(0xFFFFu16), 0xFFFFFFFFu32);
    }

    #[test]
    fn test_u16_to_u64() {
        assert_eq!(u64::sign_extend(0x0000u16), 0x0000u64);
        assert_eq!(u64::sign_extend(0x0001u16), 0x0001u64);
        assert_eq!(u64::sign_extend(0x7FFFu16), 0x7FFFu64);
        assert_eq!(u64::sign_extend(0x8000u16), 0xFFFFFFFFFFFF8000u64);
        assert_eq!(u64::sign_extend(0xFFFFu16), 0xFFFFFFFFFFFFFFFFu64);
    }

    #[test]
    fn test_u32_to_u64() {
        assert_eq!(u64::sign_extend(0x00000000u32), 0x00000000u64);
        assert_eq!(u64::sign_extend(0x00000001u32), 0x00000001u64);
        assert_eq!(u64::sign_extend(0x7FFFFFFFu32), 0x7FFFFFFFu64);
        assert_eq!(u64::sign_extend(0x80000000u32), 0xFFFFFFFF80000000u64);
        assert_eq!(u64::sign_extend(0xFFFFFFFFu32), 0xFFFFFFFFFFFFFFFFu64);
    }
}
