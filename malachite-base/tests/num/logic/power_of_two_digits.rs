use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::PowerOfTwoDigits;

#[test]
pub fn test_to_power_of_two_digits_asc() {
    fn test<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(x: T, log_base: u64, out: &[U])
    where
        T: PowerOfTwoDigits<U>,
    {
        assert_eq!(
            PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&x, log_base),
            out
        );
    };

    test::<u8, u64>(0, 6, &[]);
    test::<u16, u64>(2, 6, &[2]);
    test::<u32, u16>(123, 3, &[3, 7, 1]);
    test::<u32, u8>(1_000_000, 8, &[64, 66, 15]);
    test::<u32, u64>(1_000_000, 8, &[64, 66, 15]);
    test::<u64, u32>(1_000, 1, &[0, 0, 0, 1, 0, 1, 1, 1, 1, 1]);
}

macro_rules! to_power_of_two_digits_asc_helper {
    ($t:ident, $u:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            PowerOfTwoDigits::<$u>::to_power_of_two_digits_asc(&$t::from(100u8), $u::WIDTH + 1);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            PowerOfTwoDigits::<$u>::to_power_of_two_digits_asc(&$t::from(100u8), 0);
        }
    };
}

to_power_of_two_digits_asc_helper!(
    u8,
    u8,
    to_power_of_two_digits_asc_u8_u8_fail_1,
    to_power_of_two_digits_asc_u8_u8_fail_2
);
to_power_of_two_digits_asc_helper!(
    u8,
    u16,
    to_power_of_two_digits_asc_u8_u16_fail_1,
    to_power_of_two_digits_asc_u8_u16_fail_2
);
to_power_of_two_digits_asc_helper!(
    u8,
    u32,
    to_power_of_two_digits_asc_u8_u32_fail_1,
    to_power_of_two_digits_asc_u8_u32_fail_2
);
to_power_of_two_digits_asc_helper!(
    u8,
    u64,
    to_power_of_two_digits_asc_u8_u64_fail_1,
    to_power_of_two_digits_asc_u8_u64_fail_2
);
to_power_of_two_digits_asc_helper!(
    u8,
    u128,
    to_power_of_two_digits_asc_u8_u128_fail_1,
    to_power_of_two_digits_asc_u8_u128_fail_2
);
to_power_of_two_digits_asc_helper!(
    u8,
    usize,
    to_power_of_two_digits_asc_u8_usize_fail_1,
    to_power_of_two_digits_asc_u8_usize_fail_2
);
to_power_of_two_digits_asc_helper!(
    u16,
    u8,
    to_power_of_two_digits_asc_u16_u8_fail_1,
    to_power_of_two_digits_asc_u16_u8_fail_2
);
to_power_of_two_digits_asc_helper!(
    u16,
    u16,
    to_power_of_two_digits_asc_u16_u16_fail_1,
    to_power_of_two_digits_asc_u16_u16_fail_2
);
to_power_of_two_digits_asc_helper!(
    u16,
    u32,
    to_power_of_two_digits_asc_u16_u32_fail_1,
    to_power_of_two_digits_asc_u16_u32_fail_2
);
to_power_of_two_digits_asc_helper!(
    u16,
    u64,
    to_power_of_two_digits_asc_u16_u64_fail_1,
    to_power_of_two_digits_asc_u16_u64_fail_2
);
to_power_of_two_digits_asc_helper!(
    u16,
    u128,
    to_power_of_two_digits_asc_u16_u128_fail_1,
    to_power_of_two_digits_asc_u16_u128_fail_2
);
to_power_of_two_digits_asc_helper!(
    u16,
    usize,
    to_power_of_two_digits_asc_u16_usize_fail_1,
    to_power_of_two_digits_asc_u16_usize_fail_2
);
to_power_of_two_digits_asc_helper!(
    u32,
    u8,
    to_power_of_two_digits_asc_u32_u8_fail_1,
    to_power_of_two_digits_asc_u32_u8_fail_2
);
to_power_of_two_digits_asc_helper!(
    u32,
    u16,
    to_power_of_two_digits_asc_u32_u16_fail_1,
    to_power_of_two_digits_asc_u32_u16_fail_2
);
to_power_of_two_digits_asc_helper!(
    u32,
    u32,
    to_power_of_two_digits_asc_u32_u32_fail_1,
    to_power_of_two_digits_asc_u32_u32_fail_2
);
to_power_of_two_digits_asc_helper!(
    u32,
    u64,
    to_power_of_two_digits_asc_u32_u64_fail_1,
    to_power_of_two_digits_asc_u32_u64_fail_2
);
to_power_of_two_digits_asc_helper!(
    u32,
    u128,
    to_power_of_two_digits_asc_u32_u128_fail_1,
    to_power_of_two_digits_asc_u32_u128_fail_2
);
to_power_of_two_digits_asc_helper!(
    u32,
    usize,
    to_power_of_two_digits_asc_u32_usize_fail_1,
    to_power_of_two_digits_asc_u32_usize_fail_2
);
to_power_of_two_digits_asc_helper!(
    u64,
    u8,
    to_power_of_two_digits_asc_u64_u8_fail_1,
    to_power_of_two_digits_asc_u64_u8_fail_2
);
to_power_of_two_digits_asc_helper!(
    u64,
    u16,
    to_power_of_two_digits_asc_u64_u16_fail_1,
    to_power_of_two_digits_asc_u64_u16_fail_2
);
to_power_of_two_digits_asc_helper!(
    u64,
    u32,
    to_power_of_two_digits_asc_u64_u32_fail_1,
    to_power_of_two_digits_asc_u64_u32_fail_2
);
to_power_of_two_digits_asc_helper!(
    u64,
    u64,
    to_power_of_two_digits_asc_u64_u64_fail_1,
    to_power_of_two_digits_asc_u64_u64_fail_2
);
to_power_of_two_digits_asc_helper!(
    u64,
    u128,
    to_power_of_two_digits_asc_u64_u128_fail_1,
    to_power_of_two_digits_asc_u64_u128_fail_2
);
to_power_of_two_digits_asc_helper!(
    u64,
    usize,
    to_power_of_two_digits_asc_u64_usize_fail_1,
    to_power_of_two_digits_asc_u64_usize_fail_2
);
to_power_of_two_digits_asc_helper!(
    u128,
    u8,
    to_power_of_two_digits_asc_u128_u8_fail_1,
    to_power_of_two_digits_asc_u128_u8_fail_2
);
to_power_of_two_digits_asc_helper!(
    u128,
    u16,
    to_power_of_two_digits_asc_u128_u16_fail_1,
    to_power_of_two_digits_asc_u128_u16_fail_2
);
to_power_of_two_digits_asc_helper!(
    u128,
    u32,
    to_power_of_two_digits_asc_u128_u32_fail_1,
    to_power_of_two_digits_asc_u128_u32_fail_2
);
to_power_of_two_digits_asc_helper!(
    u128,
    u64,
    to_power_of_two_digits_asc_u128_u64_fail_1,
    to_power_of_two_digits_asc_u128_u64_fail_2
);
to_power_of_two_digits_asc_helper!(
    u128,
    u128,
    to_power_of_two_digits_asc_u128_u128_fail_1,
    to_power_of_two_digits_asc_u128_u128_fail_2
);
to_power_of_two_digits_asc_helper!(
    u128,
    usize,
    to_power_of_two_digits_asc_u128_usize_fail_1,
    to_power_of_two_digits_asc_u128_usize_fail_2
);
to_power_of_two_digits_asc_helper!(
    usize,
    u8,
    to_power_of_two_digits_asc_usize_u8_fail_1,
    to_power_of_two_digits_asc_usize_u8_fail_2
);
to_power_of_two_digits_asc_helper!(
    usize,
    u16,
    to_power_of_two_digits_asc_usize_u16_fail_1,
    to_power_of_two_digits_asc_usize_u16_fail_2
);
to_power_of_two_digits_asc_helper!(
    usize,
    u32,
    to_power_of_two_digits_asc_usize_u32_fail_1,
    to_power_of_two_digits_asc_usize_u32_fail_2
);
to_power_of_two_digits_asc_helper!(
    usize,
    u64,
    to_power_of_two_digits_asc_usize_u64_fail_1,
    to_power_of_two_digits_asc_usize_u64_fail_2
);
to_power_of_two_digits_asc_helper!(
    usize,
    u128,
    to_power_of_two_digits_asc_usize_u128_fail_1,
    to_power_of_two_digits_asc_usize_u128_fail_2
);
to_power_of_two_digits_asc_helper!(
    usize,
    usize,
    to_power_of_two_digits_asc_usize_usize_fail_1,
    to_power_of_two_digits_asc_usize_usize_fail_2
);

#[test]
pub fn test_to_power_of_two_digits_desc() {
    fn test<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(x: T, log_base: u64, out: &[U])
    where
        T: PowerOfTwoDigits<U>,
    {
        assert_eq!(
            PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&x, log_base),
            out
        );
    };

    test::<u8, u64>(0, 6, &[]);
    test::<u16, u64>(2, 6, &[2]);
    test::<u32, u16>(123, 3, &[1, 7, 3]);
    test::<u32, u8>(1_000_000, 8, &[15, 66, 64]);
    test::<u32, u64>(1_000_000, 8, &[15, 66, 64]);
    test::<u64, u32>(1_000, 1, &[1, 1, 1, 1, 1, 0, 1, 0, 0, 0]);
}

macro_rules! to_power_of_two_digits_desc_helper {
    ($t:ident, $u:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            PowerOfTwoDigits::<$u>::to_power_of_two_digits_desc(&$t::from(100u8), $u::WIDTH + 1);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            PowerOfTwoDigits::<$u>::to_power_of_two_digits_desc(&$t::from(100u8), 0);
        }
    };
}

to_power_of_two_digits_desc_helper!(
    u8,
    u8,
    to_power_of_two_digits_desc_u8_u8_fail_1,
    to_power_of_two_digits_desc_u8_u8_fail_2
);
to_power_of_two_digits_desc_helper!(
    u8,
    u16,
    to_power_of_two_digits_desc_u8_u16_fail_1,
    to_power_of_two_digits_desc_u8_u16_fail_2
);
to_power_of_two_digits_desc_helper!(
    u8,
    u32,
    to_power_of_two_digits_desc_u8_u32_fail_1,
    to_power_of_two_digits_desc_u8_u32_fail_2
);
to_power_of_two_digits_desc_helper!(
    u8,
    u64,
    to_power_of_two_digits_desc_u8_u64_fail_1,
    to_power_of_two_digits_desc_u8_u64_fail_2
);
to_power_of_two_digits_desc_helper!(
    u8,
    u128,
    to_power_of_two_digits_desc_u8_u128_fail_1,
    to_power_of_two_digits_desc_u8_u128_fail_2
);
to_power_of_two_digits_desc_helper!(
    u8,
    usize,
    to_power_of_two_digits_desc_u8_usize_fail_1,
    to_power_of_two_digits_desc_u8_usize_fail_2
);
to_power_of_two_digits_desc_helper!(
    u16,
    u8,
    to_power_of_two_digits_desc_u16_u8_fail_1,
    to_power_of_two_digits_desc_u16_u8_fail_2
);
to_power_of_two_digits_desc_helper!(
    u16,
    u16,
    to_power_of_two_digits_desc_u16_u16_fail_1,
    to_power_of_two_digits_desc_u16_u16_fail_2
);
to_power_of_two_digits_desc_helper!(
    u16,
    u32,
    to_power_of_two_digits_desc_u16_u32_fail_1,
    to_power_of_two_digits_desc_u16_u32_fail_2
);
to_power_of_two_digits_desc_helper!(
    u16,
    u64,
    to_power_of_two_digits_desc_u16_u64_fail_1,
    to_power_of_two_digits_desc_u16_u64_fail_2
);
to_power_of_two_digits_desc_helper!(
    u16,
    u128,
    to_power_of_two_digits_desc_u16_u128_fail_1,
    to_power_of_two_digits_desc_u16_u128_fail_2
);
to_power_of_two_digits_desc_helper!(
    u16,
    usize,
    to_power_of_two_digits_desc_u16_usize_fail_1,
    to_power_of_two_digits_desc_u16_usize_fail_2
);
to_power_of_two_digits_desc_helper!(
    u32,
    u8,
    to_power_of_two_digits_desc_u32_u8_fail_1,
    to_power_of_two_digits_desc_u32_u8_fail_2
);
to_power_of_two_digits_desc_helper!(
    u32,
    u16,
    to_power_of_two_digits_desc_u32_u16_fail_1,
    to_power_of_two_digits_desc_u32_u16_fail_2
);
to_power_of_two_digits_desc_helper!(
    u32,
    u32,
    to_power_of_two_digits_desc_u32_u32_fail_1,
    to_power_of_two_digits_desc_u32_u32_fail_2
);
to_power_of_two_digits_desc_helper!(
    u32,
    u64,
    to_power_of_two_digits_desc_u32_u64_fail_1,
    to_power_of_two_digits_desc_u32_u64_fail_2
);
to_power_of_two_digits_desc_helper!(
    u32,
    u128,
    to_power_of_two_digits_desc_u32_u128_fail_1,
    to_power_of_two_digits_desc_u32_u128_fail_2
);
to_power_of_two_digits_desc_helper!(
    u32,
    usize,
    to_power_of_two_digits_desc_u32_usize_fail_1,
    to_power_of_two_digits_desc_u32_usize_fail_2
);
to_power_of_two_digits_desc_helper!(
    u64,
    u8,
    to_power_of_two_digits_desc_u64_u8_fail_1,
    to_power_of_two_digits_desc_u64_u8_fail_2
);
to_power_of_two_digits_desc_helper!(
    u64,
    u16,
    to_power_of_two_digits_desc_u64_u16_fail_1,
    to_power_of_two_digits_desc_u64_u16_fail_2
);
to_power_of_two_digits_desc_helper!(
    u64,
    u32,
    to_power_of_two_digits_desc_u64_u32_fail_1,
    to_power_of_two_digits_desc_u64_u32_fail_2
);
to_power_of_two_digits_desc_helper!(
    u64,
    u64,
    to_power_of_two_digits_desc_u64_u64_fail_1,
    to_power_of_two_digits_desc_u64_u64_fail_2
);
to_power_of_two_digits_desc_helper!(
    u64,
    u128,
    to_power_of_two_digits_desc_u64_u128_fail_1,
    to_power_of_two_digits_desc_u64_u128_fail_2
);
to_power_of_two_digits_desc_helper!(
    u64,
    usize,
    to_power_of_two_digits_desc_u64_usize_fail_1,
    to_power_of_two_digits_desc_u64_usize_fail_2
);
to_power_of_two_digits_desc_helper!(
    u128,
    u8,
    to_power_of_two_digits_desc_u128_u8_fail_1,
    to_power_of_two_digits_desc_u128_u8_fail_2
);
to_power_of_two_digits_desc_helper!(
    u128,
    u16,
    to_power_of_two_digits_desc_u128_u16_fail_1,
    to_power_of_two_digits_desc_u128_u16_fail_2
);
to_power_of_two_digits_desc_helper!(
    u128,
    u32,
    to_power_of_two_digits_desc_u128_u32_fail_1,
    to_power_of_two_digits_desc_u128_u32_fail_2
);
to_power_of_two_digits_desc_helper!(
    u128,
    u64,
    to_power_of_two_digits_desc_u128_u64_fail_1,
    to_power_of_two_digits_desc_u128_u64_fail_2
);
to_power_of_two_digits_desc_helper!(
    u128,
    u128,
    to_power_of_two_digits_desc_u128_u128_fail_1,
    to_power_of_two_digits_desc_u128_u128_fail_2
);
to_power_of_two_digits_desc_helper!(
    u128,
    usize,
    to_power_of_two_digits_desc_u128_usize_fail_1,
    to_power_of_two_digits_desc_u128_usize_fail_2
);
to_power_of_two_digits_desc_helper!(
    usize,
    u8,
    to_power_of_two_digits_desc_usize_u8_fail_1,
    to_power_of_two_digits_desc_usize_u8_fail_2
);
to_power_of_two_digits_desc_helper!(
    usize,
    u16,
    to_power_of_two_digits_desc_usize_u16_fail_1,
    to_power_of_two_digits_desc_usize_u16_fail_2
);
to_power_of_two_digits_desc_helper!(
    usize,
    u32,
    to_power_of_two_digits_desc_usize_u32_fail_1,
    to_power_of_two_digits_desc_usize_u32_fail_2
);
to_power_of_two_digits_desc_helper!(
    usize,
    u64,
    to_power_of_two_digits_desc_usize_u64_fail_1,
    to_power_of_two_digits_desc_usize_u64_fail_2
);
to_power_of_two_digits_desc_helper!(
    usize,
    u128,
    to_power_of_two_digits_desc_usize_u128_fail_1,
    to_power_of_two_digits_desc_usize_u128_fail_2
);
to_power_of_two_digits_desc_helper!(
    usize,
    usize,
    to_power_of_two_digits_desc_usize_usize_fail_1,
    to_power_of_two_digits_desc_usize_usize_fail_2
);
