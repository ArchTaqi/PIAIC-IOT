# Rust programming language

Rust is designed for writing and maintaining fast, reliable, and efficient code. It can replace C++ in most cases. But it cannot replace C in bare-metal embed programming.

<hr />
## Why Rust?

- **Performance**. Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.
- **Reliability**. Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — and enable you to eliminate many classes of bugs at compile-time.
- **Productivity**. Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

With its inclusive community and top-notch libraries like:

- [Serde](https://serde.rs), for serializing and deserializing data.
- [Rayon](https://github.com/rayon-rs/rayon), for writing parallel & data race-free code.
- [Tokio](https://github.com/tokio-rs/tokio)/[async-std](https://github.com/async-rs/async-std), for writing non-blocking, low-latency network services.
- [tracing](https://github.com/tokio-rs/tracing), for instrumenting Rust programs to collect structured, event-based diagnostic information.


- **Go vs Python vs. Rust**

    - **Go** is a popular language, rather simple to learn, with lot of libraries, a big community. The fact it is backed by Google helped with the latter.For programming servers, Go is an excellent choice. Very easy to pick up, you get productive in it very quickly. The built-in concurrent programming model is easier to reason about than others. The tooling is excellent, and the compiler is lightning fast. Go is a nice, simple language. Its minimalist feature set is orthogonal and powerful. Performance is not an issue; Go is faster than Java or JavaScript, the two most popular languages on the planet. And it's great for web development, too (A Word from The Beegoist)! (FYI, the Beego web framework is crazy hot in China!)

    - **Python** compiles to bytecodes (as does Java), but you still need the Python virtual machine (the VM), “hogging” memory (managing memory) when those bytecodes are run. Whether the source recompiles or not, the Python VM has to accompany the bytecode files, which slows things down. A bare chip wouldn’t know what to do with bytecode files (unless it’s a chip customized to run Python bytecodes — these exist, see MicroPython). Java is a little different because many ARM chips have a Java bytecode mode (see Jazelle).

    - **Rust** is a bit harder to learn (although its syntax is relatively simple), has different concepts that must sink in your mind before becoming proficient. It has a good **type system**, and a powerful concept around memory management: **no memory leak nor null pointer exception**!. Rust goes straight to the chip’s assembly language, like C and C++ do. No virtual machine, no garbage collection. Rust aims to manage memory at compile time, meaning the language itself makes it hard to legally introduce memory leaks etc.

> You might be more productive faster with Go, you will learn interesting things, and become a better programmer if you learn Rust... :-. Python will likely be implemented in Rust someday soon, if it hasn’t been already.

<hr />
