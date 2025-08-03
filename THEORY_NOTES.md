# Rust Theory Notes - 100 Exercises Summary

*Comprehensive summary of key concepts from the "100 Exercises to Learn Rust" course*

## Table of Contents
1. [Fundamentals & Basic Types](#1-fundamentals--basic-types)
2. [Control Flow & Basic Patterns](#2-control-flow--basic-patterns)
3. [Ownership & Memory Management](#3-ownership--memory-management)
4. [Structs & Methods](#4-structs--methods)
5. [Traits & Generics](#5-traits--generics)
6. [Advanced Enums & Pattern Matching](#6-advanced-enums--pattern-matching)
7. [Error Handling & Option Types](#7-error-handling--option-types)
8. [Advanced Error Handling](#8-advanced-error-handling)
9. [Package System & Dependencies](#9-package-system--dependencies)
10. [Collections & Data Structures](#10-collections--data-structures)
11. [Lifetimes & References](#11-lifetimes--references)
12. [Iterators & Functional Programming](#12-iterators--functional-programming)
13. [Concurrency - Threads](#13-concurrency---threads)
14. [Concurrency - Async/Await](#14-concurrency---asyncawait)
15. [Smart Pointers & Memory Management](#15-smart-pointers--memory-management)
16. [Testing & Documentation](#16-testing--documentation)
17. [Advanced Rust Features](#17-advanced-rust-features)
18. [Key Learning Patterns](#18-key-learning-patterns)

---

## 1. Fundamentals & Basic Types

### Variables and Type System

**Variable Declaration**
```rust
let x = 5;        // Immutable by default
let mut y = 10;   // Mutable variable
```

**Key Concepts:**
- Variables are **immutable by default** - use `mut` keyword for mutability
- **Type inference**: Compiler can deduce types from context
- **Function arguments**: Must have explicit type annotations (no inference)
- **No automatic coercion**: Rust doesn't convert between types automatically

### Integer Types

**Primitive Integer Types:**
- **Signed**: `i8`, `i16`, `i32`, `i64`, `i128`
- **Unsigned**: `u8`, `u16`, `u32`, `u64`, `u128`
- **Default**: Integer literals default to `i32`

```rust
let a: i32 = 42;     // 32-bit signed integer
let b: u64 = 100;    // 64-bit unsigned integer
let c = 25;          // Defaults to i32
```

**Key Properties:**
- Number indicates bit width (8, 16, 32, 64, 128)
- Signed types use two's complement representation
- No automatic conversion between different integer types

### Arithmetic & Overflow Handling

**Basic Operators:**
```rust
let sum = 5 + 3;      // Addition
let diff = 10 - 4;    // Subtraction
let product = 6 * 7;  // Multiplication
let quotient = 20 / 3; // Division (truncates: result is 6)
let remainder = 20 % 3; // Modulo
```

**Overflow Strategies:**

1. **Panic (Debug Mode Default):**
```rust
// This will panic in debug mode if overflow occurs
let result = i32::MAX + 1;
```

2. **Wrapping Arithmetic:**
```rust
let result = i32::MAX.wrapping_add(1); // Returns i32::MIN
let result = 255u8.wrapping_add(1);    // Returns 0
```

3. **Saturating Arithmetic:**
```rust
let result = i32::MAX.saturating_add(1); // Returns i32::MAX
let result = 0u8.saturating_sub(1);      // Returns 0
```

**Build Profiles:**
- **Debug**: Panics on overflow (helps catch bugs)
- **Release**: Wraps on overflow (performance optimization)

### Type Conversions

**Using `as` for Primitive Types:**
```rust
let a: i32 = 42;
let b: i64 = a as i64;  // Safe: smaller to larger
let c: i16 = a as i16;  // Potentially unsafe: may truncate
```

**Best Practice:** Only use `as` when going from smaller to larger types to avoid data loss.

---

## 2. Control Flow & Basic Patterns

### Branching

**`if` Expressions:**
```rust
let number = 5;

if number > 0 {
    println!("Positive");
} else if number < 0 {
    println!("Negative");
} else {
    println!("Zero");
}

// if as expression
let message = if number > 0 { "positive" } else { "non-positive" };
```

**Key Points:**
- Condition must be `bool` type (no truthy/falsy values)
- `if/else` are expressions that return values
- All branches must return the same type

### Loops

**`while` Loops:**
```rust
let mut counter = 0;
while counter < 5 {
    println!("Counter: {}", counter);
    counter += 1;
}
```

**`for` Loops:**
```rust
// Range iteration
for i in 1..5 {        // 1, 2, 3, 4 (exclusive end)
    println!("{}", i);
}

for i in 1..=5 {       // 1, 2, 3, 4, 5 (inclusive end)
    println!("{}", i);
}

// Collection iteration
let vec = vec![1, 2, 3];
for item in vec {
    println!("{}", item);
}
```

### Basic Pattern Matching

**`match` with Simple Values:**
```rust
let number = 3;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Other"),
}
```

**Guards and Ranges:**
```rust
let age = 25;

match age {
    0..=12 => println!("Child"),
    13..=19 => println!("Teenager"),
    20..=64 => println!("Adult"),
    65.. => println!("Senior"),
}
```

---

## 3. Ownership & Memory Management

### Ownership Fundamentals

**Core Rules:**
1. Each value has exactly one owner
2. Data is never mutated while being read
3. Data is never read while being mutated  
4. Data is never accessed after destruction

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2, s1 is no longer valid
    
    // println!("{}", s1); // This would cause a compile error
    println!("{}", s2);     // This is fine
}
```

**Function Ownership:**
```rust
fn take_ownership(s: String) {
    println!("{}", s);
} // s goes out of scope and is dropped

fn main() {
    let s = String::from("hello");
    take_ownership(s);  // s is moved into function
    // println!("{}", s); // Error: s is no longer valid
}
```

### References & Borrowing

**Immutable References:**
```rust
fn calculate_length(s: &String) -> usize {
    s.len()  // Can read but not modify
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Borrow s1
    println!("Length of '{}' is {}", s1, len);  // s1 still valid
}
```

**Mutable References:**
```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);  // "hello, world"
}
```

**Borrowing Rules:**
- Can have multiple immutable references OR one mutable reference
- References must always be valid (no dangling references)
- Can't modify owner while borrowed

```rust
let mut s = String::from("hello");

let r1 = &s;     // OK: multiple immutable references
let r2 = &s;     // OK
// let r3 = &mut s; // Error: can't have mutable ref while immutable refs exist

println!("{} and {}", r1, r2);  // r1, r2 last used here

let r3 = &mut s;  // OK: immutable refs are no longer used
println!("{}", r3);
```

### Memory Layout

**Stack vs Heap:**

**Stack:**
- LIFO (Last In, First Out) structure
- Very fast allocation/deallocation
- Stores function frames and local variables
- Fixed size data

**Heap:**
- Flexible allocation for dynamic data
- Slower than stack access
- Used for data with unknown size at compile time

**String Memory Layout:**
```rust
let s = String::from("hello");
```

Stack (for variable `s`):
- **ptr**: pointer to heap data
- **len**: current length (5)
- **capacity**: allocated capacity

Heap:
- Actual string data: `"hello"`

---

## 4. Structs & Methods

### Struct Definition

**Basic Struct:**
```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
```

**Struct Instantiation:**
```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

**Field Access:**
```rust
println!("User email: {}", user1.email);

let mut user2 = User { /* ... */ };
user2.email = String::from("anotheremail@example.com");
```

### Methods & Implementation

**Implementation Blocks:**
```rust
impl User {
    // Associated function (static method)
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }
    
    // Instance method
    fn is_active(&self) -> bool {
        self.active
    }
    
    // Mutable instance method
    fn deactivate(&mut self) {
        self.active = false;
    }
    
    // Takes ownership
    fn delete(self) {
        println!("Deleting user: {}", self.username);
        // self is dropped here
    }
}
```

**Method Usage:**
```rust
// Associated function (called with ::)
let user = User::new(String::from("bob"), String::from("bob@email.com"));

// Instance methods (called with .)
println!("Active: {}", user.is_active());

let mut user = user; // Make mutable
user.deactivate();

user.delete(); // user is consumed here
```

### Encapsulation & Visibility

**Public vs Private:**
```rust
mod user {
    pub struct User {
        pub username: String,    // Public field
        email: String,           // Private field
    }
    
    impl User {
        pub fn new(username: String, email: String) -> User {
            User { username, email }
        }
        
        pub fn email(&self) -> &str {  // Public getter
            &self.email
        }
    }
}
```

### Validation Patterns

**Constructor with Validation:**
```rust
impl User {
    pub fn new(username: String, email: String) -> Result<User, String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        Ok(User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        })
    }
}
```

**Setter Methods:**
```rust
impl User {
    // Self-consuming setter (chainable)
    pub fn username(mut self, username: String) -> Self {
        self.username = username;
        self
    }
    
    // Mutable reference setter
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
}
```

---

## 5. Traits & Generics

### Trait Fundamentals

**Trait Definition:**
```rust
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn brief_summary(&self) -> String {
        String::from("(Read more...)")
    }
}
```

**Trait Implementation:**
```rust
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}
```

**Using Traits:**
```rust
let article = Article {
    title: String::from("Rust Traits"),
    content: String::from("Traits define shared behavior"),
};

println!("{}", article.summarize());
```

### Core Standard Library Traits

**`Clone` Trait:**
```rust
#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1.clone();  // Deep copy
// Both p1 and p2 are valid
```

**`Copy` Trait:**
```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // Implicit copy
// Both p1 and p2 are valid (no move occurred)
```

**Key Difference:**
- `Copy`: Implicit duplication for simple stack data
- `Clone`: Explicit duplication, can be expensive

**`PartialEq` and `Eq`:**
```rust
#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
}

let person1 = Person { name: "Alice".to_string(), age: 30 };
let person2 = Person { name: "Alice".to_string(), age: 30 };

assert_eq!(person1, person2);  // Uses PartialEq
```

**`Debug` Trait:**
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };
println!("{:?}", p);      // Point { x: 1, y: 2 }
println!("{:#?}", p);     // Pretty-printed format
```

### Advanced Trait Concepts

**Trait Bounds:**
```rust
fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple bounds
fn process<T: Summary + Clone>(item: T) {
    let copy = item.clone();
    println!("{}", copy.summarize());
}

// Where clause for complex bounds
fn complex_function<T, U>(t: T, u: U) -> String
where
    T: Summary + Clone,
    U: Summary + Debug,
{
    format!("{} - {:?}", t.summarize(), u)
}
```

**Associated Types:**
```rust
trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: usize,
    max: usize,
}

impl Iterator for Counter {
    type Item = usize;  // Concrete type for associated type
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}
```

**Orphan Rule:**
You can only implement a trait for a type if:
- You own the trait, OR
- You own the type

This prevents conflicts between different crates implementing the same trait for the same type.

**Operator Overloading:**
```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
let p3 = p1 + p2;  // Uses the Add implementation
assert_eq!(p3, Point { x: 4, y: 6 });
```

**Derive Macros:**
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}
```

Common derivable traits:
- `Debug`: Debug printing with `{:?}`
- `Clone`: Explicit duplication
- `Copy`: Implicit duplication (only for simple types)
- `PartialEq`/`Eq`: Equality comparison
- `Hash`: Hash map keys
- `Default`: Default value construction

**Deref Trait:**
```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

let m = MyBox::new(String::from("Rust"));
let s: &str = &m;  // Deref coercion: &MyBox<String> -> &String -> &str
```

**From and Into Traits:**
```rust
#[derive(Debug)]
struct Person {
    name: String,
}

impl From<String> for Person {
    fn from(name: String) -> Self {
        Person { name }
    }
}

// Into is automatically implemented when From is implemented
let person: Person = "Alice".to_string().into();
let person2 = Person::from("Bob".to_string());
```

---

## 6. Advanced Enums & Pattern Matching

### Enhanced Enum Modeling

**Complex Enum Variants:**
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

**Type Safety with Enums:**
```rust
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { session_id: String },
    Error { code: u32, message: String },
}

impl ConnectionState {
    fn is_connected(&self) -> bool {
        matches!(self, ConnectionState::Connected { .. })
    }
}
```

### Advanced Pattern Matching

**Destructuring in Match:**
```rust
enum Message {
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Move { x, y } => {
            println!("Move to coordinates: ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}
```

**Pattern Guards:**
```rust
fn categorize_number(x: i32) {
    match x {
        n if n < 0 => println!("Negative: {}", n),
        0 => println!("Zero"),
        n if n % 2 == 0 => println!("Positive even: {}", n),
        n => println!("Positive odd: {}", n),
    }
}
```

**Pattern Matching with Ranges:**
```rust
fn analyze_grade(grade: u32) {
    match grade {
        90..=100 => println!("A"),
        80..=89 => println!("B"),
        70..=79 => println!("C"),
        60..=69 => println!("D"),
        _ => println!("F"),
    }
}
```

### `if let` and `let else` Patterns

**`if let` for Simple Cases:**
```rust
let some_option = Some(42);

if let Some(value) = some_option {
    println!("Got value: {}", value);
}

// Equivalent to:
match some_option {
    Some(value) => println!("Got value: {}", value),
    None => (),
}
```

**`let else` Pattern (Rust 1.65+):**
```rust
fn process_config(config: Option<String>) {
    let Some(config_value) = config else {
        println!("No configuration provided");
        return;
    };
    
    println!("Using config: {}", config_value);
}
```

### Tuples and Tuple Structs

**Tuples:**
```rust
let point: (i32, i32) = (10, 20);
let (x, y) = point;  // Destructuring

// Accessing by index
println!("x: {}, y: {}", point.0, point.1);

// Different types in tuple
let mixed: (i32, f64, char) = (42, 3.14, 'a');
```

**Tuple Structs:**
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// black and origin are different types even though they have the same structure
```

**Unit Struct:**
```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

### Match Exhaustiveness

**Ensuring All Cases:**
```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(direction: Direction) {
    match direction {
        Direction::North => println!("Moving north"),
        Direction::South => println!("Moving south"),
        Direction::East => println!("Moving east"),
        Direction::West => println!("Moving west"),
        // No need for _ => because all variants are covered
    }
}
```

**Non-exhaustive Enums:**
```rust
#[non_exhaustive]
pub enum Error {
    NotFound,
    PermissionDenied,
    // More variants may be added in future versions
}

// When matching, must include catch-all:
match error {
    Error::NotFound => handle_not_found(),
    Error::PermissionDenied => handle_permission_denied(),
    _ => handle_unknown_error(), // Required for non_exhaustive enums
}
```

---

## 7. Error Handling & Option Types

### The Option Type

**Option Definition:**
```rust
enum Option<T> {
    Some(T),
    None,
}
```

**Using Option:**
```rust
fn divide(x: f64, y: f64) -> Option<f64> {
    if y != 0.0 {
        Some(x / y)
    } else {
        None
    }
}

let result = divide(10.0, 3.0);
match result {
    Some(value) => println!("Result: {}", value),
    None => println!("Cannot divide by zero"),
}
```

**Option Methods:**
```rust
let some_number = Some(5);
let no_number: Option<i32> = None;

// unwrap - panics if None
let value = some_number.unwrap();

// unwrap_or - provides default value
let value = no_number.unwrap_or(0);

// unwrap_or_else - computes default value
let value = no_number.unwrap_or_else(|| {
    println!("Computing default");
    42
});

// expect - panics with custom message
let value = some_number.expect("Should have a number");
```

### Result Type for Fallible Operations

**Result Definition:**
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**Using Result:**
```rust
use std::fs::File;
use std::io::ErrorKind;

fn open_file(filename: &str) -> Result<File, std::io::Error> {
    File::open(filename)
}

match open_file("hello.txt") {
    Ok(file) => println!("File opened successfully"),
    Err(error) => match error.kind() {
        ErrorKind::NotFound => println!("File not found"),
        ErrorKind::PermissionDenied => println!("Permission denied"),
        other_error => println!("Other error: {:?}", other_error),
    },
}
```

### Error Propagation

**The `?` Operator:**
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;  // Early return if error
    let mut username = String::new();
    file.read_to_string(&mut username)?;         // Early return if error
    Ok(username)
}

// Equivalent to:
fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let file = File::open("username.txt");
    let mut file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

**Chaining with `?`:**
```rust
fn process_data() -> Result<i32, Box<dyn std::error::Error>> {
    let data = fetch_data()?;
    let processed = transform_data(data)?;
    let result = finalize_data(processed)?;
    Ok(result)
}
```

### Option and Result Combinators

**Option Combinators:**
```rust
let number = Some(42);

// map - transform the value if present
let doubled = number.map(|x| x * 2);  // Some(84)

// and_then - chain operations that return Option
let result = number.and_then(|x| {
    if x > 40 {
        Some(x * 2)
    } else {
        None
    }
});  // Some(84)

// filter - keep value only if predicate is true
let filtered = number.filter(|&x| x > 50);  // None

// or - provide alternative Option
let alternative = None.or(Some(100));  // Some(100)

// or_else - compute alternative Option
let computed = None.or_else(|| Some(compute_default()));
```

**Result Combinators:**
```rust
let result: Result<i32, String> = Ok(42);

// map - transform the Ok value
let doubled = result.map(|x| x * 2);  // Ok(84)

// map_err - transform the Err value
let with_context = result.map_err(|e| format!("Error occurred: {}", e));

// and_then - chain operations that return Result
let chained = result.and_then(|x| {
    if x > 40 {
        Ok(x * 2)
    } else {
        Err("Too small".to_string())
    }
});

// or_else - handle error and provide alternative
let handled = result.or_else(|_| Ok(0));
```

### Null Pointer Elimination

**No Null in Rust:**
```rust
// This doesn't exist in Rust:
// let ptr = null;

// Instead, use Option:
let maybe_value: Option<&str> = None;
let definitely_value: &str = "hello";

// Compiler ensures you handle the None case:
if let Some(value) = maybe_value {
    println!("Got value: {}", value);
} else {
    println!("No value present");
}
```

**Safe Null Checks:**
```rust
fn get_first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}

let text = "Hello world";
match get_first_word(text) {
    Some(word) => println!("First word: {}", word),
    None => println!("No words found"),
}
```

---

## 8. Advanced Error Handling

### Custom Error Types

**Simple Error Enum:**
```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeLogarithm,
    Overflow,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
```

### Error Trait Implementation

**Display and Debug:**
```rust
use std::fmt;

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeLogarithm,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeLogarithm => write!(f, "Cannot take logarithm of negative number"),
            MathError::Overflow => write!(f, "Calculation resulted in overflow"),
        }
    }
}

impl std::error::Error for MathError {}
```

**Error with Context:**
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CalculationError {
    operation: String,
    source: Box<dyn Error>,
}

impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in {} operation", self.operation)
    }
}

impl Error for CalculationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
```

### TryFrom and TryInto Traits

**TryFrom for Fallible Conversions:**
```rust
use std::convert::TryFrom;

#[derive(Debug)]
struct PositiveNumber(u32);

#[derive(Debug)]
enum PositiveNumberError {
    Negative,
    Zero,
}

impl TryFrom<i32> for PositiveNumber {
    type Error = PositiveNumberError;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            i if i < 0 => Err(PositiveNumberError::Negative),
            0 => Err(PositiveNumberError::Zero),
            i => Ok(PositiveNumber(i as u32)),
        }
    }
}

// Usage:
let result = PositiveNumber::try_from(-5);
match result {
    Ok(num) => println!("Got positive number: {:?}", num),
    Err(e) => println!("Error: {:?}", e),
}

// TryInto is automatically implemented:
let result: Result<PositiveNumber, _> = 10.try_into();
```

### Error Boxing and Type Erasure

**Using Box<dyn Error>:**
```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_config() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("config.txt")?;  // io::Error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;       // io::Error
    
    let number: i32 = contents.trim().parse()?; // ParseIntError
    
    Ok(format!("Config value: {}", number))
}
```

**Custom Error Boxing:**
```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Parse(err) => write!(f, "Parse error: {}", err),
            AppError::Custom(msg) => write!(f, "Application error: {}", msg),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Parse(err) => Some(err),
            AppError::Custom(_) => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse(err)
    }
}
```

### Error Handling Best Practices

**Fail Fast vs Graceful Degradation:**
```rust
// Fail fast - panic on unrecoverable errors
fn calculate_factorial(n: u32) -> u64 {
    if n > 20 {
        panic!("Factorial too large to compute");
    }
    
    (1..=n).product::<u64>()
}

// Graceful degradation - return Result for recoverable errors
fn safe_factorial(n: u32) -> Result<u64, String> {
    if n > 20 {
        return Err("Number too large for factorial".to_string());
    }
    
    Ok((1..=n).product::<u64>())
}
```

**Early Return Pattern:**
```rust
fn validate_user_input(input: &str) -> Result<u32, String> {
    if input.is_empty() {
        return Err("Input cannot be empty".to_string());
    }
    
    if input.len() > 10 {
        return Err("Input too long".to_string());
    }
    
    let number = input.parse::<u32>()
        .map_err(|_| "Invalid number format".to_string())?;
    
    if number == 0 {
        return Err("Number cannot be zero".to_string());
    }
    
    Ok(number)
}
```

### Third-Party Error Handling Crates

**Using `thiserror` for Derive Macros:**
```rust
// Add to Cargo.toml: thiserror = "1.0"

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    
    #[error("unknown data store error")]
    Unknown,
}
```

**Using `anyhow` for Application Errors:**
```rust
// Add to Cargo.toml: anyhow = "1.0"

use anyhow::{Context, Result};

fn read_user_data(path: &str) -> Result<String> {
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;
    
    let processed = process_data(&data)
        .context("Failed to process user data")?;
    
    Ok(processed)
}
```

---

## 9. Package System & Dependencies

### Rust Package Structure

**Package Hierarchy:**
```
my_project/
├── Cargo.toml          # Package manifest
├── Cargo.lock          # Dependency lock file
├── src/
│   ├── main.rs         # Binary crate root
│   ├── lib.rs          # Library crate root
│   └── bin/
│       └── another_binary.rs
├── tests/              # Integration tests
├── examples/           # Example programs
└── benches/            # Benchmarks
```

**Cargo.toml Structure:**
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A sample Rust project"
license = "MIT OR Apache-2.0"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
clap = "4.0"

[dev-dependencies]
criterion = "0.4"

[features]
default = ["json_support"]
json_support = ["serde/json"]
async_support = ["tokio"]
```

### Library vs Binary Crates

**Library Crate (lib.rs):**
```rust
// src/lib.rs
pub mod config;
pub mod database;
pub mod utils;

pub use config::Config;
pub use database::Database;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let config = Config::new();
        assert!(config.is_valid());
    }
}
```

**Binary Crate (main.rs):**
```rust
// src/main.rs
use my_project::{Config, Database};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file("config.toml")?;
    let db = Database::connect(&config.database_url)?;
    
    println!("Application started successfully");
    Ok(())
}
```

### Module System Deep Dive

**Module Definition and Structure:**
```rust
// src/lib.rs
pub mod network {
    pub mod client {
        pub fn connect() -> Result<Connection, Error> {
            // Implementation
        }
    }
    
    pub mod server {
        pub fn listen(port: u16) -> Result<(), Error> {
            // Implementation
        }
    }
}

// Alternative: separate files
// src/network/mod.rs
pub mod client;
pub mod server;

// src/network/client.rs
pub fn connect() -> Result<Connection, Error> {
    // Implementation
}
```

**Visibility and Access Control:**
```rust
mod my_module {
    pub struct PublicStruct {
        pub public_field: i32,
        private_field: String,  // Private by default
    }
    
    impl PublicStruct {
        pub fn new(value: i32) -> Self {
            PublicStruct {
                public_field: value,
                private_field: String::new(),
            }
        }
        
        pub fn get_private(&self) -> &str {
            &self.private_field
        }
    }
    
    pub(crate) fn crate_visible_function() {
        // Visible within this crate only
    }
    
    pub(super) fn parent_visible_function() {
        // Visible to parent module only
    }
    
    fn private_function() {
        // Only visible within this module
    }
}
```

**Use Statements and Imports:**
```rust
// Bringing items into scope
use std::collections::HashMap;
use std::io::{self, Write, BufRead};

// Renaming imports
use std::io::Result as IoResult;

// Glob imports (use sparingly)
use std::collections::*;

// Re-exporting
pub use my_module::PublicStruct;

// Conditional compilation
#[cfg(target_os = "windows")]
use winapi::um::fileapi;

#[cfg(target_os = "linux")]
use libc::open;
```

### Dependency Management

**Version Specification:**
```toml
[dependencies]
# Caret requirements (compatible updates)
serde = "^1.0.0"     # Same as "1.0.0", allows 1.x.x
serde = "1.0"        # Same as above

# Tilde requirements (patch level updates)
regex = "~1.5.4"     # Allows 1.5.x

# Exact version
log = "=0.4.14"      # Exactly 0.4.14

# Range requirements
rand = ">=0.8, <0.9"

# Git dependencies
my_lib = { git = "https://github.com/user/repo.git", branch = "main" }

# Local path dependencies
utils = { path = "../utils" }
```

**Feature Gates:**
```toml
[features]
default = ["std"]
std = []
serde_support = ["serde", "serde_json"]
async_support = ["tokio", "futures"]

[dependencies]
serde = { version = "1.0", optional = true }
serde_json = { version = "1.0", optional = true }
tokio = { version = "1.0", optional = true }
```

**Using Features in Code:**
```rust
// Only compile when "serde_support" feature is enabled
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct MyStruct {
    pub field: String,
}

#[cfg(feature = "async_support")]
pub async fn async_function() {
    // Async implementation
}

#[cfg(not(feature = "async_support"))]
pub fn async_function() {
    panic!("Async support not enabled");
}
```

### Workspace Management

**Workspace Cargo.toml:**
```toml
[workspace]
members = [
    "core",
    "web",
    "cli",
]

[workspace.dependencies]
serde = "1.0"
tokio = "1.0"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
```

**Member Package:**
```toml
[package]
name = "core"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
web = { path = "../web" }
```

---

## 10. Collections & Data Structures

### Arrays and Fixed-Size Collections

**Array Fundamentals:**
```rust
// Array declaration
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

// Array access
let first = arr[0];
let slice = &arr[1..3];  // [2, 3]

// Arrays are stack-allocated and have fixed size
fn process_array(arr: [i32; 5]) {
    for element in arr {
        println!("{}", element);
    }
}
```

**Array vs Slice:**
```rust
fn take_array(arr: [i32; 5]) {
    // Takes ownership of the entire array
}

fn take_slice(slice: &[i32]) {
    // Takes a reference to any slice of i32
    println!("Length: {}", slice.len());
}

let arr = [1, 2, 3, 4, 5];
take_array(arr);           // Moves the array
// take_array(arr);        // Error: arr was moved

let arr2 = [1, 2, 3, 4, 5];
take_slice(&arr2);         // Borrows the array as a slice
take_slice(&arr2[1..3]);   // Borrows a portion
```

### Vectors - Dynamic Arrays

**Vector Creation and Manipulation:**
```rust
// Creating vectors
let mut v1: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3, 4, 5];
let v3 = Vec::with_capacity(10);  // Pre-allocate capacity

// Adding elements
v1.push(1);
v1.push(2);
v1.extend([3, 4, 5]);
v1.append(&mut vec![6, 7, 8]);

// Removing elements
let last = v1.pop();           // Returns Option<T>
let removed = v1.remove(0);    // Removes and returns element at index
v1.clear();                    // Removes all elements
```

**Vector Memory Layout:**
```rust
let mut v = Vec::with_capacity(4);
println!("Length: {}, Capacity: {}", v.len(), v.capacity());  // 0, 4

v.push(1);
v.push(2);
println!("Length: {}, Capacity: {}", v.len(), v.capacity());  // 2, 4

// When capacity is exceeded, vector reallocates with larger capacity
v.push(3);
v.push(4);
v.push(5);  // This may trigger reallocation
println!("Length: {}, Capacity: {}", v.len(), v.capacity());  // 5, 8 (or similar)
```

**Safe and Unsafe Access:**
```rust
let v = vec![1, 2, 3, 4, 5];

// Safe access with bounds checking
let third = v.get(2);    // Returns Option<&T>
match third {
    Some(value) => println!("Third element: {}", value),
    None => println!("No third element"),
}

// Direct indexing (panics if out of bounds)
let third = v[2];        // Returns T (or panics)

// Iterating safely
for (index, value) in v.iter().enumerate() {
    println!("Index {}: {}", index, value);
}
```

**Vector Iteration Modes:**
```rust
let mut v = vec![1, 2, 3, 4, 5];

// Immutable references
for item in &v {
    println!("{}", item);  // item: &i32
}

// Mutable references
for item in &mut v {
    *item *= 2;           // item: &mut i32
}

// Take ownership
for item in v {
    println!("{}", item);  // item: i32
}
// v is no longer accessible after this loop
```

### HashMap - Key-Value Storage

**HashMap Creation and Usage:**
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// Inserting values
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Alternative creation
let teams = vec![String::from("Blue"), String::from("Red")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```

**HashMap Access Patterns:**
```rust
let mut scores = HashMap::new();
scores.insert("Blue", 10);
scores.insert("Red", 50);

// Getting values
let blue_score = scores.get("Blue");  // Returns Option<&V>
match blue_score {
    Some(score) => println!("Blue team score: {}", score),
    None => println!("Blue team not found"),
}

// Direct access with default
let yellow_score = scores.get("Yellow").unwrap_or(&0);

// Checking existence
if scores.contains_key("Blue") {
    println!("Blue team exists");
}
```

**HashMap Modification Patterns:**
```rust
let mut scores = HashMap::new();

// Insert or update
scores.insert("Blue", 10);
scores.insert("Blue", 25);  // Updates existing value

// Insert only if key doesn't exist
scores.entry("Yellow").or_insert(50);
scores.entry("Blue").or_insert(0);   // Won't change existing value

// Update based on existing value
let blue_entry = scores.entry("Blue").or_insert(0);
*blue_entry += 10;

// Complex update logic
scores.entry("Green").and_modify(|score| *score += 10).or_insert(10);
```

**Hash and Eq Requirements:**
```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.age.hash(state);
    }
}

// Now Person can be used as HashMap key
let mut people = HashMap::new();
let person = Person {
    name: "Alice".to_string(),
    age: 30,
};
people.insert(person, "Engineer".to_string());
```

### BTreeMap - Ordered Key-Value Storage

**BTreeMap vs HashMap:**
```rust
use std::collections::{HashMap, BTreeMap};

// HashMap: O(1) average access, no ordering
let mut hash_map = HashMap::new();
hash_map.insert(3, "three");
hash_map.insert(1, "one");
hash_map.insert(2, "two");

// BTreeMap: O(log n) access, maintains order
let mut btree_map = BTreeMap::new();
btree_map.insert(3, "three");
btree_map.insert(1, "one");
btree_map.insert(2, "two");

// Iteration order differs
for (key, value) in &hash_map {
    println!("{}: {}", key, value);  // Arbitrary order
}

for (key, value) in &btree_map {
    println!("{}: {}", key, value);  // Ordered: 1, 2, 3
}
```

**BTreeMap Range Operations:**
```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(1, "one");
map.insert(3, "three");
map.insert(5, "five");
map.insert(7, "seven");
map.insert(9, "nine");

// Range queries
for (key, value) in map.range(3..7) {
    println!("{}: {}", key, value);  // 3: three, 5: five
}

// Split operations
let split_off = map.split_off(&5);  // Splits at key 5
// map now contains keys < 5, split_off contains keys >= 5
```

### Index and IndexMut Traits

**Custom Indexing:**
```rust
use std::ops::{Index, IndexMut};

struct Matrix {
    data: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0; cols]; rows],
            rows,
            cols,
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = i32;
    
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row][col]
    }
}

// Usage
let mut matrix = Matrix::new(3, 3);
matrix[(1, 2)] = 42;              // Uses IndexMut
let value = matrix[(1, 2)];       // Uses Index
```

### Slices and Borrowed Views

**Slice Fundamentals:**
```rust
let arr = [1, 2, 3, 4, 5];
let vec = vec![1, 2, 3, 4, 5];

// Creating slices
let slice1: &[i32] = &arr[1..4];        // [2, 3, 4]
let slice2: &[i32] = &vec[..3];         // [1, 2, 3]
let slice3: &[i32] = &arr[..];          // Entire array
let slice4: &[i32] = &vec[2..];         // [3, 4, 5]

// Slice methods
println!("Length: {}", slice1.len());
println!("First: {:?}", slice1.first());  // Option<&T>
println!("Last: {:?}", slice1.last());    // Option<&T>
```

**Slice Patterns and Operations:**
```rust
fn process_slice(data: &[i32]) {
    match data {
        [] => println!("Empty slice"),
        [single] => println!("Single element: {}", single),
        [first, rest @ ..] => {
            println!("First: {}, rest length: {}", first, rest.len());
        }
    }
}

// Slice splitting
let data = [1, 2, 3, 4, 5, 6];
let (left, right) = data.split_at(3);  // ([1, 2, 3], [4, 5, 6])

// Windows and chunks
let numbers = [1, 2, 3, 4, 5];
for window in numbers.windows(3) {
    println!("Window: {:?}", window);  // [1,2,3], [2,3,4], [3,4,5]
}

for chunk in numbers.chunks(2) {
    println!("Chunk: {:?}", chunk);    // [1,2], [3,4], [5]
}
```

**Mutable Slices:**
```rust
let mut data = [1, 2, 3, 4, 5];

// Mutable slice operations
let slice = &mut data[1..4];
slice[0] = 10;  // Changes data[1] to 10

// Slice sorting and manipulation
let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6];
numbers.sort();                    // In-place sorting
numbers.reverse();                 // In-place reversal

// Safe mutable splitting
let (left, right) = numbers.split_at_mut(4);
left[0] = 100;   // Modifies left portion
right[0] = 200;  // Modifies right portion
```

---

## 11. Lifetimes & References

### Lifetime Fundamentals

**What are Lifetimes:**
Lifetimes ensure that references are valid for as long as needed. They're part of Rust's type system that prevents dangling references.

```rust
fn main() {
    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("short");
        result = longest(&string1, &string2);  // Error: string2 doesn't live long enough
    }
    println!("Longest: {}", result);
}

// This function needs lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**Lifetime Annotation Syntax:**
```rust
// Lifetime parameters are declared after the function name
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Multiple lifetime parameters
fn announce_and_return<'a, 'b>(
    announcement: &'a str, 
    value: &'b str
) -> &'a str {
    println!("Attention: {}", announcement);
    announcement  // Return has lifetime 'a
}

// Lifetime constraints
fn longest_with_constraint<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    // 'b: 'a means 'b must live at least as long as 'a
    if x.len() > y.len() { x } else { y }
}
```

### Lifetime Elision Rules

**When Lifetimes Can Be Omitted:**
Rust applies three rules to infer lifetimes automatically:

1. **Input Rule**: Each parameter gets its own lifetime
2. **Single Input Rule**: If there's exactly one input lifetime, it's used for all outputs
3. **Method Rule**: If `&self` or `&mut self` is present, its lifetime is used for all outputs

```rust
// These are equivalent due to elision rules:

// Written explicitly
fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}

// Inferred by compiler
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// Method example - elision works with self
impl<'a> MyStruct<'a> {
    fn get_data(&self) -> &str {  // Lifetime inferred from &self
        &self.data
    }
}
```

**When Explicit Lifetimes Are Required:**
```rust
// Multiple input lifetimes, ambiguous output
fn longest(x: &str, y: &str) -> &str {  // Error: ambiguous lifetime
    if x.len() > y.len() { x } else { y }
}

// Fixed with explicit lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Lifetime Constraints and Bounds

**Lifetime Subtyping:**
```rust
// 'b: 'a means "'b outlives 'a" or "'b is at least as long as 'a"
fn copy_if_longer<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if y.len() > x.len() {
        y  // Valid because 'b: 'a
    } else {
        x
    }
}
```

**Static Lifetime:**
```rust
// 'static means the reference lives for the entire program duration
let s: &'static str = "Hello, world!";  // String literals have static lifetime

// Static lifetime constraint
fn get_static() -> &'static str {
    "This is a static string"
}

// Function requiring static lifetime
fn store_reference(r: &'static str) {
    // Can store this reference globally
}
```

**Lifetime Bounds in Structs:**
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    // Method with explicit lifetime
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part  // Uses lifetime from &self
    }
    
    // Method with different lifetime
    fn return_longer<'b>(&self, other: &'b str) -> &'b str 
    where 
        'a: 'b  // self.part's lifetime must outlive 'b
    {
        if self.part.len() > other.len() {
            self.part
        } else {
            other
        }
    }
}
```

### Iterator Lifetime Relationships

**Iterator Borrowing:**
```rust
fn find_words_starting_with<'a>(
    text: &'a str, 
    prefix: &str
) -> impl Iterator<Item = &'a str> {
    text.split_whitespace()
        .filter(move |word| word.starts_with(prefix))
}

// Usage
let text = "hello world rust programming";
let rust_words: Vec<&str> = find_words_starting_with(text, "ru").collect();
```

**Lifetime Challenges with Iterators:**
```rust
// This won't compile - returning references to local data
fn bad_iterator() -> impl Iterator<Item = &str> {
    let data = vec!["a", "b", "c"];
    data.iter().map(|s| *s)  // Error: data doesn't live long enough
}

// Solutions:
// 1. Return owned data
fn good_iterator_owned() -> impl Iterator<Item = String> {
    vec!["a", "b", "c"].into_iter().map(|s| s.to_string())
}

// 2. Take input with appropriate lifetime
fn good_iterator_borrowed<'a>(data: &'a [&str]) -> impl Iterator<Item = &'a str> {
    data.iter().copied()
}
```

### Common Lifetime Patterns

**Builder Pattern with Lifetimes:**
```rust
struct QueryBuilder<'a> {
    table: &'a str,
    conditions: Vec<&'a str>,
}

impl<'a> QueryBuilder<'a> {
    fn new(table: &'a str) -> Self {
        QueryBuilder {
            table,
            conditions: Vec::new(),
        }
    }
    
    fn add_condition(mut self, condition: &'a str) -> Self {
        self.conditions.push(condition);
        self
    }
    
    fn build(&self) -> String {
        format!("SELECT * FROM {} WHERE {}", 
                self.table, 
                self.conditions.join(" AND "))
    }
}
```

**Lifetime in Error Types:**
```rust
#[derive(Debug)]
struct ValidationError<'a> {
    field: &'a str,
    message: &'a str,
}

fn validate_user<'a>(name: &'a str, email: &'a str) -> Result<(), ValidationError<'a>> {
    if name.is_empty() {
        return Err(ValidationError {
            field: "name",
            message: "Name cannot be empty",
        });
    }
    
    if !email.contains('@') {
        return Err(ValidationError {
            field: "email", 
            message: "Invalid email format",
        });
    }
    
    Ok(())
}
```

**Higher-Ranked Trait Bounds (HRTB):**
```rust
// for<'a> syntax for higher-ranked trait bounds
fn call_with_ref<F>(f: F) 
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = String::from("hello");
    let result = f(&s);
    println!("Result: {}", result);
}

// This closure works with any lifetime
call_with_ref(|s| s);
```

---

## 12. Iterators & Functional Programming

### Iterator Trait Deep Dive

**Core Iterator Trait:**
```rust
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // Many default implementations provided:
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
    fn count(self) -> usize { /* default implementation */ }
    fn last(self) -> Option<Self::Item> { /* default implementation */ }
    // ... many more methods
}
```

**IntoIterator Trait:**
```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter;
}

// Examples of IntoIterator implementations:
let vec = vec![1, 2, 3];

// These all use different IntoIterator implementations:
for item in vec {}           // Takes ownership
for item in &vec {}          // Borrows immutably
for item in &mut vec {}      // Borrows mutably
```

### Creating Custom Iterators

**Implementing Iterator:**
```rust
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
    
    // Optional: provide size_hint for optimization
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.max - self.current;
        (remaining, Some(remaining))
    }
}

// Usage
let counter = Counter::new(5);
for num in counter {
    println!("{}", num);  // 0, 1, 2, 3, 4
}
```

**Iterator for Custom Collections:**
```rust
struct MyVec<T> {
    data: Vec<T>,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec { data: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.data.push(item);
    }
}

// Implementing IntoIterator to make MyVec iterable
impl<T> IntoIterator for MyVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> IntoIterator for &MyVec<T> {
    type Item = &T;
    type IntoIter = std::slice::Iter<'_, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

// Usage
let mut my_vec = MyVec::new();
my_vec.push(1);
my_vec.push(2);

for item in &my_vec {
    println!("{}", item);  // Borrows
}

for item in my_vec {
    println!("{}", item);  // Takes ownership
}
```

### Iterator Adaptors vs Consumers

**Iterator Adaptors (Lazy):**
Iterator adaptors transform one iterator into another but don't consume the iterator until a consumer is called.

```rust
let vec = vec![1, 2, 3, 4, 5];

// These are lazy - no work is done yet:
let iter = vec.iter()
    .map(|x| x * 2)        // Transform each element
    .filter(|&&x| x > 4)   // Keep elements > 4
    .enumerate();          // Add indices

// Only when we consume the iterator is work done:
let results: Vec<_> = iter.collect();
println!("{:?}", results);  // [(0, 6), (1, 8), (2, 10)]
```

**Common Iterator Adaptors:**
```rust
let numbers = vec![1, 2, 3, 4, 5];

// map - transform each element
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// filter - keep elements matching predicate
let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();

// enumerate - add indices
let with_indices: Vec<(usize, &i32)> = numbers.iter().enumerate().collect();

// zip - combine with another iterator
let letters = vec!['a', 'b', 'c'];
let paired: Vec<(&i32, &char)> = numbers.iter().zip(letters.iter()).collect();

// take/skip - limit elements
let first_three: Vec<&i32> = numbers.iter().take(3).collect();
let skip_two: Vec<&i32> = numbers.iter().skip(2).collect();

// chain - connect iterators
let more_numbers = vec![6, 7, 8];
let chained: Vec<&i32> = numbers.iter().chain(more_numbers.iter()).collect();

// flatten - flatten nested structures
let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
let flattened: Vec<&i32> = nested.iter().flatten().collect();
```

**Consumers (Eager):**
Consumers actually execute the iterator chain and produce a final result.

```rust
let numbers = vec![1, 2, 3, 4, 5];

// collect - gather into collection
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let as_set: std::collections::HashSet<i32> = numbers.into_iter().collect();

// reduce/fold - accumulate values
let sum = numbers.iter().fold(0, |acc, x| acc + x);
let product = numbers.iter().reduce(|acc, x| acc * x);  // Returns Option

// find - locate element
let found = numbers.iter().find(|&&x| x > 3);  // Some(&4)

// any/all - boolean tests
let has_even = numbers.iter().any(|&x| x % 2 == 0);
let all_positive = numbers.iter().all(|&x| x > 0);

// for_each - side effects
numbers.iter().for_each(|x| println!("{}", x));

// count - count elements
let count = numbers.iter().filter(|&&x| x > 2).count();

// min/max - find extremes
let min = numbers.iter().min();  // Some(&1)
let max = numbers.iter().max();  // Some(&5)
```

### Advanced Iterator Patterns

**Custom Adaptors:**
```rust
// Extension trait for custom iterator methods
trait IteratorExt: Iterator {
    fn my_custom_map<F, B>(self, f: F) -> impl Iterator<Item = B>
    where
        Self: Sized,
        F: Fn(Self::Item) -> B,
    {
        self.map(f)  // Can build on existing adaptors
    }
    
    fn debug_each(self) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: std::fmt::Debug,
    {
        self.map(|item| {
            println!("Processing: {:?}", item);
            item
        })
    }
}

// Implement for all iterators
impl<I: Iterator> IteratorExt for I {}

// Usage
let results: Vec<i32> = vec![1, 2, 3]
    .into_iter()
    .debug_each()
    .my_custom_map(|x| x * 2)
    .collect();
```

**Stateful Iterator Adaptors:**
```rust
// Custom adaptor that keeps running total
struct RunningSum<I> {
    iter: I,
    sum: i32,
}

impl<I> Iterator for RunningSum<I>
where
    I: Iterator<Item = i32>,
{
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(value) => {
                self.sum += value;
                Some(self.sum)
            }
            None => None,
        }
    }
}

// Extension trait to add the adaptor
trait RunningSumExt: Iterator<Item = i32> {
    fn running_sum(self) -> RunningSum<Self>
    where
        Self: Sized,
    {
        RunningSum { iter: self, sum: 0 }
    }
}

impl<I: Iterator<Item = i32>> RunningSumExt for I {}

// Usage
let sums: Vec<i32> = vec![1, 2, 3, 4]
    .into_iter()
    .running_sum()
    .collect();
println!("{:?}", sums);  // [1, 3, 6, 10]
```

### Performance Considerations

**Zero-Cost Abstractions:**
```rust
// These compile to equivalent assembly:

// Functional style
let sum: i32 = (0..1_000_000)
    .map(|x| x * x)
    .filter(|&x| x % 2 == 0)
    .sum();

// Imperative style
let mut sum = 0;
for i in 0..1_000_000 {
    let squared = i * i;
    if squared % 2 == 0 {
        sum += squared;
    }
}
```

**Iterator vs For Loop Performance:**
```rust
// Iterator (often faster due to optimizations)
fn sum_iterator(data: &[i32]) -> i32 {
    data.iter().sum()
}

// Manual loop
fn sum_manual(data: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in data {
        sum += item;
    }
    sum
}

// Collect vs extend performance
let mut vec = Vec::new();

// Slower - multiple allocations
for item in source.iter().map(|x| x * 2) {
    vec.push(item);
}

// Faster - size hint allows better allocation
vec.extend(source.iter().map(|x| x * 2));

// Often fastest - single allocation
let vec: Vec<_> = source.iter().map(|x| x * 2).collect();
```

**Avoiding Unnecessary Collections:**
```rust
// Inefficient - creates intermediate vector
let result: Vec<i32> = data
    .iter()
    .map(|x| x * 2)
    .collect::<Vec<_>>()  // Unnecessary intermediate collection
    .into_iter()
    .filter(|&x| x > 10)
    .collect();

// Efficient - no intermediate collection
let result: Vec<i32> = data
    .iter()
    .map(|x| x * 2)
    .filter(|&x| x > 10)
    .collect();
```

### `impl Trait` in Iterator Context

**Returning Complex Iterator Types:**
```rust
// Without impl Trait (verbose and exposes implementation details)
fn complex_iterator_verbose(data: Vec<i32>) 
    -> std::iter::Map<
        std::iter::Filter<std::vec::IntoIter<i32>, fn(&i32) -> bool>,
        fn(i32) -> String
    > 
{
    data.into_iter()
        .filter(|&x| x > 0)
        .map(|x| format!("Item: {}", x))
}

// With impl Trait (clean and flexible)
fn complex_iterator(data: Vec<i32>) -> impl Iterator<Item = String> {
    data.into_iter()
        .filter(|&x| x > 0)
        .map(|x| format!("Item: {}", x))
}

// Can change implementation without breaking API
fn complex_iterator_v2(data: Vec<i32>) -> impl Iterator<Item = String> {
    data.into_iter()
        .filter_map(|x| {
            if x > 0 {
                Some(format!("Item: {}", x))
            } else {
                None
            }
        })
}
```

**Conditional Iterator Types:**
```rust
// Use Box<dyn Iterator> for dynamic dispatch when needed
fn conditional_iterator(use_filter: bool, data: Vec<i32>) 
    -> Box<dyn Iterator<Item = i32>> 
{
    if use_filter {
        Box::new(data.into_iter().filter(|&x| x > 0))
    } else {
        Box::new(data.into_iter())
    }
}
```

---

## 13. Concurrency - Threads

### Thread Fundamentals

**Creating and Managing Threads:**
```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Main thread continues executing
    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for the spawned thread to complete
    handle.join().unwrap();
    println!("Thread completed");
}
```

**Capturing Variables in Threads:**
```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Move ownership into the thread
    let handle = thread::spawn(move || {
        println!("Thread received: {:?}", data);
        // data is now owned by this thread
    });
    
    // println!("{:?}", data); // Error: data was moved
    
    handle.join().unwrap();
}
```

**Thread Builder Pattern:**
```rust
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("worker-thread".into())
        .stack_size(32 * 1024); // 32KB stack
    
    let handle = builder.spawn(|| {
        println!("Hello from named thread: {:?}", thread::current().name());
    }).unwrap();
    
    handle.join().unwrap();
}
```

### Message Passing Between Threads

**Single Producer, Single Consumer Channel:**
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec![
            "Hello",
            "from",
            "the",
            "thread",
        ];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive messages
    for received in rx {
        println!("Received: {}", received);
    }
}
```

**Multiple Producer Pattern:**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // Spawn multiple producer threads
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(format!("Message from thread {}", i)).unwrap();
        });
    }
    
    // Close the original sender
    drop(tx);
    
    // Receive all messages
    for received in rx {
        println!("Got: {}", received);
    }
}
```

**Synchronous Channel (Bounded):**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // Channel with capacity of 2
    let (tx, rx) = mpsc::sync_channel(2);
    
    let tx_clone = tx.clone();
    thread::spawn(move || {
        for i in 1..=5 {
            println!("Sending {}", i);
            tx_clone.send(i).unwrap(); // Will block when buffer is full
            println!("Sent {}", i);
        }
    });
    
    thread::sleep(std::time::Duration::from_millis(500));
    
    for received in rx {
        println!("Received: {}", received);
        thread::sleep(std::time::Duration::from_millis(200));
    }
}
```

### Shared State Concurrency

**Mutex for Mutual Exclusion:**
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}
```

**Handling Mutex Poison:**
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);
    
    let handle = thread::spawn(move || {
        let mut v = data_clone.lock().unwrap();
        v.push(4);
        panic!("Thread panicked!"); // This poisons the mutex
    });
    
    let _ = handle.join(); // Ignore the panic result
    
    // Handle poisoned mutex
    match data.lock() {
        Ok(v) => println!("Data: {:?}", *v),
        Err(poisoned) => {
            println!("Mutex was poisoned, recovering data");
            let v = poisoned.into_inner();
            println!("Recovered data: {:?}", *v);
        }
    }
}
```

**RwLock for Read-Write Access:**
```rust
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // Spawn reader threads
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let r = data.read().unwrap();
            println!("Reader {}: {:?}", i, *r);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // Spawn writer thread
    let data = Arc::clone(&data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut w = data.write().unwrap();
        w.push(6);
        println!("Writer: Added element");
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final data: {:?}", *data.read().unwrap());
}
```

### Thread Synchronization Primitives

**Barrier for Thread Synchronization:**
```rust
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} is working...", i);
            thread::sleep(Duration::from_millis(100 * i as u64));
            
            println!("Thread {} reached barrier", i);
            barrier.wait(); // Wait for all threads to reach this point
            
            println!("Thread {} passed barrier", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

**Condvar for Conditional Waiting:**
```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);
    
    // Spawned thread
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });
    
    // Main thread
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    
    while !*started {
        println!("Waiting for condition...");
        started = cvar.wait(started).unwrap();
    }
    
    println!("Condition met!");
}
```

### Thread Safety Traits

**Send and Sync Traits:**
```rust
// Send: Types that can be transferred between threads
// Sync: Types that can be safely accessed from multiple threads

use std::rc::Rc;
use std::sync::Arc;

fn main() {
    // Arc<T> is Send + Sync if T is Send + Sync
    let shared_data = Arc::new(vec![1, 2, 3]);
    
    let handle = std::thread::spawn({
        let data = Arc::clone(&shared_data);
        move || {
            println!("Thread data: {:?}", data);
        }
    });
    
    handle.join().unwrap();
    
    // Rc<T> is neither Send nor Sync
    let rc_data = Rc::new(vec![1, 2, 3]);
    
    // This won't compile:
    // let handle = std::thread::spawn(move || {
    //     println!("Thread data: {:?}", rc_data);
    // });
}
```

**Implementing Send and Sync:**
```rust
use std::marker::PhantomData;

// Custom type that is Send but not Sync
struct SendOnly<T> {
    data: T,
    _phantom: PhantomData<*const ()>, // Makes it !Sync
}

unsafe impl<T: Send> Send for SendOnly<T> {}
// Deliberately not implementing Sync

// Custom type that needs manual Send/Sync implementation
struct RawPointerWrapper {
    ptr: *const i32,
}

// SAFETY: We're ensuring the pointer is always valid
unsafe impl Send for RawPointerWrapper {}
unsafe impl Sync for RawPointerWrapper {}
```

### Thread Pool Pattern

**Simple Thread Pool:**
```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });
        
        Worker { id, thread }
    }
}

// Usage
fn main() {
    let pool = ThreadPool::new(4);
    
    for i in 0..8 {
        pool.execute(move || {
            println!("Job {} executing", i);
            thread::sleep(std::time::Duration::from_millis(100));
        });
    }
    
    thread::sleep(std::time::Duration::from_millis(1000));
}
```

---

## 14. Concurrency - Async/Await

### Asynchronous Programming Fundamentals

**Async Functions and Futures:**
```rust
// Async function returns a Future
async fn hello_world() {
    println!("Hello, world!");
}

// Equivalent manual implementation
fn hello_world_manual() -> impl std::future::Future<Output = ()> {
    async {
        println!("Hello, world!");
    }
}

// Async function with return value
async fn get_number() -> i32 {
    42
}

// Async function with parameters
async fn process_data(data: String) -> String {
    format!("Processed: {}", data)
}
```

**The Future Trait:**
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Simplified Future trait
trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),    // Future completed with result
    Pending,     // Future not ready, will be polled again later
}
```

**Awaiting Futures:**
```rust
async fn main_example() {
    let result = get_number().await;
    println!("Got number: {}", result);
    
    let processed = process_data("hello".to_string()).await;
    println!("{}", processed);
}
```

### Async Runtimes

**Tokio Runtime Setup:**
```rust
// Automatic runtime with macro
#[tokio::main]
async fn main() {
    println!("Hello from async main!");
    hello_world().await;
}

// Manual runtime setup
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("Hello from manual runtime!");
        hello_world().await;
    });
}

// Single-threaded runtime
#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Single-threaded async runtime");
}

// Multi-threaded runtime (default)
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    println!("Multi-threaded runtime with 4 workers");
}
```

**Runtime Configuration:**
```rust
use tokio::runtime::{Builder, Runtime};

fn custom_runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(8)
        .thread_name("my-async-worker")
        .thread_stack_size(3 * 1024 * 1024) // 3MB stack
        .enable_all() // Enable I/O and time drivers
        .build()
        .unwrap()
}

fn main() {
    let rt = custom_runtime();
    rt.block_on(async {
        // Your async code here
    });
}
```

### Async Task Management

**Spawning Async Tasks:**
```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // Spawn a task that runs concurrently
    let handle1 = task::spawn(async {
        for i in 1..=5 {
            println!("Task 1: {}", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        "Task 1 complete"
    });
    
    let handle2 = task::spawn(async {
        for i in 1..=3 {
            println!("Task 2: {}", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        }
        "Task 2 complete"
    });
    
    // Wait for both tasks to complete
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
    
    println!("{}, {}", result1, result2);
}
```

**Task Joining and Selection:**
```rust
use tokio::{join, select, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    // join! waits for ALL tasks to complete
    let (result1, result2, result3) = join!(
        sleep(Duration::from_millis(100)),
        sleep(Duration::from_millis(200)),
        sleep(Duration::from_millis(150))
    );
    println!("All tasks completed");
    
    // select! waits for the FIRST task to complete
    select! {
        _ = sleep(Duration::from_millis(100)) => {
            println!("First timer finished");
        }
        _ = sleep(Duration::from_millis(200)) => {
            println!("Second timer finished");
        }
        _ = sleep(Duration::from_millis(50)) => {
            println!("Third timer finished"); // This will likely be selected
        }
    }
}
```

### Error Handling in Async Code

**Result and Option in Async:**
```rust
use std::error::Error;

async fn might_fail() -> Result<String, Box<dyn Error>> {
    // Simulate async work that might fail
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    if rand::random::<bool>() {
        Ok("Success!".to_string())
    } else {
        Err("Something went wrong".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match might_fail().await {
        Ok(result) => println!("Got: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Using ? operator
    let result = might_fail().await?;
    println!("Success: {}", result);
    
    Ok(())
}
```

**Timeout and Cancellation:**
```rust
use tokio::time::{timeout, Duration};

async fn slow_operation() -> Result<String, &'static str> {
    tokio::time::sleep(Duration::from_millis(2000)).await;
    Ok("Completed".to_string())
}

#[tokio::main]
async fn main() {
    // Timeout after 1 second
    match timeout(Duration::from_millis(1000), slow_operation()).await {
        Ok(result) => {
            match result {
                Ok(value) => println!("Got: {}", value),
                Err(e) => println!("Operation failed: {}", e),
            }
        }
        Err(_) => println!("Operation timed out"),
    }
}
```

### Async Constraints and Lifetimes

**Send Bound for Spawned Tasks:**
```rust
use std::rc::Rc;

#[tokio::main]
async fn main() {
    let data = "Hello".to_string();
    
    // This works - String is Send
    let handle = tokio::task::spawn(async move {
        println!("{}", data);
    });
    
    handle.await.unwrap();
    
    // This won't compile - Rc is not Send
    let rc_data = Rc::new("Hello".to_string());
    
    // tokio::task::spawn(async move {
    //     println!("{}", rc_data); // Error: Rc is not Send
    // });
    
    // Solution: use Arc instead
    let arc_data = std::sync::Arc::new("Hello".to_string());
    let handle = tokio::task::spawn(async move {
        println!("{}", arc_data);
    });
    
    handle.await.unwrap();
}
```

**'static Lifetime Requirement:**
```rust
async fn process_data(data: &'static str) -> String {
    format!("Processed: {}", data)
}

#[tokio::main]
async fn main() {
    // This works - string literal has 'static lifetime
    let result = process_data("hello").await;
    println!("{}", result);
    
    // For owned data, move it into the async block
    let owned_data = "hello".to_string();
    let handle = tokio::task::spawn(async move {
        process_owned_data(owned_data).await
    });
    
    handle.await.unwrap();
}

async fn process_owned_data(data: String) -> String {
    format!("Processed: {}", data)
}
```

### Async I/O and Networking

**File I/O:**
```rust
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Write to file
    fs::write("example.txt", "Hello, async world!").await?;
    
    // Read from file
    let contents = fs::read_to_string("example.txt").await?;
    println!("File contents: {}", contents);
    
    // Manual file operations
    let mut file = fs::File::create("manual.txt").await?;
    file.write_all(b"Manual write").await?;
    
    let mut file = fs::File::open("manual.txt").await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    println!("Manual read: {}", buffer);
    
    Ok(())
}
```

**TCP Networking:**
```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    
    loop {
        let n = socket.read(&mut buffer).await?;
        
        if n == 0 {
            break; // Connection closed
        }
        
        // Echo back to client
        socket.write_all(&buffer[0..n]).await?;
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client: {}", addr);
        
        // Spawn a task for each client
        tokio::task::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}
```

### Async Traits and Advanced Patterns

**Async Traits (using async-trait crate):**
```rust
use async_trait::async_trait;

#[async_trait]
trait AsyncProcessor {
    async fn process(&self, data: String) -> Result<String, String>;
}

struct SimpleProcessor;

#[async_trait]
impl AsyncProcessor for SimpleProcessor {
    async fn process(&self, data: String) -> Result<String, String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(format!("Processed: {}", data))
    }
}

#[tokio::main]
async fn main() {
    let processor = SimpleProcessor;
    match processor.process("hello".to_string()).await {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

**Stream Processing:**
```rust
use tokio_stream::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

// Custom stream implementation
struct NumberStream {
    current: usize,
    max: usize,
}

impl NumberStream {
    fn new(max: usize) -> Self {
        NumberStream { current: 0, max }
    }
}

impl Stream for NumberStream {
    type Item = usize;
    
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Poll::Ready(Some(current))
        } else {
            Poll::Ready(None)
        }
    }
}

#[tokio::main]
async fn main() {
    let mut stream = NumberStream::new(5);
    
    while let Some(number) = stream.next().await {
        println!("Got: {}", number);
    }
    
    // Stream combinators
    let doubled: Vec<usize> = NumberStream::new(5)
        .map(|x| x * 2)
        .collect()
        .await;
    
    println!("Doubled: {:?}", doubled);
}
```

**Channels for Async Communication:**
```rust
use tokio::sync::{mpsc, oneshot, broadcast};

#[tokio::main]
async fn main() {
    // MPSC (Multiple Producer, Single Consumer)
    let (tx, mut rx) = mpsc::channel(32);
    
    tokio::task::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });
    
    while let Some(value) = rx.recv().await {
        println!("MPSC received: {}", value);
    }
    
    // Oneshot (one-time communication)
    let (tx, rx) = oneshot::channel();
    
    tokio::task::spawn(async move {
        tx.send("Hello from oneshot").unwrap();
    });
    
    let result = rx.await.unwrap();
    println!("Oneshot received: {}", result);
    
    // Broadcast (one-to-many)
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    
    tokio::task::spawn(async move {
        tx.send("Broadcast message").unwrap();
    });
    
    println!("Receiver 1: {}", rx1.recv().await.unwrap());
    println!("Receiver 2: {}", rx2.recv().await.unwrap());
}
```

---

## 15. Smart Pointers & Memory Management

### Box<T> - Heap Allocation

**Basic Usage:**
```rust
// Allocate data on the heap
let boxed_int = Box::new(42);
println!("Value: {}", boxed_int);

// Useful for large data that would overflow the stack
let large_array = Box::new([0; 1_000_000]);

// Enable recursive types
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

**Box Patterns:**
```rust
// Pattern matching with Box
fn sum_list(list: Box<List>) -> i32 {
    match *list {
        Cons(head, tail) => head + sum_list(tail),
        Nil => 0,
    }
}

// Box as trait object
trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

### Rc<T> - Reference Counting

**Shared Ownership:**
```rust
use std::rc::Rc;

let data = Rc::new(vec![1, 2, 3, 4]);
let data1 = Rc::clone(&data);  // Increment reference count
let data2 = Rc::clone(&data);  // Increment reference count

println!("Reference count: {}", Rc::strong_count(&data)); // 3

// All references share the same data
println!("Data1: {:?}", data1);
println!("Data2: {:?}", data2);
```

**Rc with Interior Mutability:**
```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
let data1 = Rc::clone(&shared_data);
let data2 = Rc::clone(&shared_data);

// Mutate through RefCell
data1.borrow_mut().push(4);

println!("Data2: {:?}", data2.borrow()); // [1, 2, 3, 4]
```

### RefCell<T> - Interior Mutability

**Runtime Borrow Checking:**
```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);

// Immutable borrow
{
    let borrowed = data.borrow();
    println!("Length: {}", borrowed.len());
} // borrowed goes out of scope

// Mutable borrow
{
    let mut borrowed = data.borrow_mut();
    borrowed.push(4);
}

// Multiple immutable borrows are OK
let borrow1 = data.borrow();
let borrow2 = data.borrow();
println!("Data: {:?} {:?}", borrow1, borrow2);
```

**RefCell Patterns:**
```rust
use std::cell::RefCell;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
    
    fn send(&self, message: &str) {
        // Can mutate even with &self
        self.sent_messages.borrow_mut().push(String::from(message));
    }
    
    fn message_count(&self) -> usize {
        self.sent_messages.borrow().len()
    }
}
```

### Arc<T> - Atomic Reference Counting

**Thread-Safe Shared Ownership:**
```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3, 4, 5]);
let mut handles = vec![];

for i in 0..3 {
    let data = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("Thread {}: {:?}", i, data);
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

**Arc with Mutex:**
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

### Weak<T> - Breaking Reference Cycles

**Preventing Memory Leaks:**
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        child.parent.borrow_mut().clone_from(&Rc::downgrade(parent));
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}

// Usage
let parent = Node::new(1);
let child1 = Node::new(2);
let child2 = Node::new(3);

Node::add_child(&parent, &child1);
Node::add_child(&parent, &child2);

// Access parent from child (if it still exists)
if let Some(parent) = child1.parent.borrow().upgrade() {
    println!("Child's parent: {}", parent.value);
}
```

### Memory Layout and Performance

**Choosing the Right Smart Pointer:**
```rust
// Stack allocation - fastest
let value = 42;

// Heap allocation - when size is large or unknown
let boxed = Box::new(expensive_computation());

// Shared ownership - single-threaded
use std::rc::Rc;
let shared = Rc::new(data);

// Shared ownership - multi-threaded
use std::sync::Arc;
let thread_safe_shared = Arc::new(data);

// Interior mutability - single-threaded
use std::cell::RefCell;
let mutable = RefCell::new(data);

// Interior mutability - multi-threaded
use std::sync::Mutex;
let thread_safe_mutable = Mutex::new(data);
```

**Performance Characteristics:**
```rust
// Performance comparison
struct PerformanceTest {
    // Direct - no indirection, stack allocated
    direct: i32,
    
    // Box - one indirection, heap allocated
    boxed: Box<i32>,
    
    // Rc - reference counting overhead
    shared: Rc<i32>,
    
    // Arc - atomic reference counting overhead
    atomic_shared: Arc<i32>,
}

// Choose based on needs:
// - Direct: fastest access
// - Box: when you need heap allocation
// - Rc: when you need shared ownership (single-threaded)
// - Arc: when you need shared ownership (multi-threaded)
```

---

## 16. Testing & Documentation

### Unit Testing

**Basic Test Structure:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn test_subtraction() {
        assert_eq!(5 - 3, 2);
    }
    
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test should panic");
    }
    
    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_specific_panic() {
        divide_by_zero();
    }
}

fn divide_by_zero() {
    panic!("divide by zero");
}
```

**Testing with Results:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_with_result() -> Result<(), String> {
        let result = risky_operation()?;
        assert_eq!(result, 42);
        Ok(())
    }
    
    #[test]
    fn test_error_conditions() {
        let result = might_fail(false);
        assert!(result.is_err());
        
        let result = might_fail(true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }
}

fn risky_operation() -> Result<i32, String> {
    Ok(42)
}

fn might_fail(should_succeed: bool) -> Result<&'static str, &'static str> {
    if should_succeed {
        Ok("success")
    } else {
        Err("failure")
    }
}
```

**Custom Test Assertions:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_custom_assertions() {
        let user = User::new("Alice", 30);
        
        // Custom assertion helper
        assert_user_valid(&user);
        
        // Testing floating point numbers
        let result = 0.1 + 0.2;
        assert!((result - 0.3).abs() < f64::EPSILON);
        
        // Testing collections
        let vec = vec![1, 2, 3];
        assert_eq!(vec.len(), 3);
        assert!(vec.contains(&2));
    }
    
    fn assert_user_valid(user: &User) {
        assert!(!user.name().is_empty(), "User name should not be empty");
        assert!(user.age() > 0, "User age should be positive");
    }
}

struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: &str, age: u32) -> Self {
        User {
            name: name.to_string(),
            age,
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn age(&self) -> u32 {
        self.age
    }
}
```

### Integration Testing

**Integration Test Structure:**
```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn test_public_api() {
    let result = my_crate::public_function(42);
    assert_eq!(result, 84);
}

#[test]
fn test_complex_workflow() {
    let mut system = my_crate::System::new();
    system.initialize();
    
    let result = system.process_data("test input");
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert_eq!(output, "processed: test input");
}
```

**Test Organization:**
```rust
// tests/common/mod.rs
pub fn setup() -> TestEnvironment {
    TestEnvironment::new()
}

pub struct TestEnvironment {
    // Setup common test infrastructure
}

impl TestEnvironment {
    pub fn new() -> Self {
        TestEnvironment {}
    }
    
    pub fn create_test_user(&self) -> User {
        User::new("test_user", 25)
    }
}

// tests/user_tests.rs
mod common;

use common::setup;

#[test]
fn test_user_creation() {
    let env = setup();
    let user = env.create_test_user();
    assert_eq!(user.name(), "test_user");
}
```

### Documentation Testing

**Doc Tests:**
```rust
/// Calculates the factorial of a number.
///
/// # Examples
///
/// ```
/// use my_crate::factorial;
///
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// ```
///
/// # Panics
///
/// This function will panic if the input is negative.
///
/// ```should_panic
/// use my_crate::factorial;
///
/// factorial(-1); // This will panic
/// ```
pub fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

/// A user in the system.
///
/// # Examples
///
/// ```
/// use my_crate::User;
///
/// let user = User::new("Alice", 30);
/// assert_eq!(user.name(), "Alice");
/// assert_eq!(user.age(), 30);
/// ```
pub struct User {
    name: String,
    age: u32,
}

impl User {
    /// Creates a new user.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::User;
    ///
    /// let user = User::new("Bob", 25);
    /// ```
    pub fn new(name: &str, age: u32) -> Self {
        User {
            name: name.to_string(),
            age,
        }
    }
    
    /// Returns the user's name.
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Returns the user's age.
    pub fn age(&self) -> u32 {
        self.age
    }
}
```

### Benchmarking

**Criterion Benchmarks:**
```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use my_crate::*;

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

fn comparison_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("algorithms");
    
    group.bench_function("bubble_sort", |b| {
        b.iter(|| {
            let mut data = vec![64, 34, 25, 12, 22, 11, 90];
            bubble_sort(black_box(&mut data));
        })
    });
    
    group.bench_function("quick_sort", |b| {
        b.iter(|| {
            let mut data = vec![64, 34, 25, 12, 22, 11, 90];
            quick_sort(black_box(&mut data));
        })
    });
    
    group.finish();
}

criterion_group!(benches, fibonacci_benchmark, comparison_benchmark);
criterion_main!(benches);

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = len - 1;
    let mut i = 0;
    
    for j in 0..len - 1 {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, pivot);
    i
}
```

### Documentation Best Practices

**Comprehensive Documentation:**
```rust
//! # My Crate
//!
//! This crate provides utilities for data processing and analysis.
//!
//! ## Quick Start
//!
//! ```
//! use my_crate::Processor;
//!
//! let processor = Processor::new();
//! let result = processor.process("hello world");
//! println!("Result: {}", result);
//! ```
//!
//! ## Features
//!
//! - Fast data processing
//! - Memory efficient algorithms
//! - Thread-safe operations

/// A high-performance data processor.
///
/// The `Processor` struct provides efficient methods for processing
/// various types of data with configurable options.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use my_crate::Processor;
///
/// let processor = Processor::new();
/// let result = processor.process("input data");
/// ```
///
/// With configuration:
///
/// ```
/// use my_crate::{Processor, Config};
///
/// let config = Config::builder()
///     .buffer_size(1024)
///     .parallel(true)
///     .build();
///
/// let processor = Processor::with_config(config);
/// ```
pub struct Processor {
    config: Config,
}

impl Processor {
    /// Creates a new processor with default configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::Processor;
    ///
    /// let processor = Processor::new();
    /// ```
    pub fn new() -> Self {
        Processor {
            config: Config::default(),
        }
    }
    
    /// Creates a processor with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to use
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::{Processor, Config};
    ///
    /// let config = Config::default();
    /// let processor = Processor::with_config(config);
    /// ```
    pub fn with_config(config: Config) -> Self {
        Processor { config }
    }
    
    /// Processes the input data and returns the result.
    ///
    /// # Arguments
    ///
    /// * `input` - The data to process
    ///
    /// # Returns
    ///
    /// Returns the processed data as a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::Processor;
    ///
    /// let processor = Processor::new();
    /// let result = processor.process("hello");
    /// assert_eq!(result, "HELLO");
    /// ```
    ///
    /// # Errors
    ///
    /// This function does not return errors in the current implementation,
    /// but future versions may return `Result<String, ProcessError>`.
    pub fn process(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    buffer_size: usize,
    parallel: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            buffer_size: 512,
            parallel: false,
        }
    }
}

impl Config {
    /// Creates a new config builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::Config;
    ///
    /// let config = Config::builder()
    ///     .buffer_size(1024)
    ///     .build();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            config: Config::default(),
        }
    }
    
    /// Sets the buffer size.
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.config.buffer_size = size;
        self
    }
    
    /// Enables or disables parallel processing.
    pub fn parallel(mut self, parallel: bool) -> Self {
        self.config.parallel = parallel;
        self
    }
    
    /// Builds the final configuration.
    pub fn build(self) -> Config {
        self.config
    }
}
```

---

## 17. Advanced Rust Features

### Unsafe Rust

**Unsafe Capabilities:**
```rust
// Raw pointer dereferencing
unsafe fn dereference_raw_pointer() {
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// Calling unsafe functions
unsafe fn dangerous() {
    // Unsafe operations here
}

fn safe_wrapper() {
    unsafe {
        dangerous();
    }
}

// Accessing static mutable variables
static mut COUNTER: usize = 0;

fn add_to_count(inc: usize) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);
    
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

**Safe Abstractions with Unsafe:**
```rust
use std::slice;

// Creating a safe split_at_mut function
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Using external C code
extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_c_function() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

### Macros

**Declarative Macros:**
```rust
// Simple macro
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// Macro with parameters
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

// Macro with multiple patterns
macro_rules! vec_from {
    ($elem:expr; $n:expr) => {
        {
            let mut v = Vec::new();
            for _ in 0..$n {
                v.push($elem);
            }
            v
        }
    };
    ($($x:expr),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push($x);)+
            v
        }
    };
}

fn main() {
    say_hello!();
    foo();
    bar();
    
    let v1 = vec_from![42; 3];        // [42, 42, 42]
    let v2 = vec_from![1, 2, 3, 4];   // [1, 2, 3, 4]
}
```

**Procedural Macros:**
```rust
// Custom derive macro (in a separate crate)
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder_name = format!("{}Builder", name);
    let builder_ident = syn::Ident::new(&builder_name, name.span());
    
    let expanded = quote! {
        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident::new()
            }
        }
        
        pub struct #builder_ident {
            // Builder fields would be generated here
        }
        
        impl #builder_ident {
            pub fn new() -> Self {
                #builder_ident {
                    // Initialize builder fields
                }
            }
            
            pub fn build(self) -> #name {
                #name {
                    // Build the actual struct
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}

// Usage:
#[derive(Builder)]
struct User {
    name: String,
    age: u32,
}

// This generates a UserBuilder struct automatically
```

### Advanced Type System Features

**Higher-Ranked Trait Bounds:**
```rust
// Function that works with any lifetime
fn call_with_ref<F>(f: F) 
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = String::from("hello");
    let result = f(&s);
    println!("Result: {}", result);
}

// Closure that satisfies HRTB
let identity = |s: &str| s;
call_with_ref(identity);
```

**Associated Type Projections:**
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

trait Collect<T> {
    fn collect<I: Iterator<Item = T>>(iter: I) -> Self;
}

// Using associated type projections in where clauses
fn process_items<I>(iter: I) -> Vec<I::Item>
where
    I: Iterator,
    I::Item: Clone,
{
    iter.map(|item| item.clone()).collect()
}
```

**Phantom Types:**
```rust
use std::marker::PhantomData;

// State machine using phantom types
struct Locked;
struct Unlocked;

struct StateMachine<State> {
    _state: PhantomData<State>,
}

impl StateMachine<Locked> {
    fn new() -> Self {
        StateMachine {
            _state: PhantomData,
        }
    }
    
    fn unlock(self) -> StateMachine<Unlocked> {
        StateMachine {
            _state: PhantomData,
        }
    }
}

impl StateMachine<Unlocked> {
    fn lock(self) -> StateMachine<Locked> {
        StateMachine {
            _state: PhantomData,
        }
    }
    
    fn process(&self) {
        println!("Processing in unlocked state");
    }
}

// Usage
let machine = StateMachine::<Locked>::new();
let unlocked = machine.unlock();
unlocked.process(); // Only available in unlocked state
```

### Advanced Concurrency

**Atomic Operations:**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

let counter = Arc::new(AtomicUsize::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        for _ in 0..1000 {
            counter.fetch_add(1, Ordering::SeqCst);
        }
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Final count: {}", counter.load(Ordering::SeqCst));
```

**Custom Synchronization Primitives:**
```rust
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Semaphore {
    permits: Mutex<usize>,
    cvar: Condvar,
}

impl Semaphore {
    fn new(permits: usize) -> Self {
        Semaphore {
            permits: Mutex::new(permits),
            cvar: Condvar::new(),
        }
    }
    
    fn acquire(&self) {
        let mut permits = self.permits.lock().unwrap();
        while *permits == 0 {
            permits = self.cvar.wait(permits).unwrap();
        }
        *permits -= 1;
    }
    
    fn release(&self) {
        let mut permits = self.permits.lock().unwrap();
        *permits += 1;
        self.cvar.notify_one();
    }
}
```

### Error Handling Patterns

**Custom Error Types with From:**
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(err) => write!(f, "IO error: {}", err),
            MyError::Parse(err) => write!(f, "Parse error: {}", err),
            MyError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io(err) => Some(err),
            MyError::Parse(err) => Some(err),
            MyError::Custom(_) => None,
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

// Usage with ? operator
fn process_file(path: &str) -> Result<i32, MyError> {
    let contents = std::fs::read_to_string(path)?; // io::Error -> MyError
    let number = contents.trim().parse::<i32>()?;  // ParseIntError -> MyError
    Ok(number * 2)
}
```

---

## 18. Key Learning Patterns

### 1. Safety First Philosophy

**Memory Safety:**
Rust prevents entire classes of bugs at compile time:
```rust
// No null pointer dereferences
let value: Option<String> = None;
// value.len(); // Error: cannot call method on Option
if let Some(v) = value {
    println!("Length: {}", v.len()); // Safe access
}

// No buffer overflows
let arr = [1, 2, 3];
// println!("{}", arr[5]); // Panic in debug, but bounds are checked

// No use after free
let s = String::from("hello");
let s2 = s; // s is moved
// println!("{}", s); // Error: value used after move
```

**Thread Safety:**
```rust
use std::sync::Arc;
use std::thread;

// Compile-time prevention of data races
let data = Arc::new(vec![1, 2, 3]);

// This ensures thread safety at compile time
let handles: Vec<_> = (0..3).map(|i| {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        println!("Thread {}: {:?}", i, data);
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}
```

### 2. Zero-Cost Abstractions

**Iterator Performance:**
```rust
// High-level functional code
let sum: i32 = (0..1_000_000)
    .filter(|&x| x % 2 == 0)
    .map(|x| x * x)
    .sum();

// Compiles to the same optimized assembly as:
let mut sum = 0;
for i in (0..1_000_000).step_by(2) {
    sum += i * i;
}
```

**Generic Monomorphization:**
```rust
// Generic function
fn process<T: std::fmt::Display>(value: T) {
    println!("Processing: {}", value);
}

// Compiler generates separate optimized versions:
// process_i32(value: i32) { ... }
// process_String(value: String) { ... }
// No runtime overhead for generics
```

### 3. Explicit Design Decisions

**Error Handling:**
```rust
// Rust forces explicit error handling
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

// Must handle both success and failure cases
match parse_number("42") {
    Ok(num) => println!("Parsed: {}", num),
    Err(e) => println!("Error: {}", e),
}

// Or propagate errors explicitly
fn process_input(input: &str) -> Result<i32, std::num::ParseIntError> {
    let num = parse_number(input)?; // Explicit error propagation
    Ok(num * 2)
}
```

**Memory Management:**
```rust
// Explicit ownership transfers
fn take_ownership(s: String) -> String {
    s // Ownership transferred
}

// Explicit borrowing
fn borrow_data(s: &String) -> usize {
    s.len() // No ownership transfer
}

// Explicit mutability
fn modify_data(s: &mut String) {
    s.push_str(" modified"); // Explicit mutable access
}
```

### 4. Compiler as Teacher

**Helpful Error Messages:**
```rust
// Common ownership error
fn example() {
    let s = String::from("hello");
    let len = calculate_length(s); // s moved here
    println!("{}", s); // Error with helpful message
}

// Compiler suggests the fix:
// help: consider borrowing here: `&s`
```

**Borrow Checker Guidance:**
```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // Error with explanation about borrowing rules
    
    println!("{} {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // This is OK
    println!("{}", r3);
}
```

### 5. Composition Over Inheritance

**Trait Composition:**
```rust
// Instead of inheritance, use trait composition
trait Display {
    fn display(&self) -> String;
}

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

// Compose behaviors
struct Document {
    content: String,
}

impl Display for Document {
    fn display(&self) -> String {
        self.content.clone()
    }
}

impl Serialize for Document {
    fn serialize(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }
}

// Functions can require multiple traits
fn process_document<T: Display + Serialize>(doc: T) {
    println!("Display: {}", doc.display());
    println!("Serialized: {:?}", doc.serialize());
}
```

**Module-Based Organization:**
```rust
// Organize code into modules rather than class hierarchies
mod network {
    pub mod client {
        pub fn connect() { /* implementation */ }
    }
    
    pub mod server {
        pub fn listen() { /* implementation */ }
    }
    
    pub mod protocol {
        pub fn encode() { /* implementation */ }
        pub fn decode() { /* implementation */ }
    }
}

// Compose functionality from different modules
use network::{client, protocol};

fn main() {
    client::connect();
    protocol::encode();
}
```

### 6. Performance and Control

**Stack vs Heap Allocation:**
```rust
// Stack allocation (fast)
let array: [i32; 1000] = [0; 1000];

// Heap allocation (flexible)
let vector: Vec<i32> = vec![0; 1000];

// Choosing based on needs
fn process_small_data() -> [i32; 10] {
    [0; 10] // Stack allocated, returned by copy
}

fn process_large_data() -> Box<[i32; 1000]> {
    Box::new([0; 1000]) // Heap allocated, returned by move
}
```

**Memory Layout Control:**
```rust
#[repr(C)] // Use C-compatible layout
struct Point {
    x: f64,
    y: f64,
}

#[repr(packed)] // Pack fields tightly
struct PackedStruct {
    a: u8,
    b: u32,
}
```

### 7. Fearless Concurrency Patterns

**Safe Parallelism:**
```rust
use std::sync::Arc;
use std::thread;

// Data parallelism
fn parallel_sum(data: Vec<i32>) -> i32 {
    let data = Arc::new(data);
    let chunk_size = data.len() / 4;
    
    let handles: Vec<_> = (0..4).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let start = i * chunk_size;
            let end = if i == 3 { data.len() } else { (i + 1) * chunk_size };
            data[start..end].iter().sum::<i32>()
        })
    }).collect();
    
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum()
}
```

**Actor-Like Patterns:**
```rust
use std::sync::mpsc;
use std::thread;

enum WorkerMessage {
    Process(i32),
    Shutdown,
}

fn spawn_worker() -> mpsc::Sender<WorkerMessage> {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        for message in rx {
            match message {
                WorkerMessage::Process(data) => {
                    println!("Processing: {}", data);
                    // Do work...
                }
                WorkerMessage::Shutdown => break,
            }
        }
    });
    
    tx
}
```

### 8. Type-Driven Development

**Using Types to Prevent Bugs:**
```rust
// New type pattern for type safety
struct UserId(u64);
struct ProductId(u64);

// Prevents mixing up different ID types
fn get_user(id: UserId) -> Option<User> {
    // implementation
}

// This won't compile - type safety!
// let product_id = ProductId(123);
// get_user(product_id); // Error: expected UserId, found ProductId
```

**State Machines with Types:**
```rust
// Encode state transitions in the type system
struct Locked;
struct Unlocked;

struct Door<State> {
    _state: std::marker::PhantomData<State>,
}

impl Door<Locked> {
    fn unlock(self) -> Door<Unlocked> {
        Door { _state: std::marker::PhantomData }
    }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> {
        Door { _state: std::marker::PhantomData }
    }
    
    fn open(&self) {
        println!("Door opened");
    }
}

// Type system prevents opening a locked door
fn main() {
    let locked_door = Door::<Locked> { _state: std::marker::PhantomData };
    // locked_door.open(); // Error: method not available on Door<Locked>
    
    let unlocked_door = locked_door.unlock();
    unlocked_door.open(); // This works
}
```

### 9. Practical Wisdom

**RAII (Resource Acquisition Is Initialization):**
```rust
use std::fs::File;

// Resources are automatically cleaned up
fn process_file() -> std::io::Result<()> {
    let file = File::open("data.txt")?;
    // File is automatically closed when it goes out of scope
    // Even if an error occurs, destructor runs
    Ok(())
}
```

**Builder Pattern Implementation:**
```rust
struct HttpRequest {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

struct HttpRequestBuilder {
    url: Option<String>,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl HttpRequestBuilder {
    fn new() -> Self {
        HttpRequestBuilder {
            url: None,
            method: "GET".to_string(),
            headers: Vec::new(),
            body: None,
        }
    }
    
    fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    
    fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }
    
    fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
    
    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    
    fn build(self) -> Result<HttpRequest, &'static str> {
        let url = self.url.ok_or("URL is required")?;
        
        Ok(HttpRequest {
            url,
            method: self.method,
            headers: self.headers,
            body: self.body,
        })
    }
}

// Usage
let request = HttpRequestBuilder::new()
    .url("https://api.example.com")
    .method("POST")
    .header("Content-Type", "application/json")
    .body(r#"{"key": "value"}"#)
    .build()?;
```

### 10. Testing Patterns & Development Workflow

**Unit Testing:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_panic() {
        let _result = 10 / 0;
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Math is broken"))
        }
    }
}
```

**Integration Testing:**
```rust
// tests/integration_test.rs
use my_crate::*;

#[test]
fn test_integration() {
    // Test the public API
    let result = my_function();
    assert!(result.is_ok());
}
```

**Documentation Tests:**
```rust
/// Adds two numbers together.
/// 
/// # Examples
/// 
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 11. Cargo and Package Management

**Cargo.toml Configuration:**
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
description = "A short description"
repository = "https://github.com/username/my_project"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
criterion = "0.5"

[features]
default = ["std"]
std = []
no-std = []

[[bin]]
name = "main"
path = "src/main.rs"

[[bench]]
name = "my_benchmark"
harness = false
```

**Workspace Management:**
```toml
# Cargo.toml in workspace root
[workspace]
members = [
    "core",
    "cli",
    "web"
]

[workspace.dependencies]
serde = "1.0"

# In member crates
[dependencies]
serde = { workspace = true }
```

### 12. Performance Optimization Patterns

**Avoiding Allocations:**
```rust
// Bad: Creates new String each time
fn format_number_bad(n: i32) -> String {
    format!("Number: {}", n)
}

// Better: Use existing buffer
fn format_number_good(n: i32, buf: &mut String) {
    use std::fmt::Write;
    buf.clear();
    write!(buf, "Number: {}", n).unwrap();
}

// Best: For simple cases, use stack buffer
fn format_number_best(n: i32) -> heapless::String<32> {
    use heapless::String;
    use core::fmt::Write;
    let mut s = String::new();
    write!(s, "Number: {}", n).unwrap();
    s
}
```

**Efficient Data Structures:**
```rust
use std::collections::{HashMap, BTreeMap, HashSet};

// Choose the right collection
fn choose_collection() {
    // O(1) average lookup, random iteration order
    let mut hash_map: HashMap<String, i32> = HashMap::new();
    
    // O(log n) lookup, sorted iteration
    let mut btree_map: BTreeMap<String, i32> = BTreeMap::new();
    
    // For small collections, Vec might be faster
    let mut vec: Vec<(String, i32)> = Vec::new();
    
    // Pre-allocate when size is known
    let mut optimized_vec = Vec::with_capacity(100);
    let mut optimized_map = HashMap::with_capacity(100);
}
```

**Zero-Cost Abstractions:**
```rust
// Iterator chains compile to efficient loops
fn process_data(data: &[i32]) -> Vec<i32> {
    data.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect()
}

// This compiles to roughly:
fn process_data_optimized(data: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &item in data {
        if item > 0 {
            result.push(item * 2);
        }
    }
    result
}
```

### 13. Common Debugging Patterns

**Debug Traits and Formatting:**
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Custom debug implementation
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

// Debug macros
fn debug_example() {
    let x = 42;
    dbg!(x); // Prints file, line, and value
    println!("{:?}", x); // Standard debug print
    
    // Pretty printing
    let complex_struct = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
    println!("{:#?}", complex_struct);
}
```

**Error Context:**
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    context: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

// Usage with context
fn operation_with_context() -> Result<(), MyError> {
    std::fs::read_to_string("config.txt")
        .map_err(|e| MyError {
            context: "Failed to read configuration file".to_string(),
            source: Some(Box::new(e)),
        })?;
    Ok(())
}
```

### 14. Advanced Async Patterns

**Async Traits (with async-trait crate):**
```rust
use async_trait::async_trait;

#[async_trait]
trait AsyncProcessor {
    async fn process(&self, data: &str) -> Result<String, Box<dyn Error>>;
}

struct HttpProcessor;

#[async_trait]
impl AsyncProcessor for HttpProcessor {
    async fn process(&self, data: &str) -> Result<String, Box<dyn Error>> {
        // Async implementation
        Ok(format!("Processed: {}", data))
    }
}
```

**Stream Processing:**
```rust
use futures::stream::{self, StreamExt};
use tokio::time::{interval, Duration};

async fn stream_example() {
    // Create a stream of numbers
    let stream = stream::iter(0..10);
    
    // Transform and collect
    let doubled: Vec<i32> = stream
        .map(|x| x * 2)
        .filter(|&x| x > 5)
        .collect()
        .await;
    
    // Timed stream
    let mut ticker = interval(Duration::from_secs(1));
    for _ in 0..5 {
        ticker.tick().await;
        println!("Tick!");
    }
}
```

**Cancellation and Timeouts:**
```rust
use tokio::time::{timeout, Duration};
use tokio::select;

async fn cancellation_example() {
    let slow_operation = async {
        tokio::time::sleep(Duration::from_secs(10)).await;
        "Done"
    };
    
    // Timeout after 5 seconds
    match timeout(Duration::from_secs(5), slow_operation).await {
        Ok(result) => println!("Completed: {}", result),
        Err(_) => println!("Operation timed out"),
    }
    
    // Cancellation with select
    let operation = async { /* some work */ };
    let cancel_signal = tokio::signal::ctrl_c();
    
    select! {
        result = operation => {
            println!("Operation completed");
        }
        _ = cancel_signal => {
            println!("Operation cancelled");
        }
    }
}
```

### 15. Real-World Project Patterns

**Configuration Management:**
```rust
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

impl Config {
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    fn load_from_env() -> Result<Self, Box<dyn Error>> {
        envy::from_env().map_err(Into::into)
    }
}
```

**Application Structure:**
```rust
// main.rs
mod config;
mod handlers;
mod models;
mod services;
mod utils;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::init();
    
    // Load configuration
    let config = Config::load_from_file("config.toml")?;
    
    // Start application
    let app = create_app(config).await?;
    app.run().await?;
    
    Ok(())
}

async fn create_app(config: Config) -> Result<App, Box<dyn std::error::Error>> {
    // Application setup logic
    Ok(App::new(config))
}
```

### 16. Ecosystem Best Practices

**Logging with tracing:**
```rust
use tracing::{info, warn, error, debug, span, Level};

#[tracing::instrument]
async fn process_request(user_id: u64, request_id: String) -> Result<(), MyError> {
    let span = span!(Level::INFO, "processing", user_id, request_id);
    let _enter = span.enter();
    
    info!("Starting request processing");
    
    match do_processing().await {
        Ok(()) => {
            info!("Processing completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Processing failed: {}", e);
            Err(e)
        }
    }
}
```

**CLI Applications with clap:**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "my-app")]
#[command(about = "A fictional versioning CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a file
    Process {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Process { input, output, verbose }) => {
            if *verbose {
                println!("Processing {} -> {:?}", input, output);
            }
            // Process the file
        }
        None => {
            println!("No command specified");
        }
    }
}
```

---

## Summary

This comprehensive guide covers the essential concepts from the "100 Exercises to Learn Rust" course and beyond. The progression moves from basic syntax through advanced concurrent programming, providing both theoretical understanding and practical patterns.

**Key Takeaways:**
1. **Ownership is fundamental** - Master borrowing, lifetimes, and the borrow checker
2. **Traits are powerful** - Use them for abstraction, code reuse, and type safety
3. **Error handling is explicit** - Embrace `Result` and `Option` for robust programs
4. **Concurrency is fearless** - Leverage Rust's type system for safe parallel programming
5. **Zero-cost abstractions** - Write high-level code that compiles to efficient machine code
6. **Testing is integrated** - Use built-in testing tools and documentation tests
7. **Ecosystem is rich** - Leverage crates like serde, tokio, clap for rapid development

Continue practicing with real projects, contributing to open source, and exploring domain-specific applications to deepen your Rust expertise.

This comprehensive guide captures the essence of Rust programming as taught through the "100 Exercises to Learn Rust" course. The language's core philosophy centers on providing zero-cost abstractions while maintaining memory safety and preventing data races through its innovative ownership system and type system. Each concept builds upon previous ones, creating a cohesive understanding of how to write safe, fast, and concurrent code in Rust.