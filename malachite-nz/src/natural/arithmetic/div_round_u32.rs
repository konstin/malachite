use malachite_base::num::{
    BitAccess, DivAssignMod, DivMod, DivRound, DivRoundAssign, Parity, PrimitiveInteger,
};
use malachite_base::round::RoundingMode;
use natural::Natural::{self, Large, Small};

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of a `u32` divided by the `Natural` and rounded according to a specified
/// `RoundingMode`. The limb slice must have at least two elements and cannot have any trailing
/// zeros.
///
/// This function returns a `None` iff the rounding mode is `Exact` but the remainder of the
/// division would be nonzero.
///
/// Note that this function may only return `None`, `Some(0)`, or `Some(1)` because of the
/// restrictions placed on the input slice.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_nz::natural::arithmetic::div_round_u32::limbs_limb_div_round_limbs;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Down), Some(0));
///     assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Floor), Some(0));
///     assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Up), Some(1));
///     assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Ceiling), Some(1));
///     assert_eq!(limbs_limb_div_round_limbs(0, &[123, 456], RoundingMode::Exact), Some(0));
///     assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Exact), None);
///     assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Nearest), Some(0));
///     assert_eq!(limbs_limb_div_round_limbs(0xffff_ffff, &[123, 1], RoundingMode::Nearest),
///         Some(1));
///     assert_eq!(limbs_limb_div_round_limbs(0xffff_ffff, &[0xffff_ffff, 1],
///         RoundingMode::Nearest), Some(0));
///
///     assert_eq!(limbs_limb_div_round_limbs(0xffff_ffff, &[0xffff_fffe, 1],
///         RoundingMode::Nearest), Some(0));
///
///     assert_eq!(limbs_limb_div_round_limbs(0xffff_ffff, &[0xffff_fffd, 1],
///         RoundingMode::Nearest), Some(1));
/// }
/// ```
pub fn limbs_limb_div_round_limbs(limb: u32, limbs: &[u32], rm: RoundingMode) -> Option<u32> {
    if limb == 0 {
        Some(0)
    } else {
        match rm {
            RoundingMode::Down | RoundingMode::Floor => Some(0),
            RoundingMode::Up | RoundingMode::Ceiling => Some(1),
            RoundingMode::Exact => None,
            // 1 if 2 * limb > Natural::from_limbs_asc(limbs); otherwise, 0
            RoundingMode::Nearest => Some(if limbs.len() == 2
                && limbs[1] == 1
                && limb.get_bit(u64::from(u32::WIDTH) - 1)
                && (limb << 1) > limbs[0]
            {
                1
            } else {
                0
            }),
        }
    }
}

impl DivRound<u32> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `u32` and rounds according to a specified rounding mode, taking the
    /// `Natural` by value. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(10u32).div_round(4, RoundingMode::Down).to_string(), "2");
    ///     assert_eq!(Natural::trillion().div_round(3, RoundingMode::Floor).to_string(),
    ///         "333333333333");
    ///     assert_eq!(Natural::from(10u32).div_round(4, RoundingMode::Up).to_string(), "3");
    ///     assert_eq!(Natural::trillion().div_round(3, RoundingMode::Ceiling).to_string(),
    ///         "333333333334");
    ///     assert_eq!(Natural::from(10u32).div_round(5, RoundingMode::Exact).to_string(), "2");
    ///     assert_eq!(Natural::from(10u32).div_round(3, RoundingMode::Nearest).to_string(), "3");
    ///     assert_eq!(Natural::from(20u32).div_round(3, RoundingMode::Nearest).to_string(), "7");
    ///     assert_eq!(Natural::from(10u32).div_round(4, RoundingMode::Nearest).to_string(), "2");
    ///     assert_eq!(Natural::from(14u32).div_round(4, RoundingMode::Nearest).to_string(), "4");
    /// }
    /// ```
    fn div_round(mut self, other: u32, rm: RoundingMode) -> Natural {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<u32> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `u32` and rounds according to a specified rounding mode, taking the
    /// `Natural` by reference. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!((&Natural::from(10u32)).div_round(4, RoundingMode::Down).to_string(), "2");
    ///     assert_eq!((&Natural::trillion()).div_round(3, RoundingMode::Floor).to_string(),
    ///         "333333333333");
    ///     assert_eq!((&Natural::from(10u32)).div_round(4, RoundingMode::Up).to_string(), "3");
    ///     assert_eq!((&Natural::trillion()).div_round(3, RoundingMode::Ceiling).to_string(),
    ///         "333333333334");
    ///     assert_eq!((&Natural::from(10u32)).div_round(5, RoundingMode::Exact).to_string(),
    ///         "2");
    ///     assert_eq!((&Natural::from(10u32)).div_round(3, RoundingMode::Nearest).to_string(),
    ///         "3");
    ///     assert_eq!((&Natural::from(20u32)).div_round(3, RoundingMode::Nearest).to_string(),
    ///         "7");
    ///     assert_eq!((&Natural::from(10u32)).div_round(4, RoundingMode::Nearest).to_string(),
    ///         "2");
    ///     assert_eq!((&Natural::from(14u32)).div_round(4, RoundingMode::Nearest).to_string(),
    ///         "4");
    /// }
    /// ```
    fn div_round(self, other: u32, rm: RoundingMode) -> Natural {
        if rm == RoundingMode::Down || rm == RoundingMode::Floor {
            self / other
        } else {
            let (quotient, remainder) = self.div_mod(other);
            match rm {
                _ if remainder == 0 => quotient,
                RoundingMode::Up | RoundingMode::Ceiling => quotient + 1,
                RoundingMode::Nearest => {
                    let shifted_other = other >> 1;
                    if remainder > shifted_other
                        || remainder == shifted_other && other.is_even() && quotient.is_odd()
                    {
                        quotient + 1
                    } else {
                        quotient
                    }
                }
                RoundingMode::Exact => {
                    panic!("Division is not exact: {} / {}", self, other);
                }
                _ => unreachable!(),
            }
        }
    }
}

impl DivRound<Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural` and rounds according to a specified rounding mode, taking the
    /// `Natural` by value. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(10.div_round(Natural::from(4u32), RoundingMode::Down), 2);
    ///     assert_eq!(1000.div_round(Natural::trillion(), RoundingMode::Floor), 0);
    ///     assert_eq!(10.div_round(Natural::from(4u32), RoundingMode::Up), 3);
    ///     assert_eq!(1000.div_round(Natural::trillion(), RoundingMode::Ceiling), 1);
    ///     assert_eq!(10.div_round(Natural::from(5u32), RoundingMode::Exact), 2);
    ///     assert_eq!(10.div_round(Natural::from(3u32), RoundingMode::Nearest), 3);
    ///     assert_eq!(20.div_round(Natural::from(3u32), RoundingMode::Nearest), 7);
    ///     assert_eq!(10.div_round(Natural::from(4u32), RoundingMode::Nearest), 2);
    ///     assert_eq!(14.div_round(Natural::from(4u32), RoundingMode::Nearest), 4);
    /// }
    /// ```
    fn div_round(self, other: Natural, rm: RoundingMode) -> u32 {
        self.div_round(&other, rm)
    }
}

impl<'a> DivRound<&'a Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural` and rounds according to a specified rounding mode, taking the
    /// `Natural` by reference. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(10.div_round(&Natural::from(4u32), RoundingMode::Down), 2);
    ///     assert_eq!(1000.div_round(&Natural::trillion(), RoundingMode::Floor), 0);
    ///     assert_eq!(10.div_round(&Natural::from(4u32), RoundingMode::Up), 3);
    ///     assert_eq!(1000.div_round(&Natural::trillion(), RoundingMode::Ceiling), 1);
    ///     assert_eq!(10.div_round(&Natural::from(5u32), RoundingMode::Exact), 2);
    ///     assert_eq!(10.div_round(&Natural::from(3u32), RoundingMode::Nearest), 3);
    ///     assert_eq!(20.div_round(&Natural::from(3u32), RoundingMode::Nearest), 7);
    ///     assert_eq!(10.div_round(&Natural::from(4u32), RoundingMode::Nearest), 2);
    ///     assert_eq!(14.div_round(&Natural::from(4u32), RoundingMode::Nearest), 4);
    /// }
    /// ```
    fn div_round(self, other: &'a Natural, rm: RoundingMode) -> u32 {
        if *other == 0 {
            panic!("division by zero");
        } else {
            match *other {
                Small(small) => self.div_round(small, rm),
                Large(ref limbs) => {
                    limbs_limb_div_round_limbs(self, limbs, rm).unwrap_or_else(|| {
                        panic!("Division is not exact: {} / {}", self, other);
                    })
                }
            }
        }
    }
}

impl DivRoundAssign<u32> for Natural {
    /// Divides a `Natural` by a `u32` in place and rounds according to a specified rounding mode.
    /// See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(4, RoundingMode::Down);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Natural::trillion();
    ///     n.div_round_assign(3, RoundingMode::Floor);
    ///     assert_eq!(n.to_string(), "333333333333");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(4, RoundingMode::Up);
    ///     assert_eq!(n.to_string(), "3");
    ///
    ///     let mut n = Natural::trillion();
    ///     n.div_round_assign(3, RoundingMode::Ceiling);
    ///     assert_eq!(n.to_string(), "333333333334");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(5, RoundingMode::Exact);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(3, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "3");
    ///
    ///     let mut n = Natural::from(20u32);
    ///     n.div_round_assign(3, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "7");
    ///
    ///     let mut n = Natural::from(10u32);
    ///     n.div_round_assign(4, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "2");
    ///
    ///     let mut n = Natural::from(14u32);
    ///     n.div_round_assign(4, RoundingMode::Nearest);
    ///     assert_eq!(n.to_string(), "4");
    /// }
    /// ```
    fn div_round_assign(&mut self, other: u32, rm: RoundingMode) {
        if rm == RoundingMode::Down || rm == RoundingMode::Floor {
            *self /= other;
        } else {
            let remainder = self.div_assign_mod(other);
            match rm {
                _ if remainder == 0 => {}
                RoundingMode::Up | RoundingMode::Ceiling => *self += 1,
                RoundingMode::Nearest => {
                    let shifted_other = other >> 1;
                    if remainder > shifted_other
                        || remainder == shifted_other && other.is_even() && self.is_odd()
                    {
                        *self += 1;
                    }
                }
                RoundingMode::Exact => {
                    panic!("Division is not exact");
                }
                _ => {}
            }
        }
    }
}

impl DivRoundAssign<Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place and rounds according to a specified rounding mode,
    /// taking the `Natural` by value. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut n = 10;
    ///     n.div_round_assign(Natural::from(4u32), RoundingMode::Down);
    ///     assert_eq!(n, 2);
    ///
    ///     let mut n = 1000;
    ///     n.div_round_assign(Natural::trillion(), RoundingMode::Floor);
    ///     assert_eq!(n, 0);
    ///
    ///     let mut n = 10;
    ///     n.div_round_assign(Natural::from(4u32), RoundingMode::Up);
    ///     assert_eq!(n, 3);
    ///
    ///     let mut n = 1000;
    ///     n.div_round_assign(Natural::trillion(), RoundingMode::Ceiling);
    ///     assert_eq!(n, 1);
    ///
    ///     let mut n = 10;
    ///     n.div_round_assign(Natural::from(5u32), RoundingMode::Exact);
    ///     assert_eq!(n, 2);
    ///
    ///     let mut n = 10;
    ///     n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    ///     assert_eq!(n, 3);
    ///
    ///     let mut n = 20;
    ///     n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    ///     assert_eq!(n, 7);
    ///
    ///     let mut n = 10;
    ///     n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    ///     assert_eq!(n, 2);
    ///
    ///     let mut n = 14;
    ///     n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    ///     assert_eq!(n, 4);
    /// }
    /// ```
    fn div_round_assign(&mut self, other: Natural, rm: RoundingMode) {
        self.div_round_assign(&other, rm);
    }
}

impl<'a> DivRoundAssign<&'a Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place and rounds according to a specified rounding mode,
    /// taking the `Natural` by reference. See the `RoundingMode` documentation for details.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut n = 10;
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Down);
    ///     assert_eq!(n, 2);
    ///
    ///     let mut n = 1000;
    ///     n.div_round_assign(&Natural::trillion(), RoundingMode::Floor);
    ///     assert_eq!(n, 0);
    ///
    ///     let mut n = 10;
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Up);
    ///     assert_eq!(n, 3);
    ///
    ///     let mut n = 1000;
    ///     n.div_round_assign(&Natural::trillion(), RoundingMode::Ceiling);
    ///     assert_eq!(n, 1);
    ///
    ///     let mut n = 10;
    ///     n.div_round_assign(&Natural::from(5u32), RoundingMode::Exact);
    ///     assert_eq!(n, 2);
    ///
    ///     let mut n = 10;
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    ///     assert_eq!(n, 3);
    ///
    ///     let mut n = 20;
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    ///     assert_eq!(n, 7);
    ///
    ///     let mut n = 10;
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    ///     assert_eq!(n, 2);
    ///
    ///     let mut n = 14;
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    ///     assert_eq!(n, 4);
    /// }
    /// ```
    fn div_round_assign(&mut self, other: &'a Natural, rm: RoundingMode) {
        *self = self.div_round(other, rm);
    }
}
