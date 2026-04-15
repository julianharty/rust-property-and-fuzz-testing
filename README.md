
This is a small, self-contained Rust project created by working with Chat GPT using a web browser session to experiment with proptest and cargo fuzz. The code being tested is trivial and has little value beyond providing a test bed for property and fuzz tests. 

The code has been restructured to extract helper functions into a small crate that can be used by both the property testing framework and cargo fuzz without poluting the 'main' codebase. I say 'main' as the code is so small and trivial it doesn't really deserve the term.

Here are a couple of examples of how to run the property tests. I've also used `println!` to output the string that causes problems for the code being tested. The --nocapture flag means the println! output appears in the terminal session. 

To run the property based tests:
```
cargo test -p colorlib -- --nocapture
```

To set the number of property tests, use the optional environment parameter, as follows: 
```
PROPTEST_CASES=9 cargo test -p colorlib -- --nocapture
```

There's also an optional verbose environment parameter, as follows:
```
PROPTEST_CASES=9 PROPTEST_VERBOSE=1 cargo test
```

To run all the tests in the workspace (apart from the Fuzz tests):
```
cargo test --workspace -- --nocapture
```

And finally to run the fuzz tests:
```
cargo +nightly fuzz run parse_hex
```

These run for a long time by default; here's how to specify the duration to run for, the tests actually take a few more seconds before they complete.
```
cargo +nightly fuzz run parse_hex -- -max_total_time=10
``` 
