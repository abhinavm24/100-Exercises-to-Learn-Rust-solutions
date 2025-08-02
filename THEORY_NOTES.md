# Rust Theory Notes - 100 Exercises Summary

*Comprehensive summary of key concepts from the "100 Exercises to Learn Rust" course*

## Table of Contents
1. [Fundamentals & Basic Types](#1-fundamentals--basic-types)
2. [Control Flow](#2-control-flow)
3. [Ownership & Memory Management](#3-ownership--memory-management)
4. [Structs & Methods](#4-structs--methods)
5. [Traits & Generics](#5-traits--generics)
6. [Error Handling](#6-error-handling)
7. [Collections & Data Structures](#7-collections--data-structures)
8. [Iterators & Functional Programming](#8-iterators--functional-programming)
9. [Concurrency - Threads](#9-concurrency---threads)
10. [Concurrency - Async/Await](#10-concurrency---asyncawait)
11. [Key Learning Patterns](#key-learning-patterns)

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

## 2. Control Flow

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

### Pattern Matching & Enums

**Enum Definition:**
```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

enum Message {
    Text(String),
    Number(i32),
    Coordinates { x: i32, y: i32 },
}
```

**`match` Expressions:**
```rust
let status = Status::Active;

match status {
    Status::Active => println!("System is running"),
    Status::Inactive => println!("System is down"),
    Status::Pending => println!("System is starting"),
}

// With data extraction
let msg = Message::Coordinates { x: 10, y: 20 };
match msg {
    Message::Text(content) => println!("Text: {}", content),
    Message::Number(num) => println!("Number: {}", num),
    Message::Coordinates { x, y } => println!("Point: ({}, {})", x, y),
}
```

**`if let` Syntax:**
```rust
let some_value = Some(42);

if let Some(number) = some_value {
    println!("Found number: {}", number);
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

---

## 6. Error Handling

### Panic vs Result

**Panics (Unrecoverable Errors):**
```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}
```

**Result (Recoverable Errors):**
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

### Working with Result

**Pattern Matching:**
```rust
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}
```

**Unwrap and Expect:**
```rust
let result = divide(10, 2).unwrap();          // Panics on Err
let result = divide(10, 2).expect("Failed to divide"); // Custom panic message
```

**Error Propagation with `?`:**
```rust
fn calculate() -> Result<i32, String> {
    let a = divide(10, 2)?;  // Returns early if Err
    let b = divide(a, 3)?;   // Returns early if Err
    Ok(b * 2)
}
```

### Custom Error Types

**Simple Error Enum:**
```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    Overflow,
}

fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
```

**Error Trait Implementation:**
```rust
use std::fmt;
use std::error::Error;

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::Overflow => write!(f, "Calculation overflow"),
        }
    }
}

impl Error for MathError {}
```

**Error Chaining:**
```rust
#[derive(Debug)]
struct ParseIntError {
    source: std::num::ParseIntError,
}

impl Error for ParseIntError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

---

## 7. Collections & Data Structures

### Vectors

**Creating and Using Vectors:**
```rust
// Creating vectors
let mut v1: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3, 4, 5];

// Adding elements
v1.push(1);
v1.push(2);

// Accessing elements
let third = v2[2];                    // Panics if index out of bounds
let third = v2.get(2);                // Returns Option<&T>

match v2.get(2) {
    Some(value) => println!("Third element: {}", value),
    None => println!("No third element"),
}
```

**Vector Memory Layout:**
- **ptr**: Pointer to heap-allocated data
- **len**: Current number of elements
- **capacity**: Total allocated space

**Iteration:**
```rust
let v = vec![1, 2, 3];

// Immutable references
for item in &v {
    println!("{}", item);
}

// Mutable references
let mut v = vec![1, 2, 3];
for item in &mut v {
    *item += 1;
}

// Take ownership
for item in v {
    println!("{}", item);
}
// v is no longer accessible
```

### HashMaps

**Creating and Using HashMaps:**
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// Inserting values
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Accessing values
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // Returns Option<&V>

match score {
    Some(s) => println!("Score: {}", s),
    None => println!("Team not found"),
}
```

**HashMap Requirements:**
- Keys must implement `Eq + Hash`
- Equal keys must produce equal hash values

**Common Operations:**
```rust
let mut map = HashMap::new();

// Insert or update
map.insert("key", "value");

// Insert only if key doesn't exist
map.entry("key").or_insert("default");

// Update existing value
let entry = map.entry("key").or_insert(0);
*entry += 1;

// Iteration
for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

### Arrays vs Vectors

**Arrays (Fixed Size):**
```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];  // Stack allocated
let arr = [3; 5];                      // [3, 3, 3, 3, 3]
```

**Vectors (Dynamic Size):**
```rust
let mut vec: Vec<i32> = Vec::new();    // Heap allocated
vec.push(1);
vec.push(2);
```

---

## 8. Iterators & Functional Programming

### Iterator Traits

**Core Iterator Trait:**
```rust
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // Many default implementations provided
}
```

**IntoIterator Trait:**
```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter;
}
```

### Creating and Using Iterators

**From Collections:**
```rust
let vec = vec![1, 2, 3, 4, 5];

// Different iteration modes
for item in vec.iter() {        // Immutable references
    println!("{}", item);
}

for item in vec.iter_mut() {    // Mutable references (if vec is mut)
    *item *= 2;
}

for item in vec.into_iter() {   // Take ownership
    println!("{}", item);
}
// vec is no longer accessible after into_iter()
```

**For Loop Desugaring:**
```rust
// This for loop...
for item in collection {
    // body
}

// ...is equivalent to:
let mut iter = collection.into_iter();
loop {
    match iter.next() {
        Some(item) => {
            // body
        }
        None => break,
    }
}
```

### Iterator Adaptors and Consumers

**Iterator Adaptors (Lazy):**
```rust
let vec = vec![1, 2, 3, 4, 5];

let doubled: Vec<i32> = vec
    .iter()
    .map(|x| x * 2)        // Transform each element
    .filter(|&x| x > 4)    // Keep elements > 4
    .collect();            // Consumer: collect into Vec

println!("{:?}", doubled); // [6, 8, 10]
```

**Common Adaptors:**
```rust
let numbers = vec![1, 2, 3, 4, 5];

// map: transform elements
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// filter: keep elements matching predicate
let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();

// enumerate: add indices
let with_indices: Vec<(usize, &i32)> = numbers.iter().enumerate().collect();

// take: take first n elements
let first_three: Vec<&i32> = numbers.iter().take(3).collect();

// skip: skip first n elements
let skip_two: Vec<&i32> = numbers.iter().skip(2).collect();
```

**Consumers:**
```rust
let numbers = vec![1, 2, 3, 4, 5];

// collect: consume into collection
let collected: Vec<i32> = numbers.iter().cloned().collect();

// fold/reduce: accumulate values
let sum = numbers.iter().fold(0, |acc, x| acc + x);

// find: find first matching element
let found = numbers.iter().find(|&x| x > 3);

// any/all: boolean predicates
let has_even = numbers.iter().any(|&x| x % 2 == 0);
let all_positive = numbers.iter().all(|&x| x > 0);

// for_each: side effects
numbers.iter().for_each(|x| println!("{}", x));
```

### Lifetimes in Iterators

**Lifetime Parameters:**
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**Iterator Borrowing:**
```rust
fn process_strings(strings: &[String]) -> Vec<&str> {
    strings
        .iter()
        .filter(|s| s.len() > 3)
        .map(|s| s.as_str())
        .collect()
}
// Returned references are tied to input slice lifetime
```

---

## 9. Concurrency - Threads

### Thread Fundamentals

**What are Threads:**
- Independent execution contexts within a process
- Each thread has its own stack and instruction pointer
- Threads share memory space within the same process
- OS scheduler manages CPU time allocation

**Creating Threads:**
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();  // Wait for thread to complete
}
```

### Thread Communication

**Channels (Message Passing):**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("Hello from thread");
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
```

**Multiple Producers:**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(format!("Message {}", i)).unwrap();
        });
    }
    
    drop(tx);  // Close the original sender
    
    for received in rx {
        println!("Got: {}", received);
    }
}
```

### Shared State Concurrency

**Mutex (Mutual Exclusion):**
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

**RwLock (Read-Write Lock):**
```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // Multiple readers
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let r = data.read().unwrap();
            println!("Reader {}: {:?}", i, *r);
        });
        handles.push(handle);
    }
    
    // Single writer
    let data = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut w = data.write().unwrap();
        w.push(4);
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Thread Safety Traits

**`Send` Trait:**
- Types that can be safely transferred between threads
- Most types are `Send`, notable exceptions: `Rc<T>`, raw pointers

**`Sync` Trait:**
- Types that can be safely accessed from multiple threads simultaneously
- `&T` is `Send` if `T` is `Sync`

```rust
// These implement Send + Sync
let data1: i32 = 42;
let data2: String = String::from("hello");
let data3: Vec<i32> = vec![1, 2, 3];

// These do NOT implement Send/Sync
use std::rc::Rc;
let data4: Rc<i32> = Rc::new(42);  // Not Send or Sync
```

---

## 10. Concurrency - Async/Await

### Asynchronous Fundamentals

**Async Functions:**
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
```

**Awaiting Futures:**
```rust
async fn main_async() {
    hello_world().await;  // Wait for completion
}
```

### Future Trait

**Core Future Trait:**
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),    // Future completed with result
    Pending,     // Future not ready, try again later
}
```

**Future as State Machine:**
```rust
async fn example() -> i32 {
    let a = fetch_data().await;
    let b = process_data(a).await;
    b + 1
}

// Compiler generates state machine:
enum ExampleFuture {
    Start,
    WaitingForFetchData(/* fetch_data future */),
    WaitingForProcessData(DataType, /* process_data future */),
    Done,
}
```

### Async Runtimes

**Tokio Runtime:**
```rust
// Single-threaded runtime
#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello from single-threaded runtime!");
}

// Multi-threaded runtime (default)
#[tokio::main]
async fn main() {
    println!("Hello from multi-threaded runtime!");
}
```

**Spawning Tasks:**
```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let handle1 = task::spawn(async {
        // Task 1 work
        42
    });
    
    let handle2 = task::spawn(async {
        // Task 2 work
        "hello"
    });
    
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
    
    println!("Results: {} {}", result1, result2);
}
```

### Async Constraints

**`Send` Requirement:**
```rust
// This works - i32 is Send
async fn good_async() -> i32 {
    let x = 42;
    some_async_call().await;
    x
}

// This might not work if Rc is held across await
use std::rc::Rc;
async fn problematic_async() -> i32 {
    let x = Rc::new(42);
    some_async_call().await;  // Error: Rc is not Send
    *x
}
```

**`'static` Lifetime for Spawned Tasks:**
```rust
#[tokio::main]
async fn main() {
    let data = "Hello".to_string();
    
    // This won't compile - spawned task can't reference local data
    // let handle = task::spawn(async {
    //     println!("{}", data);
    // });
    
    // Solution: move data into the task
    let handle = task::spawn(async move {
        println!("{}", data);
    });
    
    handle.await.unwrap();
}
```

### Common Async Patterns

**Concurrent Execution:**
```rust
use tokio::join;

async fn fetch_user(id: u32) -> String {
    // Simulate API call
    format!("User {}", id)
}

#[tokio::main]
async fn main() {
    // Sequential (slow)
    let user1 = fetch_user(1).await;
    let user2 = fetch_user(2).await;
    
    // Concurrent (fast)
    let (user1, user2) = join!(
        fetch_user(1),
        fetch_user(2)
    );
    
    println!("{}, {}", user1, user2);
}
```

**Error Handling:**
```rust
async fn might_fail() -> Result<String, &'static str> {
    // Simulate work that might fail
    Ok("Success".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = might_fail().await?;
    println!("Result: {}", result);
    Ok(())
}
```

---

## Key Learning Patterns

### 1. Type Safety First
Rust prevents many errors at compile time through its type system:
- No null pointer dereferences
- No buffer overflows
- No data races
- Memory safety without garbage collection

### 2. Zero-Cost Abstractions
High-level features don't compromise runtime performance:
- Iterators compile to the same code as loops
- Generics have no runtime overhead (monomorphization)
- Traits enable polymorphism without virtual function calls when statically dispatched

### 3. Explicit Trade-offs
Rust makes performance and safety trade-offs visible:
- Explicit memory management through ownership
- Explicit error handling with `Result`
- Explicit concurrency with `Send`/`Sync` traits

### 4. Compiler as Teacher
Error messages guide toward correct solutions:
- Ownership errors explain borrowing rules
- Type errors suggest trait implementations
- Lifetime errors clarify reference validity

### 5. Composition Over Inheritance
Rust favors composition through:
- Traits for shared behavior
- Generics for code reuse
- Modules for code organization

### 6. Fearless Concurrency
Safe concurrent programming through:
- Ownership prevents data races
- Type system enforces thread safety
- Message passing and shared state both supported safely

This comprehensive guide covers the foundational concepts taught throughout the "100 Exercises to Learn Rust" course, progressing from basic syntax to advanced concurrent programming patterns, all unified by Rust's core principles of safety, performance, and explicitness.