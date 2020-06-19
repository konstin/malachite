use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_wrapping_pow() {
    fn test<T>(x: T, y: u64, out: T)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.wrapping_pow(y), out);

        let mut x = x;
        x.wrapping_pow_assign(y);
        assert_eq!(x, out);
    };
    test::<u8>(0, 0, 1);
    test::<u64>(123, 0, 1);
    test::<u64>(123, 1, 123);
    test::<u16>(0, 123, 0);
    test::<u16>(1, 123, 1);
    test::<i16>(-1, 123, -1);
    test::<i16>(-1, 124, 1);
    test::<u8>(3, 3, 27);
    test::<i32>(-10, 9, -1_000_000_000);
    test::<i32>(-10, 10, 1_410_065_408);
    test::<i16>(-10, 9, 13_824);
    test::<i16>(10, 9, -13_824);
    test::<i64>(123, 456, 2_409_344_748_064_316_129);
}
