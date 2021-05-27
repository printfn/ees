# ees: Simple Error-Handling Library

`ees` is a simple error-handling library. Rather than provide its own error-related
types, it uses `std::error::Error` and provides a number of convenience functions.

```rust
use std::io::Read;

// Use ees::Error for arbitrary owned errors
fn do_work() -> Result<(), ees::Error> {
    let mut file = std::fs::File::open("hello world")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        // Construct an error on the fly with a given message
        ees::bail!("file is empty");
    }
    Ok(())
}

// ees::ErrorRef<'_> represents an arbitrary borrowed error
fn take_an_error(error: ees::ErrorRef<'_>) {
    // Print the complete error chain
    println!("Error: {}", ees::print_error_chain(error));
}

// Use ees::MainResult to automatically create nicely-formatted error messages
// in the main() function
fn main() -> ees::MainResult {
    do_work()?;
    Ok(())
}
```
