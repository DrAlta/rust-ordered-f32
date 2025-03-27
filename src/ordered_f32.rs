#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct OrderedF32(pub f32);
impl OrderedF32 {
    pub const fn new(numor: i64, denom: u64) -> Self {
        OrderedF32(numor as f32 / denom as f32)

    }
}
