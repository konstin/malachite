use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

impl PartialOrdAbs<Natural> for Float {
    /// Compares the absolute value of a [`Float`] to a
    /// [`Natural`].
    ///
    /// NaN is not comparable to any [`Natural`]. Infinity and negative infinity are greater in
    /// absolute value than any [`Natural`]. Both the [`Float`] zero and the [`Float`] negative
    /// zero are equal to the [`Natural`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Float::from(80).lt_abs(&Natural::from(100u32)));
    /// assert!(Float::INFINITY.gt_abs(&Natural::from(100u32)));
    /// assert!(Float::NEGATIVE_INFINITY.gt_abs(&Natural::from(100u32)));
    /// ```
    fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
        match (self, other) {
            (float_nan!(), _) => None,
            (float_infinity!(), _) | (float_negative_infinity!(), _) => Some(Ordering::Greater),
            (float_either_zero!(), y) => Some(if *y == 0 {
                Ordering::Equal
            } else {
                Ordering::Less
            }),
            (
                Float(Finite {
                    exponent: e_x,
                    significand: x,
                    ..
                }),
                y,
            ) => Some(if *other == 0 {
                Ordering::Greater
            } else if *e_x <= 0 {
                Ordering::Less
            } else {
                e_x.unsigned_abs()
                    .cmp(&other.significant_bits())
                    .then_with(|| x.cmp_normalized(y))
            }),
        }
    }
}

impl PartialOrdAbs<Float> for Natural {
    /// Compares a [`Natural`] to the absolute value of a [`Float`].
    ///
    /// No [`Natural`] is comparable to NaN. Every [`Natural`] is smaller in absolute value than
    /// infinity and negative infinity. The [`Natural`] zero is equal to both the [`Float`] zero
    /// and the [`Float`] negative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(100u32).gt_abs(&Float::from(80)));
    /// assert!(Natural::from(100u32).lt_abs(&Float::INFINITY));
    /// assert!(Natural::from(100u32).lt_abs(&Float::NEGATIVE_INFINITY));
    /// ```
    #[inline]
    fn partial_cmp_abs(&self, other: &Float) -> Option<Ordering> {
        other.partial_cmp_abs(self).map(Ordering::reverse)
    }
}
