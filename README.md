#  100 exercises to learn Rust [![official JetBrains project](http://jb.gg/badges/official.svg)](https://confluence.jetbrains.com/display/ALL/JetBrains+on+GitHub)

Greetings and welcome to `100 exercises to learn Rust`.
This project contains small exercises to get you used to reading and writing Rust code.
This includes reading and responding to compiler messages!

## About This Project
This project serves as a comprehensive, hands-on learning course for Rust, available at [rust-exercises.com/100-exercises](https://rust-exercises.com/100-exercises/). 
It is designed to provide an in-IDE learning experience, intended to be used as a RustRover plugin.
This integration allows for interactive challenges, real-time feedback, and guided practice directly within your development environment.

### Key Features
- **Interactive Learning**: 100 progressive exercises designed for hands-on practice
- **IDE Integration**: Built specifically for RustRover with real-time feedback
- **Comprehensive Coverage**: From basic syntax to advanced topics like concurrency and async programming
- **Structured Progression**: Carefully designed learning path building complexity gradually
- **Practical Focus**: Learn by building real-world examples and solving coding challenges

## Course Structure

The course is organized into 9 main sections with over 100 individual exercises:

### 1. **Introduction**
Getting started with the course, IDE basics, and your first "Hello, World!" program.

### 2. **A Basic Calculator**
Foundation concepts including:
- Integers and variables
- Control flow (if/else, loops)
- Error handling (panics)
- Arithmetic operations and overflow handling

### 3. **Ticket v1**
Object-oriented programming concepts:
- Structs and methods
- Ownership and borrowing
- Modules and visibility
- Memory management (stack vs heap)
- References and lifetimes

### 4. **Traits**
Rust's powerful trait system:
- Defining and implementing traits
- Trait bounds and generic programming
- Derive macros
- Operator overloading
- Standard library traits (Clone, Copy, Drop, etc.)

### 5. **Ticket v2**
Advanced type system features:
- Enums and pattern matching
- Error handling with Result and Option
- Custom error types
- Package management and dependencies

### 6. **Ticket Management**
Collections and advanced programming:
- Arrays, vectors, and slices
- Iterators and functional programming
- HashMap and BTreeMap
- Advanced lifetimes and borrowing

### 7. **Threads**
Concurrent programming:
- Thread creation and management
- Channels for communication
- Synchronization primitives (Mutex, RwLock, Arc)
- Thread safety and the Send/Sync traits

### 8. **Futures**
Asynchronous programming:
- Async/await syntax
- Future trait and task spawning
- Async runtimes
- Cancellation and error handling

### 9. **Going Further**
Wrap-up and next steps for continued learning.

## Exercise Format

Each exercise follows a consistent structure:

```
ExerciseName/
├── Theory/           # Explanatory content and concepts
│   ├── Cargo.toml
│   └── task.md
├── Task/             # Hands-on coding challenge
│   ├── Cargo.toml
│   ├── src/
│   ├── tests/
│   └── task.md
└── lesson-info.yaml  # Metadata for IDE integration
```

- **Theory sections** provide conceptual explanations
- **Task sections** contain coding challenges with TODO comments
- **Tests** validate your solutions automatically
- Each exercise is a separate Cargo crate for isolation

## Getting Started

1. Open this project in RustRover
2. Navigate to the Introduction section
3. Follow the exercises in order
4. Read the theory, complete the tasks, and run the tests
5. Use the compiler messages to guide your learning

## License
`100-exercises-to-learn-rust` is distributed under the terms of the Apache License (Version 2.0). See [LICENSE file](LICENSE) for details.

## Contributing
Please be sure to review `100-exercises-to-learn-rust` [contributing guidelines](CONTRIBUTING.md) to learn how to help the project.

## Credits
The materials are based on the original course [100-exercises-to-learn-rust](https://github.com/mainmatter/100-exercises-to-learn-rust) written by [Luca Palmieri](https://www.lpalmieri.com/)!
