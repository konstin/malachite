use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_overflowing_square() {
    fn test<T: PrimitiveInt>(x: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_square(), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_square_assign(), overflow);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, false);
    test::<i16>(1, 1, false);
    test::<u32>(2, 4, false);
    test::<i64>(3, 9, false);
    test::<u128>(10, 100, false);
    test::<isize>(123, 15129, false);
    test::<u32>(1000, 1000000, false);

    test::<i16>(-1, 1, false);
    test::<i32>(-2, 4, false);
    test::<i64>(-3, 9, false);
    test::<i128>(-10, 100, false);
    test::<isize>(-123, 15129, false);
    test::<i32>(-1000, 1000000, false);

    test::<u16>(1000, 16960, true);
    test::<i16>(-1000, 16960, true);
}
