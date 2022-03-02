**macro**
A macro is like a function that only accepts an input that matches a specific pattern or regex. In rust macros are denoted by `!` suffix.

Example:

```rust
println!("Hello, World");
```

**panic**
Throwing an error at runtime is called panicking.

**variables**
---
Variables are _immutable by default_ in Rust, this is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers.

```rust
// declearing a variable (i.e immutable)
let x = 5;
// or 
let x: String = "asd";
```

_Mutable Variables_
```rust
let mut x = 3;
// or
let mut x : String = "Asd";
```

> You can use the same varible name twice the previous one will be shadowed by rust.


**Contants**
constants are values that are bound to a name and are not allowed to change.

_Differences b/w constants and variables_
- First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.
- you declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.
    ```rust
    const CONSTANT_STRING: String = "CANNOT BE CHANGED OR SHADOWED";
    ```
- Rust’s naming convention for constants is to use all uppercase with underscores between words.  
- Constants are valid for the entire time a program runs, within the scope they were declared in.

**Shadowing**
Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what the program sees when the variable is used. We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:
```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```

> The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
