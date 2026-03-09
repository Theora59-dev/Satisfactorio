#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// +Y
    Above = 0,
    /// -Y
    Below = 1,
    /// -X
    Left  = 2,
    /// +X
    Right = 3,
    /// +Z
    Front = 4,
    /// -Z
    Back  = 5,
}

impl Direction {
    #[inline(always)]
    pub fn from_bits_unchecked(v: u8) -> Self {
        debug_assert!(v < 6);
        unsafe { std::mem::transmute(v) }
    }

    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}