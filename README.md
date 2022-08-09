# Welcome to my Rust Book Repo

## This repo is a collection of projects made while completing The [Rust Programming Language Book](https://doc.rust-lang.org/book)

### guessing_game

This project is a CLI number guessing game. It generates a random number from 1 to 100 (inclusive), and has the user guess until they have correctly guessed the number. The program will tell the user if the number they have provided is greater or less than the hidden random number.

### variables

Program explores rust compilers rules surrounding variables. Including scope, mutability, and data types.

### functions

Covers functions, expressions, and statements.

### ownership

Covers concepts of the stack vs heap in memory allocation and how rust deals with the common pain points of other low-level languages. Covers references and borrowing, and introduces the concept of lifetimes. Also covers the slice type.

### structs

Covers defining and instantiating structs, as well as ownership and partially covers lifetimes. Covers methods, and associated functions within the rectangles project.

### enums

Covers defining enums, and there many use cases. How the match control flow can be useful with them. Also covers the if let syntax.

### restaurant

Project builds a restaurant library crate to demonstrate packages and modules.
Chapter covers packages, crates, modules and use, paths, and public vs private within crate scope.

### collections

Covers the common standard library collections: strings, Vecs, and Hash Maps.

### error_handling

Covers the difference between unrecoverable and recoverable errors, how you should handle them, and when to expect/implement the different kinds.

### Generic Types, Traits, and Lifetimes

Generics are used to cover duplication of concepts in rust. Covers extracting functions to reduce code duplication, generic types in structs and enums, how traits are used to define behavior in a generic way, and lifetimes which is a variety of generics that tell the compiler how references relate to one another.

### adder

Project used to work with the default tests that come with the creation of a library using cargo. Demonstrates the different test attributes, and how you can build tests that have enum signatures. Covers difference between unit and integration tests in Rust, as well as how to organize modules and crates within the tests directory of a project.
