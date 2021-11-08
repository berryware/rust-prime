# A Prime Iterator in rust

Inspired by Alessio Saltarin's [Primality Test in Scala](https://dev.to/guildenstern70/a-pure-functional-primality-test-in-scala-3gif), I thought that it would be interesting to implement the same primality test in rust and then use this test as a means to build an Iterator that lazily generates prime numbers.

The primality test is based on the fact that all primes greater than 3 are of the form 6k ± 1, where k is any integer greater than 0. The rust version of this algorithm is:

```rust
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
```
`is_prime` first tests for `n<4` and returns `n>1`. `n=2` or `n=3` will return `true` all other `n<4` return `false`. The next test checks if n is an even multiple of 2 or 3. If it is, return `false` as n is not prime.

The final test checks to see if n is evenly divisible by any `6k ± 1` up to the square root of n. This is done by starting at 5 and checking if n is an even multiple of 5 or 5+2, 6-1 or 6+1 respectively. If it is an even multiple we will return `false`, if not we add 6 and test again. If no even multiple is found, n is prime and return `true`.

Many will point out that using sqrt() and ceil() are expensive, but I only use it once up front to get the upper bound for the comparison. This removes the need to square the `6k ± 1` in the test loop to determine when to break. It also allowed me to use the range `(5..=max_p).step_by(6).find` with the predicate `|p| n % p == 0 || n % (p+2) == 0` to identify non-primes. I add that to a match statement to return `false` if a non-prime is identified and `true` if n is prime.

Now that we have a primality test we can construct the lazy prime Iterator in rust. The code for the Iterator is:
```rust
pub struct Prime {
  last: u64,
  next: u64
}

impl Prime {
  pub fn new() -> Prime {
    Prime {
      last: 2,
      next: 3
    }
  }
}

impl Iterator for Prime {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    let prime = self.last;
    self.last = self.next;
    loop {
      self.next += match self.next%6 {
        1 => 4,
        _ => 2,
      };
      if is_prime(self.next) {
        break;
      }
    }
    Some(prime)
  }
}
```
This iterator operates with constant memory and only generates one prime at a time, but it performs a modulo and match operation inside the loop. After some consideration, both the modulo and match can be removed from inside the loop. If we initialize the `struct Prime` with the first two primes and the first `6k ± 1` we only need to add 6 to get the next trial prime. The final Iterator is:

```rust
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
```