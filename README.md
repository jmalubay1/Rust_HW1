###Jordan Malubay
##Modexp

Modexp is a Rust binary crate that to calculate an exponential modulo from command-line arguments.  

modexp = $x^y (mod m)$


###Usage
```bash
cargo run x y m
```
Where x and y and non-negative integers, and m is a positive integer

###Implementation Notes
Rust syntax is going to take me a while to get used to, but it so far it appears to be more well defined than C++.  I find the things I am writing have less ambiguity, especially across multiple functions and scopes.  With the pseudo code that was provided putting everything together wasn't too hard.  The only issue I ran into was trying to put together the unit tests.  After a few hours of fiddling I discovered that using the [#should_panic] for a test does not catch an exit() so any test will fail since the exit cannot return any result.  I manually tested my inputs correctness and keep the program within specifications.  Handling the errors with the panic instead of the exit(1) would have made unit-testing a bit easier.  
