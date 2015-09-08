//!
//!  gaussian.rs
//!
//!  Created by Mitchell Nordine at 03:28AM on May 30, 2014.
//!
//!

extern crate num;
extern crate rand;
extern crate utils;

use utils::map_range;
use rand::{Rand, random};
use std::cell::RefCell;
use std::fmt::Debug;
use num::{Float, FromPrimitive};

/// A thread local global for holding onto the second generated value without requiring the user
/// stores an instance of some type.
thread_local!(static MAYBE_NEXT_VALUE: RefCell<Option<f64>> = RefCell::new(None));

#[inline]
fn two<F>() -> F where F: Float { let one: F = F::one(); one + one }

/// Gen raw gaussian value with dist. at 0.
#[inline]
pub fn gen_raw<F>() -> F where F: Float + FromPrimitive + Rand {
    MAYBE_NEXT_VALUE.with(|maybe_next_value| {
        if let Some(next_value) = *maybe_next_value.borrow() {
            *maybe_next_value.borrow_mut() = None;
            FromPrimitive::from_f64(next_value).unwrap()
        } else {
            let (zero, one, two): (F, F, F) = (F::zero(), F::one(), two::<F>());
            let (mut va, mut vb, mut s): (F, F, F) = (zero, zero, zero);
            while s >= one || s == zero {
                va = two * random::<F>() - one;
                vb = two * random::<F>() - one;
                s = va * vb + va * vb
            };
            let multi = ((-two) * s.abs().ln() / s).abs().sqrt();
            *maybe_next_value.borrow_mut() = (vb * multi).to_f64();
            va * multi
        }
    })
}

/// Gen gaussian value with dist. at 'n' with rand randomness.
/// Result will always be in range 0.0 - 1.0, with a mean of 0.5.
#[inline]
pub fn gen<F>(n: F, randomness: f32) -> F
where F: Float + Rand + FromPrimitive + Debug {
    let (zero, one): (F, F) = (F::zero(), F::one());
    assert!(n >= zero && n <= one, "Gaussian::gen : given `n` ({:?}) must \
            be a percentage between 0 and 1.", n);
    let mut ans = gen_raw::<F>()
                * FromPrimitive::from_f32(randomness.powf(2.0)).unwrap()
                + (n * two::<F>() - one);
    ans = map_range(ans, -one, one, zero, one);
    if ans >= one || ans < zero {
        gen::<F>(n, randomness)
    } else {
        ans
    }
}

/// Gen gaussian value mapped to a range.
#[inline]
pub fn gen_map<F>(n: F, randomness: f32, min_range: F, max_range: F) -> F
where F: Float + Rand + FromPrimitive + Debug {
    let (zero, one): (F, F) = (F::zero(), F::one());
    let perc = map_range(n, min_range, max_range, zero, one);
    map_range(gen(perc, randomness), zero, one, min_range, max_range)
}

