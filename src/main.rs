///! Command line Modular Exponentation tool
///!
///! Jordan Malubay 2021

/// Print a usage error message and exit.
/// This function is from the Moodle HW1 hints.
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

/// Parse the given string as a `u64`.
/// This function is from the Moodle HW1 hints.
fn parsenum(s: &str) -> u64 {
    let n: u32 = s.parse().unwrap_or_else(|_| error());
    u64::from(n)
}

/// Modular exponenttation function (x^y) mod m
/// Input: x,y,m - positive inegters less than 2^32
/// Output: u64
fn modexp(x:u64,y:u64,m:u64) -> u64 {
  if x == 0 {
    return 0;
  }
  if y == 0 {
    return 1;
  }
  let mut z = modexp(x,y/2,m);
  z = (z * z) % m;
  if (y % 2) == 1 {
    z = ((z % m) * x) % m;
  }
  z
}

fn main() {
    println!("{}",modexp(2,20,17));
}
