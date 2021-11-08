pub fn is_prime(n: u64) -> bool {
  if n < 4 {
    n > 1
  } else if n % 2 == 0 || n % 3 == 0 {
    false
  } else {
    let max_p = (n as f64).sqrt().ceil() as u64;
    match (5..=max_p).step_by(6).find(|p| n % p == 0 || n % (p+2) == 0) {
      Some(_) => false,
      None => true
    }
  }
}

pub struct Prime {
  curr: u64,
  next: u64,
  trial1: u64,
  trial2: u64
}

impl Prime {
  pub fn new() -> Prime {
    Prime {
      curr: 2,
      next: 3,
      trial1: 5,
      trial2: 7
    }
  }
}

impl Iterator for Prime {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    let prime = self.curr;
    self.curr = self.next;
    loop {
      self.next = self.trial1;
      self.trial1 = self.trial2;
      self.trial2 = self.next+6;
      if is_prime(self.next) {
        break;
      }
    }
    Some(prime)
  }
}


#[cfg(test)]
mod tests {
  use crate::{is_prime, Prime};
  const PRIMES: [u64; 100] = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,373,379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,463,467,479,487,491,499,503,509,521,523,541];

  #[test]
  fn check_prime() {
    for p in PRIMES {
      assert!(is_prime(p));
    }
  }

  #[test]
  fn check_not_prime() {
    let not_primes = [1,4,6,8,9,10,12,14,15,16,18,20,21,22,24,25,26,27,28,30,32,33,34,35,36];
    for p in not_primes {
      assert!(!is_prime(p));
    }
  }

  #[test]
  fn check_32bit_prime() {
    assert!(is_prime(2147483647));
  }

  #[test]
  fn check_prime_iterator() {
    let mut prime_it = Prime::new();
    for idx in 0..PRIMES.len() {
      assert_eq!(prime_it.next().unwrap(), PRIMES[idx]);
    }
  }
}
