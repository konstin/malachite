use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{ComparableFloat, ComparableFloatRef, Float};
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use std::cmp::Ordering;

impl PartialOrdAbs for Float {
    /// Compares the absolute values of two [`Float`]s.
    ///
    /// This implementation follows the IEEE 754 standard. `NaN` is not comparable to anything, not
    /// even itself. [`Float`]s with different precisions are equal if they represent the same
    /// numeric value.
    ///
    /// For different comparison behavior that provides a total order, consider using
    /// [`ComparableFloat`] or [`ComparableFloatRef`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Zero
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Float::NAN.partial_cmp_abs(&Float::NAN), None);
    /// assert_eq!(Float::ZERO.partial_cmp_abs(&Float::NEGATIVE_ZERO), Some(Ordering::Equal));
    /// assert_eq!(Float::ONE.partial_cmp_abs(&Float::one_prec(100)), Some(Ordering::Equal));
    /// assert!(Float::INFINITY.gt_abs(&Float::ONE));
    /// assert!(Float::NEGATIVE_INFINITY.gt_abs(&Float::ONE));
    /// assert!(Float::ONE_HALF.lt_abs(&Float::ONE));
    /// assert!(Float::ONE_HALF.lt_abs(&Float::NEGATIVE_ONE));
    /// ```
    fn partial_cmp_abs(&self, other: &Float) -> Option<Ordering> {
        match (self, other) {
            (float_nan!(), _) | (_, float_nan!()) => None,
            (float_either_infinity!(), float_either_infinity!())
            | (float_either_zero!(), float_either_zero!()) => Some(Ordering::Equal),
            (float_either_infinity!(), _) | (_, float_either_zero!()) => Some(Ordering::Greater),
            (_, float_either_infinity!()) | (float_either_zero!(), _) => Some(Ordering::Less),
            (
                Float(Finite {
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                Float(Finite {
                    exponent: e_y,
                    significand: y,
                    ..
                }),
            ) => Some(e_x.cmp(e_y).then_with(|| x.cmp_normalized_no_shift(y))),
        }
    }
}

impl<'a> OrdAbs for ComparableFloatRef<'a> {
    /// Compares the absolute values of two [`ComparableFloatRef`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how
    /// [`ComparableFloatRef`]s are ordered by absolute value, from least to greatest:
    ///   - NaN
    ///   - Positive and negative zero
    ///   - Nonzero finite floats
    ///   - Positive and negative infinity
    ///
    /// For different comparison behavior that follows the IEEE 754 standard, consider just using
    /// [`Float`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Zero
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::{ComparableFloatRef, Float};
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::NAN).partial_cmp_abs(&ComparableFloatRef(&Float::NAN)),
    ///     Some(Ordering::Equal)
    /// );
    /// assert_eq!(
    ///     ComparableFloatRef(&Float::ZERO)
    ///         .partial_cmp_abs(&ComparableFloatRef(&Float::NEGATIVE_ZERO)),
    ///     Some(Ordering::Equal)
    /// );
    /// assert!(ComparableFloatRef(&Float::ONE).lt_abs(&ComparableFloatRef(&Float::one_prec(100))));
    /// assert!(ComparableFloatRef(&Float::INFINITY).gt_abs(&ComparableFloatRef(&Float::ONE)));
    /// assert!(
    ///     ComparableFloatRef(&Float::NEGATIVE_INFINITY).gt_abs(&ComparableFloatRef(&Float::ONE))
    /// );
    /// assert!(ComparableFloatRef(&Float::ONE_HALF).lt_abs(&ComparableFloatRef(&Float::ONE)));
    /// assert!(
    ///     ComparableFloatRef(&Float::ONE_HALF).lt_abs(&ComparableFloatRef(&Float::NEGATIVE_ONE))
    /// );
    /// ```
    #[allow(clippy::match_same_arms)]
    fn cmp_abs(&self, other: &ComparableFloatRef<'a>) -> Ordering {
        match (&self.0, &other.0) {
            (float_nan!(), float_nan!())
            | (float_either_infinity!(), float_either_infinity!())
            | (float_either_zero!(), float_either_zero!()) => Ordering::Equal,
            (float_either_infinity!(), _) | (_, float_nan!()) => Ordering::Greater,
            (_, float_either_infinity!()) | (float_nan!(), _) => Ordering::Less,
            (float_either_zero!(), _) => Ordering::Less,
            (_, float_either_zero!()) => Ordering::Greater,
            (
                Float(Finite {
                    exponent: e_x,
                    precision: p_x,
                    significand: x,
                    ..
                }),
                Float(Finite {
                    exponent: e_y,
                    precision: p_y,
                    significand: y,
                    ..
                }),
            ) => e_x
                .cmp(e_y)
                .then_with(|| x.cmp_normalized_no_shift(y))
                .then_with(|| p_x.cmp(p_y)),
        }
    }
}

impl<'a> PartialOrdAbs for ComparableFloatRef<'a> {
    /// Compares the absolute values of two [`ComparableFloatRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &ComparableFloatRef) -> Option<Ordering> {
        Some(self.cmp_abs(other))
    }
}

impl OrdAbs for ComparableFloat {
    /// Compares the absolute values of two [`ComparableFloat`]s.
    ///
    /// This implementation does not follow the IEEE 754 standard. This is how
    /// [`ComparableFloat`]s are ordered by absolute value, from least to greatest:
    ///   - NaN
    ///   - Positive and negative zero
    ///   - Nonzero finite floats
    ///   - Positive and negative infinity
    ///
    /// For different comparison behavior that follows the IEEE 754 standard, consider just using
    /// [`Float`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Zero
    /// };
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::{ComparableFloat, Float};
    /// use std::cmp::Ordering;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     ComparableFloat(Float::NAN).partial_cmp_abs(&ComparableFloat(Float::NAN)),
    ///     Some(Ordering::Equal)
    /// );
    /// assert_eq!(
    ///     ComparableFloat(Float::ZERO).partial_cmp_abs(&ComparableFloat(Float::NEGATIVE_ZERO)),
    ///     Some(Ordering::Equal)
    /// );
    /// assert!(ComparableFloat(Float::ONE).lt_abs(&ComparableFloat(Float::one_prec(100))));
    /// assert!(ComparableFloat(Float::INFINITY).gt_abs(&ComparableFloat(Float::ONE)));
    /// assert!(ComparableFloat(Float::NEGATIVE_INFINITY).gt_abs(&ComparableFloat(Float::ONE)));
    /// assert!(ComparableFloat(Float::ONE_HALF).lt_abs(&ComparableFloat(Float::ONE)));
    /// assert!(ComparableFloat(Float::ONE_HALF).lt_abs(&ComparableFloat(Float::NEGATIVE_ONE)));
    /// ```
    #[inline]
    fn cmp_abs(&self, other: &ComparableFloat) -> Ordering {
        self.as_ref().cmp_abs(&other.as_ref())
    }
}

impl PartialOrdAbs for ComparableFloat {
    /// Compares the absolute values of two [`ComparableFloatRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp_abs(&self, other: &ComparableFloat) -> Option<Ordering> {
        Some(self.as_ref().cmp_abs(&other.as_ref()))
    }
}
