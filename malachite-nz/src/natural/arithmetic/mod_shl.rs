use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    ModMul, ModMulAssign, ModPow, ModShl, ModShlAssign, UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use std::cmp::Ordering;
use std::ops::{Shr, ShrAssign};

fn mod_shl_ref_val_unsigned<T: PrimitiveUnsigned>(x: &Natural, bits: T, m: Natural) -> Natural
where
    Natural: From<T>,
{
    if bits == T::ZERO {
        x.clone()
    } else {
        match m {
            Natural::ONE | Natural::TWO => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits), &m), m),
        }
    }
}

fn mod_shl_ref_ref_unsigned<T: PrimitiveUnsigned>(x: &Natural, bits: T, m: &Natural) -> Natural
where
    Natural: From<T>,
{
    if bits == T::ZERO {
        x.clone()
    } else {
        match m {
            &Natural::ONE | &Natural::TWO => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits), m), m),
        }
    }
}

fn mod_shl_assign_unsigned_nz<T: PrimitiveUnsigned>(x: &mut Natural, bits: T, m: Natural)
where
    Natural: From<T>,
{
    if bits != T::ZERO {
        match m {
            Natural::ONE | Natural::TWO => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits), &m), m),
        }
    }
}

fn mod_shl_assign_ref_unsigned<T: PrimitiveUnsigned>(x: &mut Natural, bits: T, m: &Natural)
where
    Natural: From<T>,
{
    if bits != T::ZERO {
        match m {
            &Natural::ONE | &Natural::TWO => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits), m), m),
        }
    }
}

macro_rules! impl_mod_shl_unsigned {
    ($t:ident) => {
        impl ModShl<$t, Natural> for Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$. Assumes the input is already reduced modulo $m$. Both [`Natural`]s
            /// are taken by value.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $2^nx \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: Natural) -> Natural {
                self.mod_shl_assign(bits, m);
                self
            }
        }

        impl<'a> ModShl<$t, &'a Natural> for Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$. Assumes the input is already reduced modulo $m$. The first
            /// [`Natural`] is taken by value and the second by reference.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $2^nx \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: &'a Natural) -> Natural {
                self.mod_shl_assign(bits, m);
                self
            }
        }

        impl<'a> ModShl<$t, Natural> for &'a Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$. Assumes the input is already reduced modulo $m$. The first
            /// [`Natural`] is taken by reference and the second by value.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $2^nx \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: Natural) -> Natural {
                mod_shl_ref_val_unsigned(self, bits, m)
            }
        }

        impl<'a, 'b> ModShl<$t, &'b Natural> for &'a Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$. Assumes the input is already reduced modulo $m$. Both [`Natural`]s
            /// are taken by reference.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $2^nx \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: &'b Natural) -> Natural {
                mod_shl_ref_ref_unsigned(self, bits, m)
            }
        }

        impl ModShlAssign<$t, Natural> for Natural {
            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$, in place. Assumes the input is already reduced modulo $m$. The
            /// [`Natural`] on the right-hand side is taken by value.
            ///
            /// $x \gets y$, where $x, y < m$ and $2^nx \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: Natural) {
                mod_shl_assign_unsigned_nz(self, bits, m);
            }
        }

        impl<'a> ModShlAssign<$t, &'a Natural> for Natural {
            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$, in place. Assumes the input is already reduced modulo $m$. The
            /// [`Natural`] on the right-hand side is taken by reference.
            ///
            /// $x \gets y$, where $x, y < m$ and $2^nx \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: &'a Natural) {
                mod_shl_assign_ref_unsigned(self, bits, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_shl_unsigned);

fn mod_shl_ref_val_signed<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
    m: Natural,
) -> Natural
where
    Natural: From<U>,
    &'a Natural: Shr<U, Output = Natural>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => x.clone(),
        Ordering::Less => x >> bits_abs,
        Ordering::Greater => match m {
            Natural::ONE | Natural::TWO => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits_abs), &m), m),
        },
    }
}

fn mod_shl_ref_ref_signed<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
    m: &Natural,
) -> Natural
where
    Natural: From<U>,
    &'a Natural: Shr<U, Output = Natural>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => x.clone(),
        Ordering::Less => x >> bits_abs,
        Ordering::Greater => match m {
            &Natural::ONE | &Natural::TWO => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits_abs), m), m),
        },
    }
}

fn mod_shl_assign_signed_nz<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &mut Natural,
    bits: S,
    m: Natural,
) where
    Natural: From<U> + ShrAssign<U>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => {}
        Ordering::Less => *x >>= bits_abs,
        Ordering::Greater => match m {
            Natural::ONE | Natural::TWO => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits_abs), &m), m),
        },
    }
}

fn mod_shl_assign_ref_signed<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &mut Natural,
    bits: S,
    m: &Natural,
) where
    Natural: From<U> + ShrAssign<U>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => {}
        Ordering::Less => *x >>= bits_abs,
        Ordering::Greater => match m {
            &Natural::ONE | &Natural::TWO => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits_abs), m), m),
        },
    }
}

macro_rules! impl_mod_shl_signed {
    ($t:ident) => {
        impl ModShl<$t, Natural> for Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$. Assumes the input is already reduced modulo $m$. Both [`Natural`]s
            /// are taken by value.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $\lfloor 2^nx \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: Natural) -> Natural {
                self.mod_shl_assign(bits, m);
                self
            }
        }

        impl<'a> ModShl<$t, &'a Natural> for Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$. Assumes the input is already reduced modulo $m$. The first
            /// [`Natural`] is taken by value and the second by reference.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $\lfloor 2^nx \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: &'a Natural) -> Natural {
                self.mod_shl_assign(bits, m);
                self
            }
        }

        impl<'a> ModShl<$t, Natural> for &'a Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$. Assumes the input is already reduced modulo $m$. The first
            /// [`Natural`] is taken by reference and the second by value.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $\lfloor 2^nx \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: Natural) -> Natural {
                mod_shl_ref_val_signed(self, bits, m)
            }
        }

        impl<'a, 'b> ModShl<$t, &'b Natural> for &'a Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$. Assumes the input is already reduced modulo $m$. Both [`Natural`]s
            /// are taken by reference.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $\lfloor 2^nx \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: &'b Natural) -> Natural {
                mod_shl_ref_ref_signed(self, bits, m)
            }
        }

        impl ModShlAssign<$t, Natural> for Natural {
            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$, in place. Assumes the input is already reduced modulo $m$. The
            /// [`Natural`] on the right-hand side is taken by value.
            ///
            /// $x \gets y$, where $x, y < m$ and $\lfloor 2^nx \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: Natural) {
                mod_shl_assign_signed_nz(self, bits, m);
            }
        }

        impl<'a> ModShlAssign<$t, &'a Natural> for Natural {
            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo another
            /// [`Natural`] $m$, in place. Assumes the input is already reduced modulo $m$. The
            /// [`Natural`] on the right-hand side is taken by reference.
            ///
            /// $x \gets y$, where $x, y < m$ and $\lfloor 2^nx \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: &'a Natural) {
                mod_shl_assign_ref_signed(self, bits, m);
            }
        }
    };
}
apply_to_signeds!(impl_mod_shl_signed);
