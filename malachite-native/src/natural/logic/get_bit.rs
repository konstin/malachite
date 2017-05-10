use natural::Natural::{self, Large, Small};

impl Natural {
    /// Determines whether the `index`th bit of `self`, or the coefficient of 2^(`index`) in the
    /// binary expansion of `self`, is 0 or 1. `false` means 0, `true` means 1.
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123).get_bit(2), false);
    /// assert_eq!(Natural::from(123).get_bit(3), true);
    /// assert_eq!(Natural::from(123).get_bit(100), false);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().get_bit(12), true);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().get_bit(100), false);
    /// ```
    pub fn get_bit(&self, index: u64) -> bool {
        match *self {
            Small(x) => index < 32 && x & (1 << index) != 0,
            Large(ref xs) => {
                let limb_index = index >> 5;
                xs.get(limb_index as usize).map_or(false, |limb| limb & (1 << (index & 0x1f)) != 0)
            }
        }
    }
}

mod internal {
    use natural::Natural::{self, Large, Small};

    impl Natural {
        /// An internal function used by `Integer::get_bit`. Pretends that `self` is actually
        /// -`self` in two's complement and tests the bit at the given index.
        pub fn get_bit_neg(&self, index: u64) -> bool {
            match *self {
                Small(x) => index >= 32 || ((!x).wrapping_add(1)) & (1 << index) != 0,
                Large(ref xs) => {
                    let limb_index = (index >> 5) as usize;
                    if limb_index >= xs.len() {
                        // We're indexing into the infinite suffix of 1s
                        return true;
                    }
                    let limb = if xs.into_iter().take(limb_index).all(|&x| x == 0) {
                        // All limbs below `limb_index` are zero, so we have a carry bit when we
                        // take the two's complement
                        (!xs[limb_index]).wrapping_add(1)
                    } else {
                        !xs[limb_index]
                    };
                    limb & (1 << (index & 0x1f)) != 0
                }
            }
        }
    }
}
