
pub fn mul(a: i8, b: i8) -> i8 {
    a * b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic(expected = "attempt to multiply with overflow")]
  fn test_mul_panic() {
    let _res = mul(100 as i8, 100 as i8);
  } 
}


