use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{
    RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::rounding_modes::RoundingMode;
use std::cmp::Ordering;

impl RoundToMultipleOfPowerOf2<u64> for Integer {
    type Output = Integer;

    /// Rounds an [`Integer`] to a multiple of $2^k$ according to a specified rounding mode. The
    /// [`Integer`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = 2^k \operatorname{sgn}(q) \lfloor |q| \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = 2^k \operatorname{sgn}(q) \lceil |q| \rceil.$
    ///
    /// $f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_of_power_of_2(pow, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by_power_of_2(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), pow / Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10).round_to_multiple_of_power_of_2(2, RoundingMode::Floor)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple_of_power_of_2(2, RoundingMode::Ceiling)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(10).round_to_multiple_of_power_of_2(2, RoundingMode::Down)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).round_to_multiple_of_power_of_2(2, RoundingMode::Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(10).round_to_multiple_of_power_of_2(2, RoundingMode::Nearest)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-12).round_to_multiple_of_power_of_2(2, RoundingMode::Exact)
    ///         .to_debug_string(),
    ///     "(-12, Equal)"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple_of_power_of_2(
        mut self,
        pow: u64,
        rm: RoundingMode,
    ) -> (Integer, Ordering) {
        let o = self.round_to_multiple_of_power_of_2_assign(pow, rm);
        (self, o)
    }
}

impl<'a> RoundToMultipleOfPowerOf2<u64> for &'a Integer {
    type Output = Integer;

    /// Rounds an [`Integer`] to a multiple of $2^k$ according to a specified rounding mode. The
    /// [`Integer`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = 2^k \operatorname{sgn}(q) \lfloor |q| \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = 2^k \operatorname{sgn}(q) \lceil |q| \rceil.$
    ///
    /// $f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_of_power_of_2(pow, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by_power_of_2(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), pow / Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(10)).round_to_multiple_of_power_of_2(2, RoundingMode::Floor)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple_of_power_of_2(2, RoundingMode::Ceiling)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(10)).round_to_multiple_of_power_of_2(2, RoundingMode::Down)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).round_to_multiple_of_power_of_2(2, RoundingMode::Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(10)).round_to_multiple_of_power_of_2(2, RoundingMode::Nearest)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-12)).round_to_multiple_of_power_of_2(2, RoundingMode::Exact)
    ///         .to_debug_string(),
    ///     "(-12, Equal)"
    /// );
    /// ```
    fn round_to_multiple_of_power_of_2(self, pow: u64, rm: RoundingMode) -> (Integer, Ordering) {
        if self.sign {
            let (abs, o) = (&self.abs).round_to_multiple_of_power_of_2(pow, rm);
            (
                Integer {
                    sign: self.sign,
                    abs,
                },
                o,
            )
        } else {
            let (abs, o) = (&self.abs).round_to_multiple_of_power_of_2(pow, -rm);
            (-abs, o.reverse())
        }
    }
}

impl RoundToMultipleOfPowerOf2Assign<u64> for Integer {
    /// Rounds an [`Integer`] to a multiple of $2^k$ in place, according to a specified rounding
    /// mode. An [`Ordering`] is returned, indicating whether the returned value is less than,
    /// equal to, or greater than the original value.
    ///
    /// See the [`RoundToMultipleOfPowerOf2`](RoundToMultipleOfPowerOf2) documentation for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_of_power_of_2_assign(pow, RoundingMode::Exact);`
    /// - `assert!(x.divisible_by_power_of_2(pow));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), pow / Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// let mut n = Integer::from(10);
    /// assert_eq!(
    ///     n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Floor),
    ///     Ordering::Less
    /// );
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(
    ///     n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Ceiling),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(n, -8);
    ///
    /// let mut n = Integer::from(10);
    /// assert_eq!(
    ///     n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Down),
    ///     Ordering::Less
    /// );
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Up), Ordering::Less);
    /// assert_eq!(n, -12);
    ///
    /// let mut n = Integer::from(10);
    /// assert_eq!(
    ///     n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Nearest),
    ///     Ordering::Less
    /// );
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Integer::from(-12);
    /// assert_eq!(
    ///     n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Exact),
    ///     Ordering::Equal
    /// );
    /// assert_eq!(n, -12);
    /// ```
    fn round_to_multiple_of_power_of_2_assign(&mut self, pow: u64, rm: RoundingMode) -> Ordering {
        if self.sign {
            self.abs.round_to_multiple_of_power_of_2_assign(pow, rm)
        } else {
            let o = self.abs.round_to_multiple_of_power_of_2_assign(pow, -rm);
            if self.abs == 0 {
                self.sign = true;
            }
            o.reverse()
        }
    }
}
