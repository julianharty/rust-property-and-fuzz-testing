
This is a small, self-contained Rust project created by working with Chat GPT using a web browser session to experiment with proptest and cargo fuzz. The code being tested is trivial and has little value beyond providing a test bed for property and fuzz tests. 

The code is about to be restructured to extract helper functions into a small crate that can be used by both the property testing framework and cargo fuzz without poluting the 'main' codebase. I say 'main' as the code is so small and trivial it doesn't really deserve the term.

Here are a couple of examples of how to run the property tests, I've also used `println!` to output the string that causes problems for the code being tested.

```
PROPTEST_CASES=10 cargo test -- --nocapture 
```

```
PROPTEST_CASES=9 PROPTEST_VERBOSE=1 cargo test
```
