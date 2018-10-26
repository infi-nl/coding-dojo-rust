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
1. Enhance the `calculate_area` function to return the area of a `Circle` as well (you will need the value of π, which you can find use through `f64::consts::PI` [FIXME: does this require an import?])
1. Create a function `is_rectangular` that returns true for squares and rectangles and false for other kinds of figures.
1. Bonus: write a function `get_square_side` that returns an `Option<f32>`. If the figure is a square, the function should return its side. Otherwise the function should return `Option::None`. Protip: don't forget to wrap the square side in a `Option::Some`. [FIXME: confirm we are explaining options in the slides]
1. Bonus: write unit tests for `calculate_area`, using `#[test]` as described in FIXME ADD LINK TO DOCS

## Strings

In Rust land there are two kinds of strings. The `&str` type represents immutable strings, while `String` represents mutable ones. This can get very confusing, because string literals produce `&str` but oftentimes you want a `String`!

In the code below, `get_name` returns a `&str` and `get_greeting` returns a `String`. In the general case, strings created at compile time (literals) are immutable, while strings created at run time (e.g. by using `format!`) are mutable.

```rust
fn get_name() -> &str {
    "John Doe"
}

fn get_greeting() -> String {
    let name = get_name();
    format!("Good morning mr. {}", name)
}
```

A common mistake is to mix `&str` and `String` as if they were the same type. They are not! The code below will therefore not compile (notice how we changed the return type of the function):

```rust
fn get_name() -> String {
    "John Doe"
}
```

Fortunately, it is possible to get a `String` from a `&str` by calling `to_string`. If we take the previous (non-compiling) function and use `to_string`, we can get it to compile:

```rust
fn get_name() -> String {
    "John Doe".to_string()
}
```

 and, likewise, a `&str` from a `String`.

Two kinds of strings

Immutable vs mutable

Literals create immutable strings

You can go from immutable to mutable

FIXME: fizzbuzz?

Bonus: use Cow

## Traits and iterators

FIXME: exercise where you have to implement the iterator trait for a custom data structure
