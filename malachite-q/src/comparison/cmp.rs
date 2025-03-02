use crate::Rational;
use malachite_base::num::arithmetic::traits::Sign;
use std::cmp::Ordering;

impl PartialOrd for Rational {
    /// Compares two [`Rational`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    /// Compares two [`Rational`]s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Rational::from_str("2/3").unwrap() > Rational::ONE_HALF);
    /// assert!(Rational::from_str("-2/3").unwrap() < Rational::ONE_HALF);
    /// ```
    fn cmp(&self, other: &Rational) -> Ordering {
        if std::ptr::eq(self, other) {
            return Ordering::Equal;
        }
        // First check signs
        let self_sign = self.sign();
        let other_sign = other.sign();
        let sign_cmp = self_sign.cmp(&other_sign);
        if sign_cmp != Ordering::Equal || self_sign == Ordering::Equal {
            return sign_cmp;
        }
        // Then check if one is < 1 and the other is > 1
        let self_cmp_one = self.numerator.cmp(&self.denominator);
        let other_cmp_one = other.numerator.cmp(&other.denominator);
        let one_cmp = self_cmp_one.cmp(&other_cmp_one);
        if one_cmp != Ordering::Equal {
            return if self.sign {
                one_cmp
            } else {
                one_cmp.reverse()
            };
        }
        // Then compare numerators and denominators
        let n_cmp = self.numerator.cmp(&other.numerator);
        let d_cmp = self.denominator.cmp(&other.denominator);
        if n_cmp == Ordering::Equal && d_cmp == Ordering::Equal {
            return Ordering::Equal;
        } else {
            let nd_cmp = n_cmp.cmp(&d_cmp);
            if nd_cmp != Ordering::Equal {
                return if self.sign { nd_cmp } else { nd_cmp.reverse() };
            }
        }
        // Then compare floor ∘ log_2 ∘ abs
        let log_cmp = self
            .floor_log_base_2_abs()
            .cmp(&other.floor_log_base_2_abs());
        if log_cmp != Ordering::Equal {
            return if self.sign {
                log_cmp
            } else {
                log_cmp.reverse()
            };
        }
        // Finally, cross-multiply.
        let prod_cmp =
            (&self.numerator * &other.denominator).cmp(&(&self.denominator * &other.numerator));
        if self.sign {
            prod_cmp
        } else {
            prod_cmp.reverse()
        }
    }
}
