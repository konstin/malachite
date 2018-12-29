use integer::Integer;
use malachite_base::num::NotAssign;
use natural::Natural;
use std::ops::Not;

/// Returns the bitwise not of a slice of limbs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::not::limbs_not;
/// use std::cmp::Ordering;
///
/// assert_eq!(limbs_not(&[0, 1, 2]), [0xffffffff, 0xfffffffe, 0xfffffffd]);
/// ```
pub fn limbs_not(limbs: &[u32]) -> Vec<u32> {
    limbs.iter().map(|limb| !limb).collect()
}

/// Writes the bitwise not of a slice of limbs to the lowest `in_limbs.len()` limbs of `out_limbs`.
/// For this to work, `out_limbs` must be at least as long as `in_limbs`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::not::limbs_not_to_out;
///
/// let mut out_limbs = [0, 1, 2];
/// limbs_not_to_out(&mut out_limbs, &[0xffff0000, 0xf0f0f0f0]);
/// assert_eq!(out_limbs, [0x0000ffff, 0x0f0f0f0f, 2]);
/// ```
pub fn limbs_not_to_out(out_limbs: &mut [u32], in_limbs: &[u32]) {
    assert!(out_limbs.len() >= in_limbs.len());
    for (x, y) in out_limbs.iter_mut().zip(in_limbs.iter()) {
        *x = !y;
    }
}

/// Takes the bitwise not of a slice of limbs in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::not::limbs_not_in_place;
/// use std::cmp::Ordering;
///
/// let mut limbs = [0, 1, 2];
/// limbs_not_in_place(&mut limbs);
/// assert_eq!(limbs, [0xffffffff, 0xfffffffe, 0xfffffffd]);
/// ```
pub fn limbs_not_in_place(limbs: &mut [u32]) {
    for limb in limbs.iter_mut() {
        limb.not_assign();
    }
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by value and returning an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((!Natural::ZERO).to_string(), "-1");
///     assert_eq!((!Natural::from(123u32)).to_string(), "-124");
/// }
/// ```
impl Not for Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self + 1,
        }
    }
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by reference and returning an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((!&Natural::ZERO).to_string(), "-1");
///     assert_eq!((!&Natural::from(123u32)).to_string(), "-124");
/// }
/// ```
impl<'a> Not for &'a Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self + 1,
        }
    }
}
