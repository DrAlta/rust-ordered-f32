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