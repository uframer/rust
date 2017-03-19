# Rust程序设计语言

欢迎！本书会教你如何使用[Rust程序设计语言][rust].
Rust is a systems programming language focused on three goals: safety, speed,
and concurrency. It maintains these goals without having a garbage collector,
making it a useful language for a number of use cases other languages aren’t
good at: embedding in other languages, programs with specific space and time
requirements, and writing low-level code, like device drivers and operating
systems. It improves on current languages targeting this space by having a
number of compile-time safety checks that produce no runtime overhead, while
eliminating all data races. Rust also aims to achieve ‘zero-cost abstractions’
even though some of these abstractions feel like those of a high-level language.
Even then, Rust still allows precise control like a low-level language would.

[rust]: https://www.rust-lang.org

“The Rust Programming Language” is split into chapters. This introduction
is the first. After this:

* [上手][gs] - Set up your computer for Rust development.
* [教程：猜数字][gg] - Learn some Rust with a small project.
* [语法和语义][ss] - Each bit of Rust, broken down into small chunks.
* [高效Rust][er] - 介绍编写优秀Rust代码的进阶概念。
* [术语][gl] - A reference of terms used in the book.
* [参考书目][bi] - Background on Rust's influences, papers about Rust.

[gs]: getting-started.html
[gg]: guessing-game.html
[er]: effective-rust.html
[ss]: syntax-and-semantics.html
[gl]: glossary.html
[bi]: bibliography.html

### 如何做出贡献

生成本书的源文件可以在[GitHub][book]上找到。

[book]: https://github.com/rust-lang/rust/tree/master/src/doc/book
