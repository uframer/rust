# 注释

Rust支持两种类型的注释：*单行注释*和*文档注释*。

单行注释以‘//’开头，直到行尾。

```rust
// 单行注释从‘//’开始，一直拓展到行尾。

let x = 5; // 这也是一个单行注释。

// 如果你要解释的事情很多，注释很长，那么可以连着写
// 很多行单行注释。注意在`//`后面加一个空格，这样看
// 起来更美观。
```

文档注释用`///`开头，其中的内容支持Markdown语法：

```rust
/// 将传入的数字加一。
///
/// # 例子
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
```

文档注释还有另一种风格，以`//!`开始，它所说明的是包含这段注释的项目（例如，crate、模块、函数等）。它通常用于crate的根（lib.rs）或者模块的根（mod.rs）：

```
//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
```

如果在编写文档注释时能够同时提供一些例子的话，是非常有助于读者理解的。在上面的例子或者中我们用了一个新的宏：`assert_eq!`。这个宏比较两个值，如果它们不相等，就会`panic!`。在文档的代码例子中使用这个宏可以明确地说明代码的行为。另一个宏`assert!`，会在传给它的值为`false`时`panic!`。

你可以使用[`rustdoc`](documentation.html)工具按照文档注释生成HTML文档，还可以将注释中的代码例子当做测试运行！
