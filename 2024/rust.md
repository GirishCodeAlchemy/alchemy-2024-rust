## What is Rust?

Rust is an open-source, systems programming language that you can use to develop efficient, safe software. With Rust, you can manage memory and control other low-level details. But you can also take advantage of high-level concepts like iteration and interfaces. These features set Rust apart from low-level languages like C and C++.

Rust also offers the following advantages that make it a great fit for a wide range of applications:

- `Type safe:` The compiler assures that no operation will be applied to a variable of a wrong type.
- `Memory safe:` Rust pointers (known as references) always refer to valid memory.
- `Data race free:` Rust's borrow checker guarantees thread-safety by ensuring that multiple parts of a program can't mutate the same value at the same time.
- `Zero-cost abstractions:` Rust allows the use of high-level concepts, like iteration, interfaces, and functional programming, with minimal to no performance costs. The abstractions perform as well, as if you wrote the underlying code by hand.
- `Minimal runtime:` Rust has a minimal and optional runtime. The language also has no garbage collector to manage memory efficiently. In this way, Rust is most similar to languages like C and C++.
- `Targets bare metal:` Rust can target embedded and "bare metal" programming, making it suitable to write an operating system kernel or device drivers.


## When to use Rust
The Rust language has many strengths to consider when choosing the best language for your project:

- Rust allows for control over the performance and resource consumption of programs and libraries written in the language on par with C and C++, while still being memory safe by default. This level of control eliminates entire classes of common bugs.
- Rust has rich abstraction features that allow developers to encode many of the invariants of their program into code. the code is then checked by the compiler instead of relying on convention or documentation. This feature can often lead to the feeling of "if it compiles, it works."
- Rust has built-in tools for building, testing, documenting, and sharing code as well as a rich ecosystem of third-party tools and libraries. These tools can make some tasks that are difficult in some languages, such as building dependencies, easy and productive in Rust.


## Manage code with the Rust module system
Rust offers a collection of features to help you manage and organize your code. These features are referred to as the Rust module system. The system is composed of crates, modules, and paths, and tools to work with those items.

* ### Crates:
 A Rust crate is a compilation unit. It's the smallest piece of code the Rust compiler can run. The code in a crate is compiled together to create a binary executable or a library. In Rust, only crates are compiled as reusable units. A crate contains a hierarchy of Rust modules with an implicit, unnamed top-level module. 
 https://crates.io/

* ### Modules:
 Rust modules help you organize your program by letting you manage the scope of the individual code items inside a crate. Related code items or items that are used together can be grouped into the same module. Recursive code definitions can span other modules.

* ### Paths:
 In Rust, you can use paths to name items in your code. For example, a path can be a data definition like a vector, a code function, or even a module. The module feature also helps you control the privacy of your paths. You can specify the parts of your code that are accessible publicly versus parts that are private. This feature lets you hide the implementation details.


## Create and manage projects with Cargo
While it's possible to use the Rust compiler (rustc) directly to build crates, most projects use the Rust build tool and dependency manager called Cargo.

Cargo does lots of things for you, including:

* Create new project templates with the cargo new command.
* Build a project with the cargo build command.
* Build and run a project with the cargo run command.
* Test a project with the cargo test command.
* Check project types with the cargo check command.
* Build documentation for a project with the cargo doc command.
* Publish a library to crates.io with the cargo publish command.
* Add dependent crates to a project by adding the crate name to the Cargo.toml file.