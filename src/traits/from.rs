use crate::OrderedF32;

impl From<f32> for OrderedF32{
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<&f32> for OrderedF32{
    fn from(value: &f32) -> Self {
        Self(value.clone())
    }
}

impl From<&mut f32> for OrderedF32{
    fn from(value: &mut f32) -> Self {
        Self(value.clone())
    }
}

impl From<f64> for OrderedF32{
    fn from(value: f64) -> Self {
        Self(value as f32)
    }
}

impl From<&f64> for OrderedF32{
    fn from(value: &f64) -> Self {
        Self(value.clone() as f32)
    }
}

impl From<&mut f64> for OrderedF32{
    fn from(value: &mut f64) -> Self {
        Self(value.clone() as f32)
    }
}

impl From<i32> for OrderedF32{
    fn from(value: i32) -> Self {
        Self(value as f32)
    }
}
impl From<i64> for OrderedF32{
    fn from(value: i64) -> Self {
        Self(value as f32)
    }
}
impl From<i128> for OrderedF32{
    fn from(value: i128) -> Self {
        Self(value as f32)
    }
}


impl From<u32> for OrderedF32{
    fn from(value: u32) -> Self {
        Self(value as f32)
    }
}
impl From<u64> for OrderedF32{
    fn from(value: u64) -> Self {
        Self(value as f32)
    }
}
impl From<u128> for OrderedF32{
    fn from(value: u128) -> Self {
        Self(value as f32)
    }
}



