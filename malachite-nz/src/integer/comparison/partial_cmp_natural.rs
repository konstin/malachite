use crate::integer::Integer;
use crate::natural::Natural;
use std::cmp::Ordering;

impl PartialOrd<Natural> for Integer {
    /// Compares an [`Integer`] to a [`Natural`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Integer::from(123) > Natural::from(122u32));
    /// assert!(Integer::from(123) >= Natural::from(122u32));
    /// assert!(Integer::from(123) < Natural::from(124u32));
    /// assert!(Integer::from(123) <= Natural::from(124u32));
    /// assert!(Integer::from(-123) < Natural::from(123u32));
    /// assert!(Integer::from(-123) <= Natural::from(123u32));
    /// ```
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        if self.sign {
            self.abs.partial_cmp(other)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl PartialOrd<Integer> for Natural {
    /// Compares a [`Natural`] to an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(123u32) > Integer::from(122));
    /// assert!(Natural::from(123u32) >= Integer::from(122));
    /// assert!(Natural::from(123u32) < Integer::from(124));
    /// assert!(Natural::from(123u32) <= Integer::from(124));
    /// assert!(Natural::from(123u32) > Integer::from(-123));
    /// assert!(Natural::from(123u32) >= Integer::from(-123));
    /// ```
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        if other.sign {
            self.partial_cmp(&other.abs)
        } else {
            Some(Ordering::Greater)
        }
    }
}
