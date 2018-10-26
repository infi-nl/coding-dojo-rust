Rust for C# programmers
=======================

You might have heard about Rust, a new programming language developed by Mozilla Research. There are excellent resources out there if you want to learn the language, but to my knowledge there isn't much oriented to C# programmers. Hopefully this is a small step in the right direction.

This document is part of a Rust coding dojo held at [Infi](https://infi.nl) on November 8th 2018. It is not a detailed tutorial on how to become a Rust expert, though if that is your aim you can give [The Rust Book](https://doc.rust-lang.org/book/2018-edition/index.html) a try. Our Rust dojo is meant to awaken interest in the language and give you the tools to start your Rust journey. We will use C# as starting point, so expect lots of comparisons between the two languages.

# Introduction

## Why a new programming language?

Rust is a pretty unique language. From a researcher's perspective, it is interesting because it applies decades of academic research in a way that is actually usable. From a programmer's perspective, its main appeal is that it offers excellent performance, without giving up [memory safety](https://en.wikipedia.org/wiki/Memory_safety). Besides, Rust forces you to think rigorously about your programs, thereby helping you avoid many kinds of bugs that other languages happily allow. There is a lot to learn about programming just by learning the language.

Rust draws lots of inspiration from existing languages and integrates many features that you will find in Scala, C# or Haskell. However, there are two features that make Rust unique: *performance on by default* and *thread-safety on by default*. We will discuss them below.

## Performance on by default

When Mozilla set out to develop Rust, their goal was to create a programming language that could replace C++ in Firefox. As of [Firefox Quantum](https://hacks.mozilla.org/2017/11/entering-the-quantum-era-how-firefox-got-fast-again-and-where-its-going-to-get-faster/), parts of the browser have indeed been written in Rust! To be a viable replacement, Rust had to be at least as fast as C++. An ambitious goal.

Languages such as C++ offer the programmer an arsenal of tools to control low level aspects of a program. This is valid for Rust as well. For instance, both languages have no built-in [garbage collector](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)). Without a garbage collector, your program spends less time figuring out which memory to clean up and more time doing what you actually want it to do.

But if the languages are that similar, why create a new language at all? It turns out that C++'s low level features open the door to all kinds of [pernicious bugs](https://en.wikipedia.org/wiki/Undefined_behavior), difficult to track and often cause of [security vulnerabilities](https://security.stackexchange.com/questions/115507). Rust, on the other hand, has a *very* smart compiler, capable of offering similar low level features, while keeping the door shut for the aforementioned class of bugs.

But let's go back to Rust's performance. In the general case, when compared to C#, Rust should have:

* Faster start-up
* Smaller memory footprint
* Less heap allocations
* More agressive program optimization
* More statically dispatched functions (instead of [dynamically](https://en.wikipedia.org/wiki/Dynamic_dispatch))

There are of course more differences than the ones mentioned above, but these give a rough idea of what you can expect from Rust. Let's now look at how they influence performance in a test program. The first version is written in C# and the second one in Rust. You can find the source code in [TODO].

TODO: add example comparing C# and Rust

TODO: show numbers (startup time, runtime)

## Thread-safety on by default

Many programming languages, including C++, have been designed without taking concurrency in mind. Writing concurrent code in such languages is therefore [incredibly hard](http://www.acodersjourney.com/2017/08/top-20-cplusplus-multithreading-mistakes/). Again, there is a huge amount of things that can go wrong.

The main problem with multithreaded code is that it is possible to, at the same time, read data from one thread while mutating the same data from another thread (this is known as a data race). For instance, in C# you may be iterating through a `List` in one thread while modifying it in a different thread. Fortunately, this throws an `InvalidOperationException`. However, in some situations you are not that lucky and the program just behaves in a very weird way, without any exception to warn you (see the example at the end of this section).

Some languages such as Erlang, Haskell or Clojure solve this issue through immutability. Since you cannot mutate anything, it is impossible to concurrently read and modify the same data. This, however, limits the kinds of programs you can write. Sometimes you really need mutation. Rust claims you can have your cake and eat it too: data-race free (i.e. thread-safe) concurrency *and* mutation.

The example below shows a C# program containing a data race and its Rust equivalent. The C# program returns a bogus result, while the Rust program refuses to compile. You can find the source code in [TODO]

TODO: add example of a data race allowed by C# and prevented by Rust

TODO: show C#'s output

TODO: add example of a Rust program that does compile

Note that Rust cannot prevent [deadlocks](https://en.wikipedia.org/wiki/Deadlock) and other kinds of [race conditions](https://en.wikipedia.org/wiki/Race_condition) (race conditions are a superset of data races). In fact, no amount of static checking can prevent these problems, unless you are using a language that is not turing-complete (see [Rice's theorem](https://en.wikipedia.org/wiki/Rice's_theorem)).

## Evaluating Rust's unique features

Until Rust, high-level programming languages have resorted to garbage collection as the only way to achieve memory safety. In the same vein, they have enforced immutability and message passing to achieve thread-safety. Rust, on the other hand, leverages decades of programming language research to achieve something that seemed to be impossible:

* Memory safety without garbage collection
* Thread-safety with mutable shared data

Another way to put it is that [Rust has a static garbage collector](https://words.steveklabnik.com/borrow-checking-escape-analysis-and-the-generational-hypothesis), which determines at compile time when objects need to be destroyed, and a static thread-safety detector, which refuses to compile programs that may have data races.

So what's the catch? Is Rust really a magic programming language that automatically makes your code fast and thread-safe without any effort on your part? That sounds too good to be true. In fact, Rust has a [steep learning curve](http://julio.meroh.net/2018/06/rust-review-learning-curve.html) and forces you to think about programs and data in a way that takes time to understand. On the bright side, it all gets much easier once you have a good grip on the language.

## Other features

Talk about ADTs/enums, pattern matching

No inheritance, no classes

Traits

# Exercises

TODO: add dojo exercises
