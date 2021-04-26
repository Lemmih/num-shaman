//! Partial implementation of the C++11 Shaman library: <https://gitlab.com/numerical_shaman/shaman>

use ieee754::Ieee754;
use std::ops::Add;

#[inline(never)]
fn two_sum(x: f64, y: f64, xy: f64) -> f64 {
  let x_ = xy - y;
  let y_ = xy - x;
  (x - x_).abs() + (y - y_).abs()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Shaman<T> {
  pub value: T,
  pub error: T,
}

impl Add for Shaman<f64> {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    let x = self.value;
    let y = other.value;
    let xy = x + y;
    Shaman {
      value: xy,
      error: self.error + other.error + two_sum(x, y, xy),
    }
  }
}

impl From<f64> for Shaman<f64> {
  fn from(f: f64) -> Shaman<f64> {
    Shaman {
      value: f,
      error: f.ulp().unwrap_or(0f64),
    }
  }
}

macro_rules! unop {
  ($fn:ident, $erf:ident) => {
    extern "C" {
      fn $erf(number: f64, error: f64, result: f64) -> f64;
    }
    impl Shaman<f64> {
      pub fn $fn(self) -> Self {
        let result = self.value.$fn();
        let diff = unsafe { $erf(self.value, self.error, result) };
        Shaman {
          value: result,
          error: diff.abs(),
        }
      }
    }
  };
}
unop!(sin, erf_sin);
unop!(cos, erf_cos);

#[cfg(test)]
mod tests {
  use crate::shaman::Shaman;
  use approx::Ulps;

  #[test]
  fn it_works() {
    assert_eq!(
      (Shaman {
        value: std::f64::consts::PI,
        error: 0.1
      })
      .sin()
      .value,
      0.0
    );
    // assert_eq!(
    //     1shaman,
    //     0.0
    // );
  }
}
