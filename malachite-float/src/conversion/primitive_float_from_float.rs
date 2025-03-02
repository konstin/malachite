use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{significand_bits, Float};
use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, IsPowerOf2, ShrRound};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, RoundingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use std::cmp::Ordering;

fn primitive_float_rounding_from_float<T: PrimitiveFloat>(
    f: Float,
    rm: RoundingMode,
) -> (T, Ordering) {
    match f {
        float_nan!() => (T::NAN, Ordering::Equal),
        float_infinity!() => (T::INFINITY, Ordering::Equal),
        float_negative_infinity!() => (T::NEGATIVE_INFINITY, Ordering::Equal),
        float_zero!() => (T::ZERO, Ordering::Equal),
        float_negative_zero!() => (T::NEGATIVE_ZERO, Ordering::Equal),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let abs_rm = if sign { rm } else { -rm };
            let (x, o) = {
                let exponent = exponent - 1;
                if exponent < T::MIN_EXPONENT {
                    match abs_rm {
                        RoundingMode::Floor | RoundingMode::Down => (T::ZERO, Ordering::Less),
                        RoundingMode::Ceiling | RoundingMode::Up => {
                            (T::MIN_POSITIVE_SUBNORMAL, Ordering::Greater)
                        }
                        RoundingMode::Nearest => {
                            if exponent == T::MIN_EXPONENT - 1 && !significand.is_power_of_2() {
                                (T::MIN_POSITIVE_SUBNORMAL, Ordering::Greater)
                            } else {
                                (T::ZERO, Ordering::Less)
                            }
                        }
                        RoundingMode::Exact => panic!("Float too small for exact conversion"),
                    }
                } else if exponent > T::MAX_EXPONENT {
                    match abs_rm {
                        RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                            (T::MAX_FINITE, Ordering::Less)
                        }
                        RoundingMode::Ceiling | RoundingMode::Up => {
                            (T::INFINITY, Ordering::Greater)
                        }
                        RoundingMode::Exact => panic!("Float too large for exact conversion"),
                    }
                } else {
                    let target_prec = T::max_precision_for_sci_exponent(exponent);
                    let bits = significand_bits(&significand);
                    let (mantissa, o) = significand
                        .shr_round(i64::exact_from(bits) - i64::exact_from(target_prec), abs_rm);
                    let mantissa = u64::wrapping_from(&mantissa);
                    if mantissa.significant_bits() > target_prec {
                        if exponent == T::MAX_EXPONENT {
                            match abs_rm {
                                RoundingMode::Floor
                                | RoundingMode::Down
                                | RoundingMode::Nearest => (T::MAX_FINITE, Ordering::Less),
                                RoundingMode::Ceiling | RoundingMode::Up => {
                                    (T::INFINITY, Ordering::Greater)
                                }

                                RoundingMode::Exact => {
                                    panic!("Float too large for exact conversion")
                                }
                            }
                        } else {
                            (
                                T::from_integer_mantissa_and_exponent(
                                    mantissa >> 1,
                                    exponent - i64::exact_from(target_prec) + 2,
                                )
                                .unwrap(),
                                o,
                            )
                        }
                    } else {
                        (
                            T::from_integer_mantissa_and_exponent(
                                mantissa,
                                exponent - i64::exact_from(target_prec) + 1,
                            )
                            .unwrap(),
                            o,
                        )
                    }
                }
            };
            if sign {
                (x, o)
            } else {
                (-x, o.reverse())
            }
        }
    }
}

fn primitive_float_rounding_from_float_ref<T: PrimitiveFloat>(
    f: &Float,
    rm: RoundingMode,
) -> (T, Ordering) {
    match f {
        float_nan!() => (T::NAN, Ordering::Equal),
        float_infinity!() => (T::INFINITY, Ordering::Equal),
        float_negative_infinity!() => (T::NEGATIVE_INFINITY, Ordering::Equal),
        float_zero!() => (T::ZERO, Ordering::Equal),
        float_negative_zero!() => (T::NEGATIVE_ZERO, Ordering::Equal),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let abs_rm = if *sign { rm } else { -rm };
            let (x, o) = {
                let exponent = exponent - 1;
                if exponent < T::MIN_EXPONENT {
                    match abs_rm {
                        RoundingMode::Floor | RoundingMode::Down => (T::ZERO, Ordering::Less),
                        RoundingMode::Ceiling | RoundingMode::Up => {
                            (T::MIN_POSITIVE_SUBNORMAL, Ordering::Greater)
                        }
                        RoundingMode::Nearest => {
                            if exponent == T::MIN_EXPONENT - 1 && !significand.is_power_of_2() {
                                (T::MIN_POSITIVE_SUBNORMAL, Ordering::Greater)
                            } else {
                                (T::ZERO, Ordering::Less)
                            }
                        }
                        RoundingMode::Exact => panic!("Float too small for exact conversion"),
                    }
                } else if exponent > T::MAX_EXPONENT {
                    match abs_rm {
                        RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                            (T::MAX_FINITE, Ordering::Less)
                        }
                        RoundingMode::Ceiling | RoundingMode::Up => {
                            (T::INFINITY, Ordering::Greater)
                        }
                        RoundingMode::Exact => panic!("Float too large for exact conversion"),
                    }
                } else {
                    let target_prec = T::max_precision_for_sci_exponent(exponent);
                    let bits = significand_bits(significand);
                    let (mantissa, o) = significand
                        .shr_round(i64::exact_from(bits) - i64::exact_from(target_prec), abs_rm);
                    let mantissa = u64::wrapping_from(&mantissa);
                    if mantissa.significant_bits() > target_prec {
                        if exponent == T::MAX_EXPONENT {
                            match abs_rm {
                                RoundingMode::Floor
                                | RoundingMode::Down
                                | RoundingMode::Nearest => (T::MAX_FINITE, Ordering::Less),
                                RoundingMode::Ceiling | RoundingMode::Up => {
                                    (T::INFINITY, Ordering::Greater)
                                }

                                RoundingMode::Exact => {
                                    panic!("Float too large for exact conversion")
                                }
                            }
                        } else {
                            (
                                T::from_integer_mantissa_and_exponent(
                                    mantissa >> 1,
                                    exponent - i64::exact_from(target_prec) + 2,
                                )
                                .unwrap(),
                                o,
                            )
                        }
                    } else {
                        (
                            T::from_integer_mantissa_and_exponent(
                                mantissa,
                                exponent - i64::exact_from(target_prec) + 1,
                            )
                            .unwrap(),
                            o,
                        )
                    }
                }
            };
            if *sign {
                (x, o)
            } else {
                (-x, o.reverse())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FloatFromFloatError {
    Overflow,
    Underflow,
    Inexact,
}

fn primitive_float_try_from_float<T: PrimitiveFloat>(f: Float) -> Result<T, FloatFromFloatError> {
    match f {
        float_nan!() => Ok(T::NAN),
        float_infinity!() => Ok(T::INFINITY),
        float_negative_infinity!() => Ok(T::NEGATIVE_INFINITY),
        float_zero!() => Ok(T::ZERO),
        float_negative_zero!() => Ok(T::NEGATIVE_ZERO),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let x = {
                let exponent = exponent - 1;
                if exponent < T::MIN_EXPONENT {
                    return Err(FloatFromFloatError::Underflow);
                } else if exponent > T::MAX_EXPONENT {
                    return Err(FloatFromFloatError::Overflow);
                } else {
                    let target_prec = T::max_precision_for_sci_exponent(exponent);
                    let bits = significand_bits(&significand);
                    if bits > target_prec
                        && !significand.divisible_by_power_of_2(bits - target_prec)
                    {
                        return Err(FloatFromFloatError::Inexact);
                    }
                    let mantissa = u64::wrapping_from(
                        &(significand >> (i64::exact_from(bits) - i64::exact_from(target_prec))),
                    );
                    T::from_integer_mantissa_and_exponent(
                        mantissa,
                        exponent - i64::exact_from(target_prec) + 1,
                    )
                    .unwrap()
                }
            };
            Ok(if sign { x } else { -x })
        }
    }
}

fn primitive_float_try_from_float_ref<T: PrimitiveFloat>(
    f: &Float,
) -> Result<T, FloatFromFloatError> {
    match f {
        float_nan!() => Ok(T::NAN),
        float_infinity!() => Ok(T::INFINITY),
        float_negative_infinity!() => Ok(T::NEGATIVE_INFINITY),
        float_zero!() => Ok(T::ZERO),
        float_negative_zero!() => Ok(T::NEGATIVE_ZERO),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let x = if *significand == 0u32 {
                T::ZERO
            } else {
                let exponent = exponent - 1;
                if exponent < T::MIN_EXPONENT {
                    return Err(FloatFromFloatError::Underflow);
                } else if exponent > T::MAX_EXPONENT {
                    return Err(FloatFromFloatError::Overflow);
                } else {
                    let target_prec = T::max_precision_for_sci_exponent(exponent);
                    let bits = significand_bits(significand);
                    if bits > target_prec
                        && !significand.divisible_by_power_of_2(bits - target_prec)
                    {
                        return Err(FloatFromFloatError::Inexact);
                    }
                    let mantissa = u64::wrapping_from(
                        &(significand >> (i64::exact_from(bits) - i64::exact_from(target_prec))),
                    );
                    T::from_integer_mantissa_and_exponent(
                        mantissa,
                        exponent - i64::exact_from(target_prec) + 1,
                    )
                    .unwrap()
                }
            };
            Ok(if *sign { x } else { -x })
        }
    }
}

fn primitive_float_convertible_from_float<T: PrimitiveFloat>(f: &Float) -> bool {
    match f {
        Float(Finite {
            exponent,
            significand,
            ..
        }) => {
            let exponent = exponent - 1;
            exponent >= T::MIN_EXPONENT && exponent <= T::MAX_EXPONENT && {
                let target_prec = T::max_precision_for_sci_exponent(exponent);
                let bits = significand_bits(significand);
                bits <= target_prec || significand.divisible_by_power_of_2(bits - target_prec)
            }
        }
        _ => true,
    }
}

macro_rules! impl_primitive_float_from {
    ($t: ident) => {
        impl RoundingFrom<Float> for $t {
            /// Converts a [`Float`] to a primitive float, using a specified [`RoundingMode`] and
            /// taking the [`Float`] by value. An [`Ordering`] is also returned, indicating whether
            /// the returned value is less than, equal to, or greater than the original value.
            /// (Although a NaN is not comparable to anything, converting a NaN to a NaN will also
            /// return `Ordering::Equals`, indicating an exact conversion.)
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the [`Float`] is not exactly equal to any float of the target type, and
            /// `rm` is `Exact`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_float#rounding_from).
            #[inline]
            fn rounding_from(f: Float, rm: RoundingMode) -> ($t, Ordering) {
                primitive_float_rounding_from_float(f, rm)
            }
        }

        impl<'a> RoundingFrom<&'a Float> for $t {
            /// Converts a [`Float`] to a primitive float, using a specified [`RoundingMode`] and
            /// taking the [`Float`] by reference. An [`Ordering`] is also returned, indicating
            /// whether the returned value is less than, equal to, or greater than the original
            /// value. (Although a NaN is not comparable to anything, converting a NaN to a NaN
            /// will also return `Ordering::Equals`, indicating an exact conversion.)
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the [`Float`] is not exactly equal to any float of the target type, and
            /// `rm` is `Exact`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_float#rounding_from).
            #[inline]
            fn rounding_from(f: &'a Float, rm: RoundingMode) -> ($t, Ordering) {
                primitive_float_rounding_from_float_ref(f, rm)
            }
        }

        impl TryFrom<Float> for $t {
            type Error = FloatFromFloatError;

            /// Converts a [`Float`] to a primitive float, taking the [`Float`] by value. If the
            /// [`Float`] is not equal to a primitive float of the given type, an error is
            /// returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_float#try_from).
            #[inline]
            fn try_from(f: Float) -> Result<$t, Self::Error> {
                primitive_float_try_from_float(f)
            }
        }

        impl<'a> TryFrom<&'a Float> for $t {
            type Error = FloatFromFloatError;

            /// Converts a [`Float`] to a primitive float, taking the [`Float`] by reference. If
            /// the [`Float`] is not equal to a primitive float of the given type, an error is
            /// returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_float#try_from).
            #[inline]
            fn try_from(f: &'a Float) -> Result<$t, Self::Error> {
                primitive_float_try_from_float_ref(f)
            }
        }

        impl<'a> ConvertibleFrom<&'a Float> for $t {
            /// Determines whether a [`Float`] can be converted to a primitive float, taking the
            /// [`Float`] by reference.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_float#convertible_from).
            #[inline]
            fn convertible_from(f: &'a Float) -> bool {
                primitive_float_convertible_from_float::<$t>(f)
            }
        }
    };
}
apply_to_primitive_floats!(impl_primitive_float_from);
