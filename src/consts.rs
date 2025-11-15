use crate::OrderedF32;
impl OrderedF32 {
    pub const NEG_TEN: OrderedF32 = OrderedF32(-10.0);
    pub const NEG_NINE: OrderedF32 = OrderedF32(-9.0);
    pub const NEG_EIGHT: OrderedF32 = OrderedF32(-8.0);
    pub const NEG_SEVEN: OrderedF32 = OrderedF32(-7.0);
    pub const NEG_SIX: OrderedF32 = OrderedF32(-6.0);
    pub const NEG_FIVE: OrderedF32 = OrderedF32(-5.0);
    pub const NEG_FOUR: OrderedF32 = OrderedF32(-4.0);
    pub const NEG_THREE: OrderedF32 = OrderedF32(-3.0);
    pub const NEG_TWO: OrderedF32 = OrderedF32(-2.0);
    pub const NEG_FOUR_THIRDS: OrderedF32 = OrderedF32(-4.0 / 3.0);
    pub const NEG_ONE: OrderedF32 = OrderedF32(-1.0);
    pub const NEG_TWO_THIRDS: OrderedF32 = OrderedF32(-2.0 / 3.0);
    pub const NEG_THIRD: OrderedF32 = OrderedF32(-1.0 / 3.0);

    pub const ZERO: OrderedF32 = OrderedF32(0.0);

    pub const FOURTH: OrderedF32 = OrderedF32(1.0 / 4.0);
    pub const THIRD: OrderedF32 = OrderedF32(1.0 / 3.0);
    pub const HALF: OrderedF32 = OrderedF32(0.5);
    pub const TWO_THIRDS: OrderedF32 = OrderedF32(2.0 / 3.0);
    pub const ONE: OrderedF32 = OrderedF32(1.0);
    pub const FOUR_THIRDS: OrderedF32 = OrderedF32(4.0 / 3.0);
    pub const TWO: OrderedF32 = OrderedF32(2.0);
    pub const THREE: OrderedF32 = OrderedF32(3.0);
    pub const FOUR: OrderedF32 = OrderedF32(4.0);
    pub const FIVE: OrderedF32 = OrderedF32(5.0);
    pub const SIX: OrderedF32 = OrderedF32(6.0);
    pub const SEVEN: OrderedF32 = OrderedF32(7.0);
    pub const EIGHT: OrderedF32 = OrderedF32(8.0);
    pub const NINE: OrderedF32 = OrderedF32(9.0);
    pub const TEN: OrderedF32 = OrderedF32(10.0);

    pub const E: OrderedF32 = OrderedF32(std::f32::consts::E);
    pub const FRAC_2_PI: OrderedF32 = OrderedF32(std::f32::consts::FRAC_2_PI);
    pub const FRAC_PI_2: OrderedF32 = OrderedF32(std::f32::consts::FRAC_PI_2);
    pub const FRAC_PI_4: OrderedF32 = OrderedF32(std::f32::consts::FRAC_PI_4);
    pub const FRAC_PI_6: OrderedF32 = OrderedF32(std::f32::consts::FRAC_PI_6);
    pub const FRAC_PI_8: OrderedF32 = OrderedF32(std::f32::consts::FRAC_PI_8);
    pub const PI: OrderedF32 = OrderedF32(std::f32::consts::PI);
    pub const SQRT_2: OrderedF32 = OrderedF32(std::f32::consts::SQRT_2);
    pub const TAU: OrderedF32 = OrderedF32(std::f32::consts::TAU);
    pub const MAX: OrderedF32 = OrderedF32(std::f32::MAX);
    pub const MIN: OrderedF32 = OrderedF32(std::f32::MIN);
}
