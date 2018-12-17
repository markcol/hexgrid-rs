//! A library to work with hex grids written in Rust.

#![forbid(overflowing_literals)]
#![warn(
    missing_docs,
    clippy::all,
    rust_2018_idioms,
    intra_doc_link_resolution_failure,
    missing_copy_implementations,
    missing_debug_implementations,
    path_statements,
    trivial_bounds,
    type_alias_bounds,
    unconditional_recursion,
    unions_with_drop_fields,
    while_true,
    bad_style,
    future_incompatible,
    rust_2018_compatibility,
    rust_2018_idioms,
    unused
)]
#![allow(
    dead_code,
    unknown_lints,
    clippy::cyclomatic_complexity,
    clippy::needless_pass_by_value,
    clippy::too_many_arguments
)]

use std::ops::{Add, AddAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};

/// Representation of a specific hex location.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Hex {
    /// Horizontal position of hex.
    pub q: i32,
    /// Vertical position of hex.
    pub r: i32,
    /// Offset position of hex.
    pub s: i32,
}

impl Hex {
    /// Create a new Hex and return it to the caller.
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        Hex { q, r, s }
    }

    /// Calculate the distance between the current hex and `other`.
    pub fn distance(self, other: Hex) -> usize {
        let l = self - other;
        ((l.q.abs() + l.r.abs() + l.s.abs()) / 2) as usize
    }
}

impl Add for Hex {
    type Output = Hex;

    fn add(self, other: Hex) -> Hex {
        Hex {
            q: self.q + other.q,
            r: self.r + other.r,
            s: self.s + other.s,
        }
    }
}

impl AddAssign for Hex {
    fn add_assign(&mut self, other: Hex) {
        self.q += other.q;
        self.r += other.r;
        self.s += other.s;
    }
}

impl Sub for Hex {
    type Output = Hex;

    fn sub(self, other: Hex) -> Hex {
        Hex {
            q: self.q - other.q,
            r: self.r - other.r,
            s: self.s - other.s,
        }
    }
}

impl SubAssign for Hex {
    fn sub_assign(&mut self, other: Hex) {
        self.q -= other.q;
        self.r -= other.r;
        self.s -= other.s;
    }
}

impl Div for Hex {
    type Output = Hex;

    fn div(self, other: Hex) -> Hex {
        Hex {
            q: self.q / other.q,
            r: self.r / other.r,
            s: self.s / other.s,
        }
    }
}

impl DivAssign for Hex {
    fn div_assign(&mut self, other: Hex) {
        self.q /= other.q;
        self.r /= other.r;
        self.s /= other.s;
    }
}

impl Mul for Hex {
    type Output = Hex;

    fn mul(self, other: Hex) -> Hex {
        Hex {
            q: self.q * other.q,
            r: self.r * other.r,
            s: self.s * other.s,
        }
    }
}

impl MulAssign for Hex {
    fn mul_assign(&mut self, other: Hex) {
        self.q *= other.q;
        self.r *= other.r;
        self.s *= other.s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let a = Hex::new(1, 2, 3);
        let b = Hex::new(1, 2, 3);
        assert_eq!(&a, &b);
    }

    #[test]
    fn inequality() {
        let a = Hex::new(1, 2, 3);
        let b = Hex::new(2, 2, 3);
        assert_ne!(&a, &b);
    }

    #[test]
    fn add() {
        assert_eq!(
            Hex { q: 1, r: 2, s: 3 } + Hex { q: 4, r: 5, s: 6 },
            Hex { q: 5, r: 7, s: 9 }
        );
    }

    #[test]
    fn add_assign() {
        let mut a = Hex::new(1, 2, 3);
        a += Hex { q: 4, r: 5, s: 6 };
        assert_eq!(a, Hex { q: 5, r: 7, s: 9 });
    }

    #[test]
    fn sub() {
        assert_eq!(
            Hex { q: 5, r: 7, s: 9 } - Hex { q: 4, r: 5, s: 6 },
            Hex { q: 1, r: 2, s: 3 }
        );
    }

    #[test]
    fn sub_assign() {
        let mut a = Hex::new(5, 7, 9);
        a -= Hex { q: 4, r: 5, s: 6 };
        assert_eq!(a, Hex { q: 1, r: 2, s: 3 });
    }

    #[test]
    fn div() {
        assert_eq!(
            Hex { q: 20, r: 35, s: 54 } / Hex { q: 4, r: 5, s: 6 },
            Hex { q: 5, r: 7, s: 9 }
        );
    }

    #[test]
    fn div_assign() {
        let mut a = Hex::new(20, 35, 54);
        a /= Hex { q: 4, r: 5, s: 6 };
        assert_eq!(a, Hex { q: 5, r: 7, s: 9 });
    }

    #[test]
    fn mul() {
        assert_eq!(
            Hex { q: 5, r: 7, s: 9 } * Hex { q: 4, r: 5, s: 6 },
            Hex { q: 20, r: 35, s: 54 }
        );
    }

    #[test]
    fn mul_assign() {
        let mut a = Hex::new(5, 7, 9);
        a *= Hex { q: 4, r: 5, s: 6 };
        assert_eq!(a, Hex { q: 20, r: 35, s: 54 });
    }

    #[test]
    fn distance() {
        let a = Hex::new(5, 7, 9);
        assert_eq!(a.distance(Hex { q: 4, r: 5, s: 6 }), 3);
    }
}
