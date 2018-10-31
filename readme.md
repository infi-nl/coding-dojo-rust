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

[Rust playground](https://play.rust-lang.org/)

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

**Exercises (see `./enums`):**

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

At this point, the million dollar question is: when should I use `&str` and when `String`? In the general case, use `String` when you want to transfer ownership and `&str` otherwise. If you are not familiar with Rust's ownership model you might want to [read the docs](https://doc.rust-lang.org/book/2018-edition/ch04-00-understanding-ownership.html) or just ask for help during the dojo.

**Exercises (see `./strings`):**

1. Modify the `greet` function so it takes a `String` parameter instead of a `&str`. Can you fix the compile errors?
1. The `greet` function does not have special logic to handle empty names. If you call it with the empty string it will return `"Good morning "`, which is unfortunate. Adapt the function so it returns something more interesting (like `"Good morning stranger"`). Hint: you will probably want to use the `is_empty` method on the `String`. *Sidenote: the `String` type actually doesn't have an `is_empty` method, but `&str` does. Fortunately, a `String` can call methods defined for `&str`* because of [this magic](https://doc.rust-lang.org/book/2018-edition/ch15-02-deref.html).
1. Check out the [String](https://doc.rust-lang.org/std/string/struct.String.html) and [&str](https://doc.rust-lang.org/std/primitive.str.html) documentation to manipulate strings in creative ways inside the `greet` method. There are functions to turn strings to uppercase, trim them, split them into lines, etc.
1. Create a struct `Person` that contains a field `name` of type `String`. Implement a method `Person::greet` that returns a `String` with the greeting, in the same vein as the `greet` function we have been using in the rest of the exercise. Ensure it is possible to call `Person::greet` more than once, as in `person.greet(); person.greet();`. Hint: there is a difference between using `self` and `&self` in the first parameter of the method.
1. Bonus (difficult): follow [this blog post](https://chrismorgan.info/blog/rust-fizzbuzz.html) to implement FizzBuzz in Rust. Spoiler: you will be surprised about how complex it can get!
1. Bonus (difficult): modify the `Person` struct so that the `name` field has the `&str` type instead of `String`. Can you get it to compile?

## Traits and iterators

Iterators are among Rust's most useful features. They represent streams of data, which you can manipulate using [tons of handy functions](https://doc.rust-lang.org/std/iter/trait.Iterator.html), like `map` (equivalent to C#'s `Select`) and `filter` (equivalent to `Where`). All collections in the Rust standard library support iterating through their elements by calling the `iter` method. For instance, you can count the amount of even numbers in a vector using the following code:

```rust
let numbers = vec![42, 33, 7, 101];
let odd_count = numbers.iter().filter(|x| *x % 2 == 0).count();
```

> Sidenote: we are using `*x` in the lambda passed to `filter` because `x` is a reference to a number. Its type is `&u32`, but we need a `u32` to use the `%` operator. We remove the reference by using the *dereference* operator (`*`).

Iterators are also used when going through a collection in a `for` loop. For instance, you can print all numbers in a vector with the following code:

```rust
let numbers = vec![42, 33, 7, 101];
for x in numbers.iter() {
    println!("{}", x);
}
```

Since iterating through a collection in a for loop is such a common use case, there is syntactic sugar that allows you to omit the `iter` call completely, as shown below:

```rust
let numbers = vec![42, 33, 7, 101];
for x in &numbers {
    println!("{}", x);
}
```

> Sidenote: we are using `&numbers` here instead of `numbers` to indicate that the for loop is *borrowing* the contents of the vector, instead of *consuming* them. If you remove the `&` you won't be able to use `numbers` in the code that comes after the for loop.

Combining the features mentioned above we can come up with the following code to print the odd numbers from a vector:

```rust
fn print_odd_numbers(numbers: Vec<u32>) {
    for x in numbers.iter().filter(|x| *x % 2 == 0) {
        println!("{}", x);
    }
}
```

Rust allows you to create your own iterators as well. That is the purpose of the `Iterator` trait, defined as follows:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ...
    // Lots of functions defined below, like `map`, `filter` and friends,
    // which you get for free if you implement the `next` function
}
```

There is no more magic to it than this. An iterator is just an object with a `next` method that returns the next element in the stream, or `Nothing` if you have reached the last element. Below is an example of an iterator over all positive integers:

```rust
struct PositiveInts {
    current: u32,
}

impl PositiveInts {
    fn new() -> PositiveInts {
        PositiveInts { current: 0 }
    }
}

impl Iterator for PositiveInts {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        // Stop when we reach the last integer
        if self.current == std::u32::MAX {
            return None;
        }

        let ret = self.current;
        self.current += 1;
        Some(ret)
    }
}
```

After defining `PositiveInts` we can use it the same way as the other iterators. Notice, however, that this iterator will produce a *huge* amount of elements, so it is probably a bad idea to iterate through all of them. We can use the `take` function to express that we are only interested in the first 10 elements:

```rust
fn main() {
    // Print the first 10 positive integers
    for x in PositiveInts::new().take(10) {
        println!("{}", x);
    }
}
```

By the way, iterating through sequences of numbers is such a common operation that there is special syntax for it, called range syntax:

```rust
fn main() {
    // Print the first 10 positive integers
    for x in 0..10 {
        println!("{}", x);
    }
}
```

**Exercises (see `./iterators`):**

1. Rewrite the `print_even` function using the iterator methods `filter` and `sum`.
1. Rewrite the `sum_squares` function using the iterator methods `map` and `sum`.
1. Create an iterator that returns the elements of a `Vec<u32>`. Hint 1: you will need a struct with two fields: one to store the vector, another to store the current index. Hint 2: the type of the index number is `usize`.
1. Bonus: the `square_vector` function uses indexing to replace the values of the elements of the vector. Rewrite it so it uses `iter_mut` and modifies the values without resorting to indexing. Note: `iter_mut` is similar to `iter`, with the difference that it allows you to mutate the values returned by the iterator.
1. Bonus (advanced): create an iterator that goes through the elements of a `Vec<u32>`, *without consuming the vector*. That is, write a similar iterator to the one you get by calling `Vec::iter`.
