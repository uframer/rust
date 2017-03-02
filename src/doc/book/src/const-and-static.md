# `const`和`static`

Rust可以使用`const`关键字定义常量：

```rust
const N: i32 = 5;
```

不同于[`let`][let]定义的变量绑定，你必须使用`const`关键字。

[let]: variable-bindings.html

常量在整个程序的生命周期内都有效。更具体地说，Rust中的常量没有固定的内存地址。这是因为它们可能会被直接内联展开到使用它们的地方。因为这个机制，不同的地方对同一个常量的引用可能指向的并不是同一个内存地址。

# `static`

Rust通过`static`机制提供了类似于*全局变量*的东西。它们类似于常量，但是不会被内联展开。这意味着每个`static`值只有一个实例，并且在内存中有一个固定的地址。

这里是一个例子：

```rust
static N: i32 = 5;
```

不同于[`let`][let]定义的变量绑定，你必须使用`static`来标注类型。

静态变量在整个程序的生命周期内都有效，and therefore any
reference stored in a constant has a [`'static` lifetime][lifetimes]:

```rust
static NAME: &'static str = "Steve";
```

[lifetimes]: lifetimes.html

## 可变性

你可以使用`mut`关键字将它标记为可变的：

```rust
static mut N: i32 = 5;
```

Because this is mutable, one thread could be updating `N` while another is
reading it, causing memory unsafety. As such both accessing and mutating a
`static mut` is [`unsafe`][unsafe], and so must be done in an `unsafe` block:

```rust
# static mut N: i32 = 5;

unsafe {
    N += 1;

    println!("N: {}", N);
}
```

[unsafe]: unsafe.html

Furthermore, any type stored in a `static` must be `Sync`, and must not have
a [`Drop`][drop] implementation.

[drop]: drop.html

# 初始化

Both `const` and `static` have requirements for giving them a value. They must
be given a value that’s a constant expression. In other words, you cannot use
the result of a function call or anything similarly complex or at runtime.

# 我应该使用哪种呢？

Almost always, if you can choose between the two, choose `const`. It’s pretty
rare that you actually want a memory location associated with your constant,
and using a `const` allows for optimizations like constant propagation not only
in your crate but downstream crates.
