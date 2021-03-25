use malachite_base::num::float::PrimitiveFloat;
use std::panic::catch_unwind;

#[allow(clippy::approx_constant, clippy::decimal_literal_representation)]
#[test]
pub fn test_to_ordered_representation() {
    fn test<T: PrimitiveFloat>(x: T, out: T::UnsignedOfEqualWidth) {
        assert_eq!(x.to_ordered_representation(), out);
    }
    test::<f32>(f32::NEGATIVE_INFINITY, 0);
    test::<f32>(-f32::MAX_FINITE, 1);
    test::<f32>(-458.42188, 1000000000);
    test::<f32>(-10.0, 1046478848);
    test::<f32>(-core::f32::consts::PI, 1060565029);
    test::<f32>(-1.0, 1073741824);
    test::<f32>(-0.1, 1102263091);
    test::<f32>(-f32::MIN_POSITIVE_NORMAL, 2130706432);
    test::<f32>(-f32::MAX_SUBNORMAL, 2130706433);
    test::<f32>(-f32::MIN_POSITIVE_SUBNORMAL, 2139095039);
    test::<f32>(-0.0, 2139095040);
    test::<f32>(0.0, 2139095041);
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, 2139095042);
    test::<f32>(f32::MAX_SUBNORMAL, 2147483648);
    test::<f32>(f32::MIN_POSITIVE_NORMAL, 2147483649);
    test::<f32>(0.1, 3175926990);
    test::<f32>(0.99999994, 3204448256);
    test::<f32>(1.0, 3204448257);
    test::<f32>(1.0000001, 3204448258);
    test::<f32>(3.1415925, 3217625051);
    test::<f32>(core::f32::consts::PI, 3217625052);
    test::<f32>(3.141593, 3217625053);
    test::<f32>(10.0, 3231711233);
    test::<f32>(f32::MAX_FINITE, 4278190080);
    test::<f32>(f32::POSITIVE_INFINITY, 4278190081);

    test::<f64>(f64::NEGATIVE_INFINITY, 0);
    test::<f64>(-f64::MAX_FINITE, 1);
    test::<f64>(-10.0, 4597049319638433792);
    test::<f64>(-core::f64::consts::PI, 4604611780675359464);
    test::<f64>(-1.0, 4611686018427387904);
    test::<f64>(-0.1, 4626998257160447590);
    test::<f64>(-f64::MIN_POSITIVE_NORMAL, 9214364837600034816);
    test::<f64>(-f64::MAX_SUBNORMAL, 9214364837600034817);
    test::<f64>(-f64::MIN_POSITIVE_SUBNORMAL, 9218868437227405311);
    test::<f64>(-0.0, 9218868437227405312);
    test::<f64>(0.0, 9218868437227405313);
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, 9218868437227405314);
    test::<f64>(f64::MAX_SUBNORMAL, 9223372036854775808);
    test::<f64>(f64::MIN_POSITIVE_NORMAL, 9223372036854775809);
    test::<f64>(1.9261352099337372e-256, 10000000000000000000);
    test::<f64>(0.1, 13810738617294363035);
    test::<f64>(0.9999999999999999, 13826050856027422720);
    test::<f64>(1.0, 13826050856027422721);
    test::<f64>(1.0000000000000002, 13826050856027422722);
    test::<f64>(3.1415926535897927, 13833125093779451160);
    test::<f64>(core::f64::consts::PI, 13833125093779451161);
    test::<f64>(3.1415926535897936, 13833125093779451162);
    test::<f64>(10.0, 13840687554816376833);
    test::<f64>(f64::MAX_FINITE, 18437736874454810624);
    test::<f64>(f64::POSITIVE_INFINITY, 18437736874454810625);
}

fn to_ordered_representation_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::NAN.to_ordered_representation());
}

#[test]
pub fn to_ordered_representation_fail() {
    apply_fn_to_primitive_floats!(to_ordered_representation_fail_helper);
}
