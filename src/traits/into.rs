use crate::OrderedF32;

impl Into<f32> for OrderedF32 {
    fn into(self) -> f32 {
        self.0
    }
}
impl Into<f32> for &OrderedF32 {
    fn into(self) -> f32 {
        self.0
    }
}

impl Into<f64> for OrderedF32 {
    fn into(self) -> f64 {
        self.0 as f64
    }
}
impl Into<f64> for &OrderedF32 {
    fn into(self) -> f64 {
        self.0 as f64
    }
}

impl Into<u8> for OrderedF32 {
    fn into(self) -> u8 {
        self.0 as u8
    }
}
impl Into<u8> for &OrderedF32 {
    fn into(self) -> u8 {
        self.0 as u8
    }
}

impl Into<i8> for OrderedF32 {
    fn into(self) -> i8 {
        self.0 as i8
    }
}
impl Into<i8> for &OrderedF32 {
    fn into(self) -> i8 {
        self.0 as i8
    }
}

impl Into<u16> for OrderedF32 {
    fn into(self) -> u16 {
        self.0 as u16
    }
}
impl Into<u16> for &OrderedF32 {
    fn into(self) -> u16 {
        self.0 as u16
    }
}

impl Into<u32> for OrderedF32 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}
impl Into<u32> for &OrderedF32 {
    fn into(self) -> u32 {
        self.0 as u32
    }
}

impl Into<i32> for OrderedF32 {
    fn into(self) -> i32 {
        self.0 as i32
    }
}
impl Into<i32> for &OrderedF32 {
    fn into(self) -> i32 {
        self.0 as i32
    }
}

impl Into<u64> for OrderedF32 {
    fn into(self) -> u64 {
        self.0 as u64
    }
}
impl Into<u64> for &OrderedF32 {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl Into<i64> for OrderedF32 {
    fn into(self) -> i64 {
        self.0 as i64
    }
}
impl Into<i64> for &OrderedF32 {
    fn into(self) -> i64 {
        self.0 as i64
    }
}

impl Into<u128> for OrderedF32 {
    fn into(self) -> u128 {
        self.0 as u128
    }
}
impl Into<u128> for &OrderedF32 {
    fn into(self) -> u128 {
        self.0 as u128
    }
}

impl Into<i128> for OrderedF32 {
    fn into(self) -> i128 {
        self.0 as i128
    }
}
impl Into<i128> for &OrderedF32 {
    fn into(self) -> i128 {
        self.0 as i128
    }
}

impl Into<usize> for OrderedF32 {
    fn into(self) -> usize {
        self.0 as usize
    }
}
impl Into<usize> for &OrderedF32 {
    fn into(self) -> usize {
        self.0 as usize
    }
}
