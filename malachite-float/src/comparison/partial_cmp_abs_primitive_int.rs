use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

fn float_partial_cmp_abs_unsigned<T: PrimitiveUnsigned>(x: &Float, y: &T) -> Option<Ordering>
where
    Natural: From<T>,
{
    match (x, y) {
        (float_nan!(), _) => None,
        (float_infinity!(), _) | (float_negative_infinity!(), _) => Some(Ordering::Greater),
        (float_either_zero!(), y) => Some(if *y == T::ZERO {
            Ordering::Equal
        } else {
            Ordering::Less
        }),
        (
            Float(Finite {
                exponent: e_x,
                significand: sig_x,
                ..
            }),
            y,
        ) => Some(if *y == T::ZERO {
            Ordering::Greater
        } else if *e_x <= 0 {
            Ordering::Less
        } else {
            e_x.unsigned_abs()
                .cmp(&y.significant_bits())
                .then_with(|| sig_x.cmp_normalized(&Natural::from(*y)))
        }),
    }
}

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Float {
            /// Compares the absolute values of a [`Float`] and an unsigned primitive integer.
            ///
            /// NaN is not comparable to any primitive integer. Infinity and negative infinity are
            /// greater in absolute value than any primitive integer. Both the [`Float`] zero and
            /// the [`Float`] negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                float_partial_cmp_abs_unsigned(self, other)
            }
        }

        impl PartialOrdAbs<Float> for $t {
            /// Compares the absolute values of an unsigned primitive integer and a [`Float`].
            ///
            /// No primitive integer is comparable to NaN. Every primitive integer is smaller in
            /// absolute value than infinity and negative infinity. The integer zero is equal to
            /// both the [`Float`] zero and the [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Float) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

fn float_partial_cmp_abs_signed<T: PrimitiveSigned>(x: &Float, y: &T) -> Option<Ordering>
where
    Natural: From<<T as UnsignedAbs>::Output>,
{
    match (x, y) {
        (float_nan!(), _) => None,
        (float_infinity!(), _) | (float_negative_infinity!(), _) => Some(Ordering::Greater),
        (float_either_zero!(), y) => Some(if *y == T::ZERO {
            Ordering::Equal
        } else {
            Ordering::Less
        }),
        (
            Float(Finite {
                exponent: e_x,
                significand: sig_x,
                ..
            }),
            y,
        ) => Some(if *y == T::ZERO {
            Ordering::Greater
        } else if *e_x <= 0 {
            Ordering::Less
        } else {
            e_x.unsigned_abs()
                .cmp(&y.significant_bits())
                .then_with(|| sig_x.cmp_normalized(&Natural::from(y.unsigned_abs())))
        }),
    }
}

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Float {
            /// Compares the absolute values of a [`Float`] and a signed primitive integer.
            ///
            /// NaN is not comparable to any primitive integer. Infinity and negative infinity are
            /// greater in absolute value than any primitive integer. Both the [`Float`] zero and
            /// the [`Float`] negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                float_partial_cmp_abs_signed(self, other)
            }
        }

        impl PartialOrdAbs<Float> for $t {
            /// Compares the absolute values of a signed primitive integer and a [`Float`].
            ///
            /// No primitive integer is comparable to NaN. Every primitive integer is smaller in
            /// absolute value than infinity and negative infinity. The integer zero is equal to
            /// both the [`Float`] zero and the [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// See [here](super::partial_cmp_abs_primitive_int#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Float) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
