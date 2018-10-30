Rust Dojo 2018
==============

This document is part of a Rust coding dojo held at [Infi](https://infi.nl) on November 8th 2018. It is not a detailed tutorial on how to become a Rust expert, though if that is your aim you can give [The Rust Book](https://doc.rust-lang.org/book/2018-edition/index.html) a try. Our Rust dojo is meant to awaken interest in the language and give you the tools to start your Rust journey.

# Introduction

FIXME: copy description from the official website

Rust draws lots of inspiration from existing languages and integrates many features that you will find in Scala, C# or Haskell. However, the combination of features that Rust offers is quite unique. If you are interested, read [this](FIXME:link to old readme).

# Setup

FIXME:
* Instructions on how to install Rust
* Explain that we never use the rust compiler directly. Instead, we use cargo
* Create and run a new cargo project

# Exercises

## Enums and Result

If you see a Rust enum you may think they are just the same as in other languages. Consider for instance the code below:

```rust
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
```

In other languages you can use the `switch` statement to execute different code depending on the value of an enum. The function below does exactly this, using Rust's `match` construct, to check wheter it is Friday:

```rust
fn is_it_already_friday(day: Day) -> bool {
    match day {
        // It is Friday!
        Day::Friday => true,
        // Not friday :(
        _ => false,
    }
}
```

The name `match` hints that there is more going on than just a `switch` statement. In fact, `match` allows you to *pattern match* on a value. To illustrate this, let us define a more interesting enum that describes figures:

```rust
enum Figure {
    Square(f32),
    Rectangle(f32, f32),
}
```

Compared to `Day`, the `Figure` enum has the difference that each of the *variants* contains values. In the case of `Figure::Square`, the single `f32` represents the length of its side. In the case of `Figure::Rectangle`, there are two `f32` fields: one representing the width and other representing the height. Consider the `calculate_area` function, where we *pattern match* to extract the `f32` fields depending on the enum variant:

```rust
fn calculate_area(figure: Figure) -> f32 {
    match figure {
        Figure::Square(side) => side * side,
        Figure::Rectangle(width, height) => width * height,
    }
}
```

**Exercises:**

1. Enhance the `Figure` enum with an additional variant `Circle`.
1. Enhance the `calculate_area` function to return the area of a `Circle` as well (you will need the value of π, which you can find use through `std::f64::consts::PI`)
1. Create a function `is_rectangular` that returns true for squares and rectangles and false for other kinds of figures.
1. Bonus: write a function `get_square_side` that returns an `Option<f32>`. If the figure is a square, the function should return its side. Otherwise the function should return `Option::None`. Protip: don't forget to wrap the square side in a `Option::Some`.
1. Bonus: write unit tests for `calculate_area`, using `#[test]` as described in the [documentation](https://doc.rust-lang.org/book/2018-edition/ch11-00-testing.html).

## Strings

⚠ Warning: working with Rust strings for the first time can be highly frustrating. Good luck!

In Rust land there are two kinds of strings. The `String` type represents the normal, mutable, string we all are used to and the `&str` type represents an *immutable string reference*. This can get very confusing, so let's look at a bunch of examples.

In the code below, the `greet` function receives an immutable string as a parameter, representing the name of a person. It then formats the name and returns a new string. Notice that the types differ (the parameter uses `&str` and the return type uses `String`).

```rust
fn greet(name: &str) -> String {
    format!("Good morning {}", name)
}
```

You can test the previous function with the following code:

```rust
fn main() {
    let greeting = greet("John Doe");
    println!("{}", greeting);
    // Prints: Good morning John Doe
}
```

Since a `String` is mutable, you can modify the greeting before printing it. For instance, you could add some text after it:

```rust
fn main() {
    let mut greeting = greet("John Doe");
    greeting.push_str(", it is always nice to see you");
    println!("{}", greeting);
    // Prints: Good morning John Doe, it is always nice to see you
}
```

String literals create immutable strings, of type `&str`. This means you cannot modify them, unless you first transform them into a `String` (this is what `format!` does under the hood). The code below tries to modify a `&str`, but fails to compile:

```rust
fn main() {
    let str_greeting = "Good morning John Doe";
    // The following line triggers a compile error
    str_greeting.push_str(", it is always nice to see you");
    println!("{}", str_greeting);
}
```

To turn a `&str` into a `String` you first have to create an empty `String` and then copy the characters from the `&str`. Since this is a very common operation, the standard library includes a function that does this for you: `to_string`. See below the previous code snippet, now fixed:

```rust
fn main() {
    let str_greeting = "Good morning John Doe";
    let mut greeting = str_greeting.to_string();
    greeting.push_str(", it is always nice to see you");
    println!("{}", greeting);
}
```

It is also possible to turn a `String` into a `&str`. In the following example, the name is a `String`, but the `greet` function expects a `&str`, so the code doesn't compile:

```rust
fn main() {
    let name = "John Doe".to_string();
    // The following line triggers a compile error: expected a &str, got a String
    let greeting = greet(name);
    println!("{}", greeting);
}
```

You can go from `String` to `&str` by calling `as_str`. This is so common that Rust *coerces* the `String` type into a `&str` if the reference operator (`&`) is used (after all, `&str` is a string *reference*). The code below fixes the compile error from the previous example:

```rust
fn main() {
    let name = "John Doe".to_string();
    let greeting = greet(name.as_str());
    // Also valid:
    // let greeting = greet(&name);
    println!("{}", greeting);
}
```

At this point, the million dollar question is: when should I use `&str` and when `String`? In the general case, use `String` when you want to transfer ownership and `&str` otherwise. If you are not familiar with Rust's ownership model you might want to [read the docs](https://doc.rust-lang.org/book/2018-edition/ch04-00-understanding-ownership.html).

Exercises:

1. FIXME: something involving ownership?
1. Bonus: follow [this blog post](https://chrismorgan.info/blog/rust-fizzbuzz.html) to implement FizzBuzz in Rust. Spoiler: you will be surprised about how complex it can get!

## Traits and iterators

FIXME: exercise where you have to implement the iterator trait for a custom data structure
