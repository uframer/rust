% 变量绑定

`let`关键字用于定义*变量绑定*，也就是将一个值绑定到一个名字上，以便以后使用。例如：

```rust
fn main() {
    let x = 5;
}
```

在每个例子里都写`fn main() {`有一点儿繁琐，所以我们在将来就省略它。如果你想要自己试验这些代码，请确补上`main()`函数。

# 模式

在许多语言中，变量绑定会被称作*变量*，但是Rust的变量绑定有些不一样的地方。例如，左手边的`let`语句是一个[模式][pattern]，不是一个变量名。这意味着我们可以像下面这样写：

```rust
let (x, y) = (1, 2);
```

在这条语句被计算时，`x`会是`1`，`y`会是`2`。模式是十分强大的工具，我们会[单独介绍][pattern]它。现在我们还不需要了解那么多，所以现在只要知道有这一回事就行了。

[pattern]: patterns.html

# 类型标注

Rust是一个静态类型语言，这意味着我们需要先指定类型，然后这些类型会在编译时由编译器检查。这样说来，我们的第一个例子里并没有指明类型，那它又是怎样编译通过的呢？答案就是，Rust支持**类型推断**。如果Rust能猜出一个东西的类型是什么，你就不用去明确地指定它的类型。

不过，就算有了类型推断，你还是可以直接指定类型。类型名出线在冒号（`:`）之后：

```rust
let x: i32 = 5;
```

上面这行代码的含义是：`x`是一个具有`i32`类型的绑定，它的值是`5`。

Rust有很多不同的基本整数类型，以`i`开头的是有符号整数，以`u`开头的是无符号整数。整数的宽度可以是8、16、32或者64位。

在以后的例子中，我们会在注释里标明类型，比如前面的例子会写成这样：

```rust
fn main() {
    let x = 5; // x: i32
}
```

你可以注意到这个注释的写法类似于`let`的语法。

# 可变性

默认情况下，绑定是*不可变*的。下面的代码过不了编译：

```rust,ignore
let x = 5;
x = 10;
```

编译错误信息类似于：

```text
error: re-assignment of immutable variable `x`
     x = 10;
     ^~~~~~~
```

如果你需要一个可变的绑定，需要使用`mut`关键字：

```rust
let mut x = 5; // mut x: i32
x = 10;
```

将绑定设计为默认*不可变*有很多原因，但是我们可以将它们归结为Rust的主要目标：安全性。如果你忘记添加`mut`关键字，编译器会发现并告诉你。只有确实需要时，才应该添加`mut`关键字。

# 绑定的初始化

Rust的变量绑定同其他语言的变量有另一个不同之处：绑定必须在使用前被初始化。

下面我们试一下。将你的`src/main.rs`文件改成如下面这样：

```rust
fn main() {
    let x: i32;

    println!("Hello world!");
}
```

你可以使用`cargo build`命令构建这个程序。你会看到一个警告，但是它还是会打出"Hello, world!"：

```text
   Compiling hello_world v0.0.1 (file:///home/you/projects/hello_world)
src/main.rs:2:9: 2:10 warning: unused variable: `x`, #[warn(unused_variables)]
   on by default
src/main.rs:2     let x: i32;
                      ^
```

Rust警告我们这个变量绑定没有被用到，但也正是因为没有用到它，也就不会有什么危害，因此不会报错。如果我们试图使用这个绑定`x`，事情就会其变化。我们试一下，将你的程序改成这样：

```rust,ignore
fn main() {
    let x: i32;

    println!("The value of x is: {}", x);
}
```

现在试着构建它。你会看到一个错误：

```bash
$ cargo build
   Compiling hello_world v0.0.1 (file:///home/you/projects/hello_world)
src/main.rs:4:39: 4:40 error: use of possibly uninitialized variable: `x`
src/main.rs:4     println!("The value of x is: {}", x);
                                                    ^
note: in expansion of format_args!
<std macros>:2:23: 2:77 note: expansion site
<std macros>:1:1: 3:2 note: in expansion of println!
src/main.rs:4:5: 4:42 note: expansion site
error: aborting due to previous error
Could not compile `hello_world`.
```

Rust不允许使用一个未初始化的绑定。

我们顺带讲一下上面例子里出现的`println!`的用法。

如果你在要打印的字符串里插入一对大括号（`{}`），那么Rust会在这里插入后面列出的参数值，用过C语言的`printf`的读到这里大概就知道是怎么回事了。如果你只是写了`{}`，那么Rust会根据参数的类型安排一个合理的打印格式；你也可以直接指定打印格式，[这里][format]有大量的选项。

`println!`就先介绍到这里。

[format]: ../std/fmt/index.html

# 作用域和遮蔽

变量绑定的作用域开始于它出现的位置，结束于当前代码块（block）的末尾。代码块以`{`开头，以`}`结尾。函数体也是包在`{`和`}`之间，所以也是代码块。直接看下面的例子：

```rust,ignore
fn main() {
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); // 因为y已经脱离作用域，所以这里会编译失败
}
```

编译上面的代码会出现错误：

```bash
$ cargo build
   Compiling hello v0.1.0 (file:///home/you/projects/hello_world)
main.rs:7:62: 7:63 error: unresolved name `y`. Did you mean `x`? [E0425]
main.rs:7     println!("The value of x is {} and value of y is {}", x, y); // This won't work.
                                                                       ^
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
main.rs:7:5: 7:65 note: expansion site
main.rs:7:62: 7:63 help: run `rustc --explain E0425` to see a detailed explanation
error: aborting due to previous error
Could not compile `hello`.

To learn more, run the command again with --verbose.
```

Rust语言有一个不同于C系列语言的行为，这就是变量绑定可以被遮蔽（shadowing）。即使是在同一个代码块内，后面做出的变量绑定会遮蔽当前作用域中可见的同名绑定，社会之可以绑定到不同的类型。

```rust
let x: i32 = 8;
{
    println!("{}", x); // 输出"8".
    let x = 12;
    println!("{}", x); // 输出"12".
}
println!("{}", x); // 输出"8".
let x =  42;
println!("{}", x); // 输出"42".
```

*绑定遮蔽*和*可变绑定*有些类似，但是它们不是一回事需要注意区分。绑定遮蔽可以改变绑定的可变性，但是更重要的是它可以将同一个名字绑定到不同的类型上去。此外，当一个绑定被遮蔽时，Rust不会修改或者销毁被绑定的值，也就是不会调用它的析构方法（参考[`Drop`特征](drop.html)），即使这个值再也无法被访问到，它只有在脱离作用域时才会被销毁。

```rust
let mut x: i32 = 1;
x = 7;
let x = x; // `x`现在被绑定到7，并且是不可变的

let y = 4;
let y = "I can also be bound to text!"; // `y`现在是一个不同类型的绑定
```
